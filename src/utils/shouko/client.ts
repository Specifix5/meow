import { Client, ClientOptions } from "discord.js";
import { getLogger, GetString, Logger, getString } from "../logging.js";
import {
  getCommands,
  getMessageCommands,
  getUserCommands,
  setCommands,
  setMessageCommands,
  setUserCommands,
} from "./command.js";

export class ShoukoClient extends Client {
  logger: Logger;
  getString: GetString;
  setCommands: setCommands;
  setUserCommands: setUserCommands;
  setMessageCommands: setMessageCommands;
  getCommands: getCommands;
  getUserCommands: getUserCommands;
  getMessageCommands: getMessageCommands;
  commandCategories: string[];

  constructor(options: ClientOptions) {
    super(options);
    this.logger = getLogger();
    this.getString = getString;
    this.setCommands = setCommands;
    this.setUserCommands = setUserCommands;
    this.setMessageCommands = setMessageCommands;
    this.getCommands = getCommands;
    this.getUserCommands = getUserCommands;
    this.getMessageCommands = getMessageCommands;
    this.commandCategories = [];
  }
}
