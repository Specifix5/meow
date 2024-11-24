import {
  ApplicationCommandOptionType,
  ApplicationCommandType,
  InteractionContextType,
  ApplicationIntegrationType,
  User,
} from "discord.js";
import {
  DefaultImageURLOptions,
  EphmeralCommandOption,
  UniversalContextType,
  UniversalIntegrationType,
  UserCommandOption,
} from "../../utils/constants.js";
import { MeowClient } from "../../utils/nyan/client.js";
import { MeowInteraction, Command, UserCommand } from "../../utils/nyan/command.js";
import { getUsername } from "../../utils/nyan/helpers.js";
import { MeowEmbed } from "../../utils/nyan/embed.js";

const generateMessage = async (_client: MeowClient, interaction: MeowInteraction) => {
  const target = interaction.targetUser ?? interaction.getOption<User>("user") ?? interaction.user;
  const guildProfile = interaction.getOption<boolean>("guild") && interaction.inGuild();

  const avatarURL = guildProfile
    ? interaction.guild!.members.resolve(target.id)!.displayAvatarURL(DefaultImageURLOptions)
    : target.displayAvatarURL(DefaultImageURLOptions);

  const avatarEmbed = new MeowEmbed()
    .setTitle(`${getUsername(target)}'s avatar`)
    .setImage(avatarURL)
    .setURL(avatarURL);

  await interaction.reply({
    embeds: [avatarEmbed],
    ephemeral: interaction.getOption<boolean>("ephmeral") ?? false,
  });
};

export const userAvatar: Command = {
  name: "avatar",
  description: "Get a user's avatar and (optionally) their guild avatar",
  contexts: UniversalContextType,
  integrationTypes: UniversalIntegrationType,
  options: [
    UserCommandOption,
    {
      name: "guild",
      type: ApplicationCommandOptionType.Boolean,
      description: "Show user profile or guild profile?",
    },
    EphmeralCommandOption,
  ],
  run: generateMessage,
};

export const userAvatarContext: UserCommand = {
  name: "View Avatar",
  contexts: UniversalContextType,
  integrationTypes: UniversalIntegrationType,
  type: ApplicationCommandType.User,
  run: generateMessage,
};

export const guildAvatarContext: UserCommand = {
  name: "View Server Avatar",
  contexts: [InteractionContextType.Guild],
  integrationTypes: [ApplicationIntegrationType.GuildInstall],
  type: ApplicationCommandType.User,
  run: generateMessage,
};

export default [userAvatar, userAvatarContext, guildAvatarContext];
