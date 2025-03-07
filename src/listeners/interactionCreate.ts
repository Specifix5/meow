import {
  CommandInteraction,
  UserContextMenuCommandInteraction,
  ApplicationCommandOptionData,
  AutocompleteInteraction,
  Interaction,
} from "discord.js";
import { CreateErrorMessage } from "../utils/logging.js";
import { MeowClient } from "../utils/nyan/client.js";
import { MeowInteraction } from "../utils/nyan/command.js";
import { EMOJI } from "../utils/constants.js";

export const interactionErrorHandler = async (
  interaction: CommandInteraction | UserContextMenuCommandInteraction,
  err: unknown,
) => {
  if (err instanceof Error) {
    if (interaction.replied || interaction.deferred) {
      await interaction.followUp({
        content: CreateErrorMessage(err),
        ephemeral: true,
      });
    } else {
      await interaction.reply({
        content: CreateErrorMessage(err),
        ephemeral: true,
      });
    }
  }
};

const slashCommandHandler = async (
  client: MeowClient,
  interaction: CommandInteraction,
): Promise<void> => {
  const _command = client.getCommands().find((c) => c.name === interaction.commandName);
  if (!_command) {
    await interaction.reply({
      content: `${EMOJI.ICON_DENY} ${CreateErrorMessage(
        new Error("No matching command by the name '" + interaction.commandName + "'"),
      )}`,
      ephemeral: true,
    });
    return;
  }
  try {
    await _command.run(
      client,
      new MeowInteraction(client, interaction, _command.options as ApplicationCommandOptionData[]),
    );
  } catch (err: unknown) {
    if (err instanceof Error) {
      client.logger.error("[SlashCommands] " + err.message);
      await interactionErrorHandler(interaction, err);
    }
  }
  return;
};

const commandAutocompleteHandler = async (
  client: MeowClient,
  interaction: AutocompleteInteraction,
): Promise<void> => {
  const _command = client.getCommands().find((c) => c.name === interaction.commandName);
  if (_command?.autocomplete) {
    try {
      await _command.autocomplete(client, interaction);
    } catch (err: unknown) {
      if (err instanceof Error) {
        client.logger.error("[SlashCommands/Autocomplete] " + err.message);
      }
    }
    return;
  }
};

const commandUserContextHandler = async (
  client: MeowClient,
  interaction: UserContextMenuCommandInteraction,
): Promise<void> => {
  const _command = client.getUserCommands().find((c) => c.name === interaction.commandName);
  if (_command) {
    try {
      await _command.run(client, new MeowInteraction(client, interaction, []));
    } catch (err: unknown) {
      if (err instanceof Error) {
        client.logger.error("[UserCommands] " + err.message);
      }
      await interactionErrorHandler(interaction, err);
    }
    return;
  }
};

const interactionCreate = async (client: MeowClient, interaction: Interaction) => {
  if (interaction.isChatInputCommand()) {
    await slashCommandHandler(client, interaction);
  } else if (interaction.isAutocomplete()) {
    await commandAutocompleteHandler(client, interaction);
  } else if (interaction.isUserContextMenuCommand()) {
    await commandUserContextHandler(client, interaction);
  }
};

export default (client: MeowClient) => {
  client.on("interactionCreate", (i: Interaction) => void interactionCreate(client, i));
};
