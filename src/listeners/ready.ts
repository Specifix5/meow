import { ApplicationCommandType } from "discord.js";
import { ShoukoClient } from "../utils/shouko/client";
import { Command, MessageCommand, UserCommand } from "../utils/shouko/command";

export default (client: ShoukoClient, commands: Array<Command | UserCommand | MessageCommand>) => {
  client.on("ready", async () => {
    client.application?.commands.set(commands).then((cmd) => {
      cmd.map((c) => {
        switch (c.type) {
          case ApplicationCommandType.ChatInput:
            client.setCommands(commands.filter((_c) => _c.name === c.name) as Array<Command>);
            break;
          case ApplicationCommandType.User:
            client.setUserCommands(commands.filter((_c) => _c.name === c.name) as Array<UserCommand>);
            break;
          case ApplicationCommandType.Message:
            client.setMessageCommands(commands.filter((_c) => _c.name === c.name) as Array<MessageCommand>);
            break;
          default:
            break;
        }
      });
      client.logger.info(
        `Successfully loaded ${cmd.size} commands! (${cmd.filter((c) => c.type === ApplicationCommandType.ChatInput).size} Slash, ${cmd.filter((c) => c.type === ApplicationCommandType.User).size} User, ${cmd.filter((c) => c.type === ApplicationCommandType.Message).size} Message)`,
      );
    });

    const guilds = await client.guilds.fetch();
    client.logger.info("Logged in as " + client?.user?.tag);
    client.logger.info(`Guilds (${guilds.size}): ${guilds.map((g) => g.name).join(", ")}`);
  });
};
