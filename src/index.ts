import { GatewayIntentBits, Partials } from "discord.js";
import { DefaultWebSocketManagerOptions } from "discord.js";
import * as dotenv from "dotenv";
import thisWillLoadEverythingFr from "./utils/loadEverythingForMePleaseOnegaishimasu.js";
import { MeowClient } from "./utils/nyan/client.js";
import { NAME, MeowVERSION } from "./utils/constants.js";

dotenv.config({ path: ".env" });

(DefaultWebSocketManagerOptions.identifyProperties as { browser: string }).browser =
  "Discord Android";
(DefaultWebSocketManagerOptions.identifyProperties as { os: string }).os = "shouko";
(DefaultWebSocketManagerOptions.identifyProperties as { device: string }).device = "shouko";

const client: MeowClient = new MeowClient({
  intents: [
    GatewayIntentBits.Guilds,
    GatewayIntentBits.MessageContent,
    GatewayIntentBits.GuildMessages,
    GatewayIntentBits.DirectMessages,
  ],
  partials: [Partials.Channel, Partials.User],
});

client.logger.info(
  "Loading " + NAME + " " + MeowVERSION + ", attempting to connect to discord gateway",
);

await thisWillLoadEverythingFr(client);

client
  .login(process.env.CLIENT_TOKEN)
  .catch((err) => client.logger.error("Unable to log in: " + err));
