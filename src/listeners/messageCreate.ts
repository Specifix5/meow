import { Message, ApplicationCommandOptionData } from "discord.js";
import { PREFIX } from "../utils/constants.js";
import { ShoukoInteraction } from "../utils/shouko/command.js";
import { CreateErrorMessage } from "../utils/logging.js";
import { ShoukoClient } from "../utils/shouko/client.js";

const messageCreate = async (client: ShoukoClient, message: Message<boolean>) => {
  if (message.author.id === client.user!.id) return;

  if (message.cleanContent.startsWith(PREFIX)) {
    const interaction = new ShoukoInteraction(client, message, []);
    const _command = client.getCommands().find((c) => c.name === interaction.commandName);
    if (!_command) {
      await interaction.reply({
        content: CreateErrorMessage(
          new Error("No matching command by the name '" + interaction.commandName + "'"),
        ),
      });
      return;
    }
    try {
      const interaction = new ShoukoInteraction(
        client,
        message,
        _command.options as ApplicationCommandOptionData[],
      );
      await _command.run(client, interaction);
    } catch (err: unknown) {
      if (err instanceof Error) {
        client.logger.error("[SlashCommands] " + err.message);
        await interaction.reply({
          content: CreateErrorMessage(err),
        });
      }
    }
  }
};

export default (client: ShoukoClient) => {
  client.on("messageCreate", (message: Message<boolean>) => void messageCreate(client, message));
};
