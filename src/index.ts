import { GatewayIntentBits, Partials, REST } from "discord.js";
import { DefaultWebSocketManagerOptions } from "discord.js";
import * as dotenv from "dotenv";
import thisWillLoadEverythingFr from "./utils/loadEverythingForMePleaseOnegaishimasu.js";
import { ShoukoClient } from "./utils/shouko/client.js";
import { SHOUKOVERSION } from "./utils/constants.js";

dotenv.config({ path: ".env" });

(DefaultWebSocketManagerOptions.identifyProperties as { browser: string }).browser =
  "Discord Android";
(DefaultWebSocketManagerOptions.identifyProperties as { os: string }).os = "shouko";
(DefaultWebSocketManagerOptions.identifyProperties as { device: string }).device = "shouko";

const client: ShoukoClient = new ShoukoClient({
  intents: [
    GatewayIntentBits.Guilds,
    GatewayIntentBits.MessageContent,
    GatewayIntentBits.GuildMessages,
    GatewayIntentBits.DirectMessages,
  ],
  partials: [Partials.Channel, Partials.User],
});

client.logger.info(
  "Loading Maidbot " + SHOUKOVERSION + ", attempting to connect to discord gateway",
);

await thisWillLoadEverythingFr(client);

export const restClient = new REST({ version: "10" }).setToken(process.env.CLIENT_TOKEN!);

client
  .login(process.env.CLIENT_TOKEN)
  .catch((err) => client.logger.error("Unable to log in: " + err));
