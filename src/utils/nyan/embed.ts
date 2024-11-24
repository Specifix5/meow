import { APIEmbed, ColorResolvable, EmbedBuilder } from "discord.js";
import { EMBED_OPTIONS } from "../constants.js";

export class MeowEmbed extends EmbedBuilder {
  constructor(data?: APIEmbed) {
    super(data);
    this.setColor(EMBED_OPTIONS.DEFAULT_COLOR as ColorResolvable);
    this.setFooter({
      text: EMBED_OPTIONS.DEFAULT_FOOTER,
    });
  }
}
