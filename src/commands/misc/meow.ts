import { ApplicationCommandOptionType } from "discord.js";
import { ShoukoClient } from "../../utils/shouko/client.js";
import { Command, ShoukoInteraction } from "../../utils/shouko/command.js";

const MeowCommand: Command = {
  name: "meow",
  description: "meow",
  options: [
    {
      name: "get",
      type: ApplicationCommandOptionType.Subcommand,
      description: "THE MEOW",
      options: [
        {
          name: "test",
          description: "the yes",
          type: ApplicationCommandOptionType.Boolean,
        },
      ],
    },
    {
      name: "set",
      type: ApplicationCommandOptionType.Subcommand,
      description: "THE MEOW",
      options: [
        {
          name: "test",
          description: "the yes",
          type: ApplicationCommandOptionType.Boolean,
        },
      ],
    },
    {
      name: "del",
      type: ApplicationCommandOptionType.Subcommand,
      description: "THE MEOW",
      options: [
        {
          name: "testega",
          description: "the yes",
          type: ApplicationCommandOptionType.Boolean,
        },
        {
          name: "twaaaaa",
          description: "the yes",
          type: ApplicationCommandOptionType.Boolean,
        },
      ],
    },
  ],
  run: async (_client: ShoukoClient, interaction: ShoukoInteraction) => {
    await interaction.reply({
      content: `Subcommand used: ${interaction.getOption<boolean>("get")}`,
    });
  },
};

export default MeowCommand;
