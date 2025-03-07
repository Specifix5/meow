import { Guild, Routes } from "discord.js";
import { MeowClient } from "../utils/nyan/client.js";
import { BOT_PRONOUNS } from "../utils/constants.js";

const guildJoin = async (client: MeowClient, guild: Guild) => {
  client.logger.info(`Joined guild: ${guild.name} (${guild.id})`);
  try {
    await new Promise((resolve) => setTimeout(resolve, 1000));
    await client.rest.patch(Routes.guildMember(guild.id, "@me"), {
      body: { pronouns: BOT_PRONOUNS },
      appendToFormData: true,
    });
  } catch (err: unknown) {
    if (err instanceof Error) {
      client.logger.error("[GuildJoin] " + err.message);
    }
  }
};

export default (client: MeowClient) => {
  client.on("guildCreate", (guild: Guild) => void guildJoin(client, guild));
};
