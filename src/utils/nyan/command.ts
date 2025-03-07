import {
  ApplicationCommandOptionData,
  ApplicationCommandOptionType,
  AutocompleteInteraction,
  BaseApplicationCommandData,
  CacheType,
  ChatInputApplicationCommandData,
  CommandInteraction,
  CommandInteractionOptionResolver,
  Guild,
  GuildMember,
  InteractionReplyOptions,
  InteractionResponse,
  Message,
  MessageApplicationCommandData,
  MessageContextMenuCommandInteraction,
  MessageCreateOptions,
  MessagePayload,
  TextBasedChannel,
  User,
  UserApplicationCommandData,
  UserContextMenuCommandInteraction,
} from "discord.js";
import { PREFIX } from "../constants.js";
import { MeowClient } from "./client.js";
import { ParsedOptions, CommandOptionValue } from "../types.js";

export interface BaseCommand extends BaseApplicationCommandData {
  category?: string;
  id?: string;
}
export interface Command extends ChatInputApplicationCommandData, BaseCommand {
  run: (_client: MeowClient, _interaction: MeowInteraction) => Promise<void>;
  autocomplete?: (_client: MeowClient, _interaction: AutocompleteInteraction) => Promise<void>;
}

export interface UserCommand extends UserApplicationCommandData, BaseCommand {
  run: (_client: MeowClient, _interaction: MeowInteraction) => Promise<void>;
}

export interface MessageCommand extends MessageApplicationCommandData, BaseCommand {
  run: (_client: MeowClient, _interaction: MessageContextMenuCommandInteraction) => Promise<void>;
}

export class MeowInteraction {
  client: MeowClient;
  context: Message | CommandInteraction | UserContextMenuCommandInteraction;
  options?: CommandInteractionOptionResolver<CacheType> | ParsedOptions;
  guild: Guild | null;
  user: User;
  commandName: string;
  message: Message<boolean> | null;
  id: string;
  targetMember?: GuildMember;
  targetUser?: User;
  targetId?: string;

  constructor(
    client: MeowClient,
    context: Message | CommandInteraction | UserContextMenuCommandInteraction,
    _options: ApplicationCommandOptionData[],
  ) {
    this.client = client;
    this.context = context;
    this.user = this.getUser();
    this.guild = this.getGuild();
    this.message = null;
    this.id = this.context.id;
    if (this.isInteraction(context)) {
      this.options = context.options as CommandInteractionOptionResolver<CacheType>;
      this.commandName = (this.context as CommandInteraction).commandName;
    } else if (this.isUserContext(context)) {
      this.commandName = (this.context as UserContextMenuCommandInteraction).commandName;
      this.targetUser = (this.context as UserContextMenuCommandInteraction).targetUser as
        | User
        | undefined;
      this.targetMember = (this.context as UserContextMenuCommandInteraction).targetMember as
        | GuildMember
        | undefined;
      this.targetId = this.targetId = (this.context as UserContextMenuCommandInteraction).targetId;
    } else {
      this.commandName = parseRawArgs(context.content.trim().toLowerCase().slice(PREFIX.length))[0];
      const args = parseMessageArgs(
        client,
        parseRawArgs(context.content.trim()).slice(1),
        _options,
      );
      this.options = args;
      if (process.env.DEBUG_MODE)
        client.logger.info(
          `HybridCommandArgs [${this.commandName} (${this.user.username})]: ` +
            JSON.stringify(args),
        );
    }
  }
  inGuild(): boolean {
    return this.context.guild !== null;
  }

  getUser(): User {
    if ("author" in this.context) {
      return this.context.author;
    }
    return this.context.user;
  }

  getChannel(): TextBasedChannel | null {
    return this.context.channel;
  }

  getGuild(): Guild | null {
    return this.context.guild;
  }

  getMember(): GuildMember | null {
    if ("member" in this.context) {
      return this.context.member! as GuildMember;
    }
    return null;
  }

  isUserContextMenu(): boolean {
    return this.isUserContext(this.context);
  }

  isMessageBased(): boolean {
    return this.isMessage(this.context);
  }

  isChatInput(): boolean {
    return this.isInteraction(this.context);
  }

  // Type guard to check if context is a Message
  private isMessage(context: Message | CommandInteraction): context is Message {
    return (context as Message).author !== undefined;
  }

  // Type guard for interaction
  private isInteraction(
    context: CommandInteraction | Message | UserContextMenuCommandInteraction,
  ): context is CommandInteraction<CacheType> {
    return "options" in context;
  }

  // Type guard for usercontextinteraction
  private isUserContext(
    context: CommandInteraction | Message | UserContextMenuCommandInteraction,
  ): context is UserContextMenuCommandInteraction<CacheType> {
    return "targetUser" in context;
  }

  getOption<T extends CommandOptionValue>(name: string): T | null {
    if (this.isInteraction(this.context)) {
      const option =
        this?.options !== null
          ? (this.options as CommandInteractionOptionResolver<CacheType>).get(name)
          : null;
      if (!option) return null;

      // Type checking based on ApplicationCommandOptionType
      switch (option.type) {
        case ApplicationCommandOptionType.User:
          return option.user as T;
        case ApplicationCommandOptionType.Boolean:
          return option.value as T; // boolean
        case ApplicationCommandOptionType.String:
          return option.value as T; // string
        case ApplicationCommandOptionType.Channel:
          return option.channel as T;
        case ApplicationCommandOptionType.Subcommand:
          return option.name as T;
        default:
          return null;
      }
    } else {
      // Handle message-based command option retrieval (map args to options manually)
      const value = (this.options as ParsedOptions)[name]; // Handle string-indexing safely
      if (value === undefined || value === null) return null;
      return value as T;
    }
  }

  // Method to reply (handles both Message and Interaction)
  async reply(content: MessagePayload | InteractionReplyOptions) {
    if (this.isInteraction(this.context)) {
      if (!this.context.isRepliable()) throw new Error("Interaction has already been replied.");
      await this.context.reply(content as InteractionReplyOptions);
    } else if (this.isMessage(this.context)) {
      (content as MessageCreateOptions).allowedMentions = { repliedUser: false };
      this.message = await this.context.reply(content as MessageCreateOptions);
    } else {
      throw new Error("Cannot reply to the command context");
    }
  }

  /**
   * Sends a follow-up message to the interaction or command context.
   *
   * If the context is an interaction and it has not been replied yet, it throws an error.
   *
   * If the context is a message and it has already been replied to, it edits the existing message.
   * Otherwise, it sends a new reply to the message.
   *
   * If the context is neither an interaction nor a message, it throws an error.
   *
   * @param content The content of the message.
   * @returns The sent message or interaction response.
   */
  async followUp(
    content: MessageCreateOptions | MessagePayload | InteractionReplyOptions,
  ): Promise<InteractionResponse<boolean> | Message<boolean>> {
    if (this.isInteraction(this.context)) {
      if (!(this.context.replied || this.context.deferred))
        throw new Error("Interaction not replied yet.");
      return this.context.followUp(content as InteractionReplyOptions);
    } else if (this.isMessage(this.context)) {
      if (this.message) {
        (content as MessageCreateOptions).allowedMentions = { repliedUser: false };
        return this.message.edit(content as MessagePayload);
      }
      (content as MessageCreateOptions).allowedMentions = { repliedUser: false };
      return this.context.reply(content as MessageCreateOptions);
    }
    throw new Error("Cannot followUp to the command context");
  }

  /**
   * Sends a deferred reply to the interaction or command context.
   *
   * If the context is an interaction and it has already been replied to, it throws an error.
   *
   * If the context is a message, it sends a new reply to the message with the content of "Meow is thinking..".
   * Otherwise, it throws an error.
   *
   * @param content The content of the message.
   * @returns The sent message or interaction response.
   */
  async deferReply(
    content: MessageCreateOptions | MessagePayload | InteractionReplyOptions,
  ): Promise<InteractionResponse<boolean> | Message<boolean>> {
    if (this.isInteraction(this.context)) {
      if (!this.context.isRepliable()) throw new Error("Interaction has already been replied.");
      return this.context.deferReply(content as InteractionReplyOptions);
    } else if (this.isMessage(this.context)) {
      (content as MessageCreateOptions).content = "Meow is thinking..";
      (content as MessageCreateOptions).allowedMentions = { repliedUser: false };
      this.message = await this.context.reply(content as MessageCreateOptions);
      return this.message;
    }
    throw new Error("Cannot editReply to the command context");
  }

  async editReply(
    content: MessageCreateOptions | MessagePayload | InteractionReplyOptions,
  ): Promise<Message<boolean>> {
    if (this.isInteraction(this.context)) {
      if (!(this.context.replied || this.context.deferred))
        throw new Error("Interaction not replied yet.");
      return this.context.editReply(content as InteractionReplyOptions);
    } else if (this.isMessage(this.context)) {
      if (!this.message) throw new Error("Interaction not replied yet.");
      (content as MessageCreateOptions).allowedMentions = { repliedUser: false };
      return this.message.edit(content as MessagePayload);
    }
    throw new Error("Cannot editReply to the command context");
  }
}

const parseRawArgs = (input: string): string[] => {
  const regex = /\[\[(.*?)\]\]|(\S+)/g;
  const args = [];
  let match;

  while ((match = regex.exec(input)) !== null) {
    args.push(match[1] || match[2]);
  }
  return args;
};

const parseMessageArgs = (
  client: MeowClient,
  args: string[],
  commandOptions: ApplicationCommandOptionData[],
): ParsedOptions => {
  const options: ParsedOptions = {};
  try {
    if (!commandOptions) commandOptions = [];
    commandOptions.forEach((option, index) => {
      const arg = args[index];
      if ((option as { required: boolean }).required && (arg === undefined || arg === null))
        throw new Error("Missing required arguments: " + option.name);
      // Parse and store the argument based on the type defined in the command options
      switch (option.type) {
        case ApplicationCommandOptionType.User:
          {
            if (!arg) {
              options[option.name] = null;
              break;
            }
            const userId = arg.replace(/([^0-9]+)/g, "");
            const isValid = /^[0-9]+$/.test(userId);
            options[option.name] = isValid
              ? (client.users.cache.get(userId) ?? client.users.fetch(userId))
              : null;
          }
          break;
        case ApplicationCommandOptionType.Channel:
          const channelId = arg.replace(/([^0-9]+)/g, "");
          const isValid = /^[0-9]+$/.test(channelId);
          options[option.name] = isValid
            ? (client.channels.cache.get(channelId) ?? client.channels.fetch(channelId))
            : null;
        case ApplicationCommandOptionType.Boolean:
          options[option.name] = arg ? arg.toLowerCase() === "true" : false;
          break;
        case ApplicationCommandOptionType.String:
          options[option.name] = arg ?? null;
          break;
        case ApplicationCommandOptionType.Subcommand:
          options[option.name] = arg && arg.toLowerCase() === option.name ? true : null;
          break;
        default:
          options[option.name] = null;
          break;
      }
    });
  } catch (err: unknown) {
    if (err instanceof Error) {
      client.logger.error("ParseMessageArgs " + err.message);
    }
    return {};
  }
  return options;
};

let Commands: Command[] = [];

let UserCommands: UserCommand[] = [];

let MessageCommands: MessageCommand[] = [];

export const setCommands: setCommands = (commands: Command[]) => {
  Commands = commands;
};

export const setUserCommands: setUserCommands = (commands: UserCommand[]) => {
  UserCommands = commands;
};

export const setMessageCommands: setMessageCommands = (commands: MessageCommand[]) => {
  MessageCommands = commands;
};

export const getCommands: getCommands = (): Command[] => Commands;
export const getUserCommands: getUserCommands = (): UserCommand[] => UserCommands;
export const getMessageCommands: getMessageCommands = (): MessageCommand[] => MessageCommands;

export type setCommands = (commands: Command[]) => void;

export type setUserCommands = (commands: UserCommand[]) => void;

export type setMessageCommands = (commands: MessageCommand[]) => void;

export type getCommands = () => Command[];

export type getUserCommands = () => UserCommand[];

export type getMessageCommands = () => MessageCommand[];
