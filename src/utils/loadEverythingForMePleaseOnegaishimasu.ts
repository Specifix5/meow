import fs from "fs";
import { MeowClient } from "./nyan/client.js";
import { Command, MessageCommand, UserCommand } from "./nyan/command.js";
import path from "path";
import { CommandModule, ListenerModule } from "./types.js";
import { pathToFileURL } from "url";

export default async (client: MeowClient) => {
  const commandFolder = path.join(import.meta.dirname, "..", "commands");
  const listenerFolder = path.join(import.meta.dirname, "..", "listeners");
  const loadedListeners: string[] = [];
  const loadedCommands: (Command | UserCommand | MessageCommand)[] = [];

  await Promise.all(
    fs.readdirSync(commandFolder).map(async (dir) => {
      if (fs.statSync(path.join(commandFolder, dir)).isDirectory()) {
        const categoryName = dir[0].toUpperCase() + dir.slice(1);
        client.commandCategories.push(categoryName);

        await Promise.all(
          fs.readdirSync(path.join(commandFolder, dir)).map(async (file) => {
            if (file.endsWith(".js") || file.endsWith(".ts")) {
              const moduleName = file.replace(/\.(ts|js)$/, "");
              const cPath = pathToFileURL(path.join(commandFolder, dir, moduleName + ".js")).href;
              try {
                const command: CommandModule = (await import(cPath)) as CommandModule;
                if (!command.default)
                  throw new Error("There is no export for this command module.");
                if (Array.isArray(command.default)) {
                  command.default.forEach((cmd: Command | UserCommand | MessageCommand) => {
                    cmd.category = categoryName;
                    loadedCommands.push(cmd);
                  });
                } else {
                  const cmd = command.default;
                  cmd.category = categoryName;
                  loadedCommands.push(cmd);
                }
              } catch (err: unknown) {
                client.logger.error(
                  "Unable to load command module " + moduleName + ": " + String(err),
                );
              }
            }
          }),
        );
      }
    }),
  );

  await Promise.all(
    fs.readdirSync(listenerFolder).map(async (file) => {
      if (file.endsWith(".js") || file.endsWith(".ts")) {
        const moduleName = file.replace(/\.(ts|js)$/, "");
        const lPath = pathToFileURL(path.join(listenerFolder, moduleName + ".js")).href;
        client.logger.info("Loading listener module " + moduleName);
        try {
          const listenerModule: ListenerModule = (await import(lPath)) as ListenerModule;
          switch (moduleName) {
            case "ready":
              listenerModule.default(client, loadedCommands);
              break;
            default:
              listenerModule.default(client);
              break;
          }

          loadedListeners.push(moduleName);
        } catch (err: unknown) {
          if (err instanceof Error) {
            client.logger.error(
              "Unable to load listener module " + moduleName + ": " + err.message,
            );
          }
        }
      }
    }),
  );
};
