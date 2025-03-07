import { ApplicationCommandOptionType, User } from "discord.js";
import { MeowClient } from "../../utils/nyan/client.js";
import { Command, MeowInteraction } from "../../utils/nyan/command.js";
import {
  BASE_URLS,
  EMBED_OPTIONS,
  EMOJI,
  UniversalContextType,
  UniversalIntegrationType,
  RequiredUserCommandOption,
} from "../../utils/constants.js";
import { CreateErrorMessage } from "../../utils/logging.js";
import { MeowEmbed } from "../../utils/nyan/embed.js";
import { APIActionResult } from "../../utils/types.js";

enum Action {
  Hug = "hug",
  Kiss = "kiss",
  Slap = "slap",
  Bite = "bite",
  Pat = "pat",
  Cuddle = "cuddle",
  Shoot = "shoot",
  Yeet = "yeet",
  Tickle = "tickle",
  Stare = "stare",
  Pout = "pout",
}

const ActionCommand: Command = {
  name: "action",
  description: "Action to perform (e.g Hug, Kiss) to the user",
  options: [
    {
      name: "name",
      description: "The action to perform",
      required: true,
      type: ApplicationCommandOptionType.String,
      choices: Object.values(Action).map((a) => ({ name: a, value: a })),
    },
    RequiredUserCommandOption,
  ],
  contexts: UniversalContextType,
  integrationTypes: UniversalIntegrationType,
  run: async (client: MeowClient, interaction: MeowInteraction) => {
    const target = interaction.getOption<User>("user");
    if (!target) {
      await interaction.reply({
        content: CreateErrorMessage(new Error("User not found")),
        ephemeral: true,
      });
      return;
    }

    const action = interaction.getOption<string>("name")!;
    const req = await fetch(new URL(action, BASE_URLS.ACTIONS));

    if (!req.ok) {
      client.logger.error("[ActionCommand] Error getting action image");
      await interaction.reply({
        content: CreateErrorMessage(new Error("Failed to fetch action image")),
        ephemeral: true,
      });
      return;
    }

    const message = client
      .getString(`ACTIONS_MESSAGES_${action.toUpperCase()}`)
      ?.replaceAll("{a}", `<@${interaction.user.id}>`)
      .replaceAll("{b}", `<@${target.id}>`);

    const {
      results: [{ url, anime_name }],
    } = (await req.json()) as APIActionResult;

    await interaction.reply({
      content: message,
      embeds: [
        new MeowEmbed().setImage(url).setFooter({
          text: `source: ${anime_name}${EMOJI.ICON_EMPTY}|${EMOJI.ICON_EMPTY}${EMBED_OPTIONS.DEFAULT_FOOTER}`,
        }),
      ],
    });
  },
};

export default ActionCommand;
