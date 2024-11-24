import {
  EphmeralCommandOption,
  UniversalContextType,
  UniversalIntegrationType,
} from "../../utils/constants.js";
import { MeowClient } from "../../utils/nyan/client.js";
import { Command, MeowInteraction } from "../../utils/nyan/command.js";

const PingCommand: Command = {
  name: "ping",
  description: "meow",
  contexts: UniversalContextType,
  integrationTypes: UniversalIntegrationType,
  options: [EphmeralCommandOption],
  run: async (client: MeowClient, interaction: MeowInteraction) => {
    await interaction.reply({
      content:
        "meow!!! " +
        client
          .getCommands()
          .map((cmd) => `\`${cmd.name}: ${cmd.category}\``)
          .join(", "),
      ephemeral: interaction.getOption<boolean>("ephmeral") ?? false,
    });
  },
};

export default [PingCommand];
