import {
  ApplicationCommandOptionData,
  ApplicationCommandOptionType,
  ApplicationIntegrationType,
  ImageURLOptions,
  InteractionContextType,
} from "discord.js";

export const MESSAGES = {
  ERROR_GENERIC: [
    "uh oh! something went wrong! sorry! :c",
    "oops! i seem to have stumbled upon a little problem! :T",
    "whoops! a tiny error has occurred! >.<",
    "something went awry! sorry about that! =w=",
    "a tiny hiccup! please forgive me! TwT",
  ],
  COMMAND_RESPONSE_HELP: [
    "i'm here to assist! :3",
    "here you go! hope this helps! ^w^",
    "need a hand? i'm happy to help! :3",
    "confused? don't worry, i'm here to guide you! c:",
    "let me lend a paw! >:3",
  ],
  COMMAND_RESPONSE_PING: [
    "pong!",
    "beep boop! i'm alive! ( •̀ ω •́ )y",
    "ping received! i'm ready to roll! >:3",
    "still here, ready for action! o((>ω< ))o",
    "hello! i'm online and doing great! /ᐠ｡ꞈ｡ᐟ\\",
  ],
  COMMAND_OPTION_UNIVERSAL_EPHMERAL: "Makes the response as an ephmeral.",
  COMMAND_OPTION_USER: "The target user.",
  ACTIONS_MESSAGES_HUG: [
    "{a} hugged {b}.",
    "{a} embraced {b} tightly.",
    "{a} held {b} close.",
    "{a} wrapped their arms around {b}.",
    "{a} gave {b} a warm hug.",
  ],
  ACTIONS_MESSAGES_KISS: [
    "{a} kissed {b}'s forehead.",
    "{a} kissed {b}'s cheek.",
    "{a} pecked {b}'s lips.",
    "{a} kissed {b} passionately.",
    "{a} stole a kiss from {b}.",
  ],
  ACTIONS_MESSAGES_SLAP: [
    "{a} slapped {b} playfully.",
    "{a} lightly slapped {b} on the arm.",
    "{a} slapped {b} hard.",
    "{a} slapped {b} on the shoulder.",
    "{a} slapped {b} playfully.",
  ],
  ACTIONS_MESSAGES_BITE: [
    "{a} bit {b}'s ear playfully.",
    "{a} bit {b}'s neck softly.",
    "{a} nibbled {b}'s lip.",
    "{a} gently bit {b}'s cheek.",
    "{a} bit {b}'s hand lightly.",
  ],
  ACTIONS_MESSAGES_PAT: [
    "{a} patted {b}'s head.",
    "{a} patted {b}'s back.",
    "{a} patted {b}'s hand.",
    "{a} patted {b}'s arm.",
    "{a} gently patted {b}'s cheek.",
  ],
  ACTIONS_MESSAGES_CUDDLE: [
    "{a} cuddled {b}.",
    "{a} snuggled close to {b}.",
    "{a} wrapped themselves around {b}.",
    "{a} lay close to {b}.",
    "{a} buried their face in {b}'s neck.",
  ],
  ACTIONS_MESSAGES_SHOOT: [
    "{a} looked at {b}.",
    "{a} glanced at {b}.",
    "{a} stared at {b}.",
    "{a} gave {b} a look.",
    "{a} shot {b} a look.",
  ],
  ACTIONS_MESSAGES_YEET: [
    "{a} tossed {b} into the air.",
    "{a} threw {b} up high.",
    "{a} flung {b} playfully.",
    "{a} sent {b} flying.",
    "{a} hurled {b} upwards.",
  ],
  ACTIONS_MESSAGES_TICKLE: [
    "{a} tickled {b} playfully.",
    "{a} tickled {b}'s sides.",
    "{a} tickled {b}'s feet.",
    "{a} tickled {b}'s stomach.",
    "{a} tickled {b}'s armpits.",
  ],
  ACTIONS_MESSAGES_STARE: [
    "{a} stared at {b}.",
    "{a} looked intently at {b}.",
    "{a} fixed {b} with a stare.",
    "{a} locked eyes with {b}.",
    "{a} gazed intensely at {b}.",
  ],
  ACTIONS_MESSAGES_POUT: [
    "{a} pouted.",
    "{a} made a pout.",
    "{a} frowned.",
    "{a} sulked.",
    "{a} looked disappointed.",
  ],
};

export const EMOJI = {
  ARROW_RIGHT: "<:_:1305532650836070420>",
  ARROW_LEFT: "<:_:1305533338701922324>",
  ARROW_BRANCH_END: " ╰",
  ARROW_BRANCH: "┠",
  ICON_HAMBURGER: "<:_:1306990190899757149>",
  ICON_SLASH: "<:_:1306989298444402698>",
  ICON_DENY: "<:_:1306994045872115712>",
  ICON_ALLOW: "<:_:1306994631094833286>",
  ICON_INFO: "<:_:1307647568184082522>",
  ICON_EMPTY: "ㅤ",
  ICON_OPTIONS: "<:_:1307669903221854299>",
};

export const BASE_URLS = {
  ACTIONS: "https://nekos.best/api/v2/",
};

export const DEV_USERS: string[] = ["485749822322769920"];

export const PREFIX = "m.";

export const MeowVERSION = process.env.npm_package_version;

export const NAME = process.env.npm_package_name;

export const COMMAND_PADDED_BOX_LENGTH = 7;

export const BOT_PRONOUNS = "nya/nyan";

export const EMBED_OPTIONS = {
  DEFAULT_COLOR: "#252525",
  DEFAULT_FOOTER: `${NAME} v${MeowVERSION}`,
};

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
  description: MESSAGES.COMMAND_OPTION_UNIVERSAL_EPHMERAL,
  type: ApplicationCommandOptionType.Boolean,
  required: false,
};

export const RequiredUserCommandOption: ApplicationCommandOptionData = {
  name: "user",
  description: MESSAGES.COMMAND_OPTION_USER,
  required: true,
  type: ApplicationCommandOptionType.User,
};

export const UserCommandOption: ApplicationCommandOptionData = {
  name: "user",
  description: MESSAGES.COMMAND_OPTION_USER,
  required: false,
  type: ApplicationCommandOptionType.User,
};

export const DefaultImageURLOptions: ImageURLOptions = {
  size: 4096,
  forceStatic: false,
};
