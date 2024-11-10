import { EphmeralCommandOption } from "../../utils/constants";
import { ShoukoClient } from "../../utils/shouko/client";
import { Command, ShoukoInteraction } from "../../utils/shouko/command";

const PingCommand: Command = {
  name: "ping",
  description: "meow",
  options: [EphmeralCommandOption],
  run: async (_client: ShoukoClient, interaction: ShoukoInteraction) => {
    await interaction.reply({ content: "meow", ephemeral: interaction.getOption<boolean>("ephmeral") || false });
  },
};

export default [PingCommand];
