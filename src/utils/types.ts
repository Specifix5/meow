import { Attachment, Channel, GuildMember, User } from "discord.js";
import { ShoukoClient } from "./shouko/client.js";
import { Command, MessageCommand, UserCommand } from "./shouko/command.js";

export interface CommandModule {
  default: Command | UserCommand | MessageCommand | (Command | UserCommand | MessageCommand)[];
}

export interface ListenerModule {
  default: (client: ShoukoClient, commands?: (Command | UserCommand | MessageCommand)[]) => void;
}

/**
 * Type-safe utility for converting arguments.
 * This type represents an object where the keys are strings and the values are of type CommandOptionValue.
 */
export type ParsedOptions = Record<string, CommandOptionValue>;

export type CommandOptionValue =
  | string
  | number
  | boolean
  | User
  | Promise<User>
  | GuildMember
  | Promise<GuildMember>
  | Channel
  | Promise<Channel | null>
  | Attachment
  | null;

export enum TranslateApplicationCommandOptionType {
  "subcommand",
  "subcommandGroup",
  "string",
  "integer",
  "boolean",
  "user",
  "channel",
  "role",
  "mentionable",
  "number",
  "attachment",
}
