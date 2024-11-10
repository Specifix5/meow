import fs from "fs";
import { ShoukoClient } from "./shouko/client";
import { Command, MessageCommand, UserCommand } from "./shouko/command";
import path from "path";

export default (client: ShoukoClient) => {
  const commandFolder = path.join(__dirname, "..", "commands");
  const listenerFolder = path.join(__dirname, "..", "listeners");
  const loadedListeners: Array<string> = [];
  const loadedCommands: Array<Command | UserCommand | MessageCommand> = [];

  fs.readdirSync(commandFolder).map((dir) => {
    if (fs.statSync(path.join(commandFolder, dir)).isDirectory()) {
      fs.readdirSync(path.join(commandFolder, dir)).map((file) => {
        if (file.endsWith(".js") || file.endsWith(".ts")) {
          const moduleName = file.replace(/\.(ts|js)$/, "");
          const cPath = path.join(commandFolder, dir, moduleName + ".js");
          import(cPath).then((command) => {
            try {
              if (!command.default) throw new Error("There is no export for this command module.");
              loadedCommands.push(...command.default);
            } catch (err: any) {
              client.logger.error("Unable to load command module " + moduleName + ": " + err);
            }
          });
        }
      });
    }
  });

  fs.readdirSync(listenerFolder).map((file) => {
    if (file.endsWith(".js") || file.endsWith(".ts")) {
      const moduleName = file.replace(/\.(ts|js)$/, "");
      const lPath = path.join(listenerFolder, moduleName + ".js");
      import(lPath).then((listener) => {
        try {
          switch (moduleName) {
            case "ready":
              listener.default(client, loadedCommands);
              break;
            default:
              listener.default(client);
              break;
          }

          loadedListeners.push(moduleName);
        } catch (err: any) {
          client.logger.error("Unable to load listener module " + moduleName + ": " + err);
        }
      });
    }
  });
};
