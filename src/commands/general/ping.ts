import {
  EphmeralCommandOption,
  UniversalContextType,
  UniversalIntegrationType,
} from "../../utils/constants.js";
import { ShoukoClient } from "../../utils/shouko/client.js";
import { Command, ShoukoInteraction } from "../../utils/shouko/command.js";

const PingCommand: Command = {
  name: "ping",
  description: "meow",
  contexts: UniversalContextType,
  integrationTypes: UniversalIntegrationType,
  options: [EphmeralCommandOption],
  run: async (_client: ShoukoClient, interaction: ShoukoInteraction) => {
    await interaction.reply({
      content: "meow",
      ephemeral: interaction.getOption<boolean>("ephmeral") ?? false,
    });
  },
};

export default [PingCommand];
