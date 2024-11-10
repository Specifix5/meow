import {
  ApplicationCommandOptionData,
  ApplicationCommandOptionType,
  ApplicationIntegrationType,
  InteractionContextType,
} from "discord.js";

export const MESSAGES = {
  ERROR_GENERIC: [
    "Uhhhh, something has gone wrong!!!! sorry!!",
    "I've caught an error!!",
    "Oopsies, I might have done something wrong 3:",
    'Sorry!!!!!! (T^T ")',
    "Eeep!",
  ],
  COMMANDOPTION_UNIVERSAL_EPHMERAL: "Makes the response as an ephmeral.",
};

export const DEV_USERS: string[] = ["485749822322769920"];

export const PREFIX: string = "m.";

export const SHOUKOVERSION = process.env.npm_package_version;

export const UniversalContextType: InteractionContextType[] = [
  InteractionContextType.BotDM,
  InteractionContextType.Guild,
  InteractionContextType.PrivateChannel,
];

export const UniversalIntegrationType: ApplicationIntegrationType[] = [
  ApplicationIntegrationType.GuildInstall,
  ApplicationIntegrationType.UserInstall,
];

export const EphmeralCommandOption: ApplicationCommandOptionData = {
  name: "ephmeral",
  description: MESSAGES.COMMANDOPTION_UNIVERSAL_EPHMERAL,
  type: ApplicationCommandOptionType.Boolean,
  required: false,
};

export const TranslateApplicationCommandOptionType: object = {
  "1": "Subcommand",
  "2": "SubcommandGroup",
  "3": "String",
  "4": "Integer",
  "5": "Boolean",
  "6": "User",
  "7": "Channel",
  "8": "Role",
  "9": "Mentionable",
  "10": "Number",
  "11": "Attachment",
};
