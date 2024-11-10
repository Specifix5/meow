import { Message, ApplicationCommandOptionData } from "discord.js";
import { PREFIX } from "../utils/constants";
import { ShoukoInteraction } from "../utils/shouko/command";
import { CreateErrorMessage } from "../utils/logging";
import { ShoukoClient } from "../utils/shouko/client";

export default (client: ShoukoClient) => {
  client.on("messageCreate", async (message: Message<boolean>) => {
    if (message.author.id === client.user!.id) return;

    if (message.cleanContent.startsWith(PREFIX)) {
      const interaction = new ShoukoInteraction(client, message, []);
      const _command = client.getCommands().find((c) => c.name === interaction.commandName);
      if (!_command) {
        await interaction.reply({
          content: CreateErrorMessage(new Error("No matching command by the name '" + interaction.commandName + "'")),
        });
        return;
      } else {
        try {
          const interaction = new ShoukoInteraction(
            client,
            message,
            _command.options as ApplicationCommandOptionData[],
          );
          await _command.run(client, interaction);
        } catch (err: any) {
          client.logger.error("[SlashCommands] " + err);
          await interaction.reply({
            content: CreateErrorMessage(err),
          });
        }
        return;
      }
    }
  });
};
