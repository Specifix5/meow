import {
  ApplicationCommandOptionData,
  ApplicationCommandOptionType,
  ApplicationIntegrationType,
  InteractionContextType,
} from "discord.js";

export const MESSAGES = {
  ERROR_GENERIC: [
    "uhhhh, something has gone wrong!!!! sorry!!",
    "i've caught an error!!",
    "oopsies, I might have done something wrong 3:",
    'sorry!!!!!! (T^T ")',
    "eeep!",
  ],
  COMMANDOPTION_UNIVERSAL_EPHMERAL: "Makes the response as an ephmeral.",
  COMMANDRESPONSE_HELP: [
    "assistance at your service",
    "here's a helping hand",
    "confused, don't worry!",
  ],
};

export const EMOJI = {
  ARROW_RIGHT: "<:_:1305532650836070420>",
  ARROW_LEFT: "<:_:1305533338701922324>",
  ARROW_BRANCH: "<:_:1306089693057912862>",
  ICON_HAMBURGER: "<:_:1306990190899757149>",
  ICON_SLASH: "<:_:1306989298444402698>",
  ICON_DENY: "<:_:1306994045872115712>",
  ICON_ALLOW: "<:_:1306994631094833286>",
  ICON_INFO: "<:_:1307647568184082522>",
  ICON_EMPTY: "<:_:1307648427751899218>",
  ICON_OPTIONS: "<:_:1307669903221854299>",
};

export const DEV_USERS: string[] = ["485749822322769920"];

export const EMBED_COLOR = "#252525";

export const PREFIX = "m.";

export const SHOUKOVERSION = process.env.npm_package_version;

export const NAME = process.env.npm_package_name;

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
