import {
  CommandInteraction,
  UserContextMenuCommandInteraction,
  ApplicationCommandOptionData,
  AutocompleteInteraction,
  Interaction,
} from "discord.js";
import { CreateErrorMessage } from "../utils/logging";
import { ShoukoClient } from "../utils/shouko/client";
import { ShoukoInteraction } from "../utils/shouko/command";

export const interactionErrorHandler = async (
  interaction: CommandInteraction | UserContextMenuCommandInteraction,
  err: any,
) => {
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
};

const slashCommandHandler = async (client: ShoukoClient, interaction: CommandInteraction): Promise<void> => {
  const _command = client.getCommands().find((c) => c.name === interaction.commandName);
  if (!_command) {
    await interaction.reply(
      CreateErrorMessage(new Error("No matching command by the name '" + interaction.commandName + "'")),
    );
    return;
  } else {
    try {
      await _command.run(
        client,
        new ShoukoInteraction(client, interaction, _command.options as ApplicationCommandOptionData[]),
      );
    } catch (err: any) {
      client.logger.error("[SlashCommands] " + err);
      await interactionErrorHandler(interaction, err);
    }
    return;
  }
};

const commandAutocompleteHandler = async (
  client: ShoukoClient,
  interaction: AutocompleteInteraction,
): Promise<void> => {
  const _command = client.getCommands().find((c) => c.name === interaction.commandName);
  if (_command && _command.autocomplete) {
    try {
      await _command.autocomplete(client, interaction);
    } catch (err: any) {
      client.logger.error("[SlashCommands/Autocomplete] " + err);
    }
    return;
  }
};

const commandUserContextHandler = async (
  client: ShoukoClient,
  interaction: UserContextMenuCommandInteraction,
): Promise<void> => {
  const _command = client.getUserCommands().find((c) => c.name === interaction.commandName);
  if (_command) {
    try {
      await _command.run(client, new ShoukoInteraction(client, interaction, []));
    } catch (err: any) {
      client.logger.error("[UserCommands] " + err);
      await interactionErrorHandler(interaction, err);
    }
    return;
  }
};

export default (client: ShoukoClient) => {
  client.on("interactionCreate", async (interaction: Interaction) => {
    if (interaction.isChatInputCommand()) {
      await slashCommandHandler(client, interaction);
    } else if (interaction.isAutocomplete()) {
      await commandAutocompleteHandler(client, interaction);
    } else if (interaction.isUserContextMenuCommand()) {
      await commandUserContextHandler(client, interaction);
    }
  });
};
