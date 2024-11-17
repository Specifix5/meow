import { APIEmbed, ColorResolvable, EmbedBuilder } from "discord.js";
import { EMBED_COLOR, NAME, SHOUKOVERSION } from "../constants.js";

export class ShoukoEmbed extends EmbedBuilder {
  constructor(data?: APIEmbed) {
    super(data);
    this.setColor(EMBED_COLOR as ColorResolvable);
    this.setFooter({
      text: `${NAME} v${SHOUKOVERSION}`,
    });
  }
}
