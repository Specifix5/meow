import { GuildMember, User } from "discord.js";

/**
 * Gets a user's tag, or their username if they have no discriminator.
 * @param user - The user to get the name for.
 * @returns The username or tag of the user.
 */
export const getUsername = (user: User | GuildMember): string => {
  if (user instanceof GuildMember) user = user.user;
  if (user.tag.endsWith("#0000")) return user.username;
  return user.tag;
};
