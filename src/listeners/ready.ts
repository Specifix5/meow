import { ApplicationCommand, ApplicationCommandType, OAuth2Guild } from "discord.js";
import { MeowClient } from "../utils/nyan/client.js";
import { Command, MessageCommand, UserCommand } from "../utils/nyan/command.js";

const ready = async (client: MeowClient, commands: (Command | UserCommand | MessageCommand)[]) => {
  await client.application?.commands.set(commands).then((cmd) => {
    const slashCommands: Command[] = [];
    const userCommands: UserCommand[] = [];
    const messageCommands: MessageCommand[] = [];
    cmd.map((c: ApplicationCommand) => {
      const command = commands.find((_c) => _c.name === c.name);
      if (command) {
        command.id = c.id;
        switch (c.type) {
          case ApplicationCommandType.ChatInput:
            slashCommands.push(command as Command);
            break;
          case ApplicationCommandType.User:
            userCommands.push(command as UserCommand);
            break;
          case ApplicationCommandType.Message:
            messageCommands.push(command as MessageCommand);
            break;
          default:
            break;
        }
      }
    });

    client.setCommands(slashCommands);
    client.setUserCommands(userCommands);
    client.setMessageCommands(messageCommands);

    client.logger.info(
      `Successfully loaded ${cmd.size} commands! (${cmd.filter((c: ApplicationCommand) => c.type === ApplicationCommandType.ChatInput).size} Slash, ${cmd.filter((c) => c.type === ApplicationCommandType.User).size} User, ${cmd.filter((c) => c.type === ApplicationCommandType.Message).size} Message)`,
    );
  });

  const guilds = await client.guilds.fetch();
  client.logger.info("Logged in as " + client?.user?.tag);
  client.logger.info(
    `Guilds (${guilds.size}): ${guilds.map((g: OAuth2Guild) => g.name).join(", ")}`,
  );
};

export default (client: MeowClient, commands: (Command | UserCommand | MessageCommand)[]) => {
  client.on("ready", () => void ready(client, commands));
};
