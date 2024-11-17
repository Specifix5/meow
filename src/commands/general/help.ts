import {
  APIEmbedField,
  ApplicationCommandOptionData,
  ApplicationCommandOptionType,
  AutocompleteInteraction,
} from "discord.js";
import {
  EMOJI,
  EphmeralCommandOption,
  NAME,
  PREFIX,
  UniversalContextType,
  UniversalIntegrationType,
} from "../../utils/constants.js";
import { TranslateApplicationCommandOptionType } from "../../utils/types.js";
import { Command, ShoukoInteraction } from "../../utils/shouko/command.js";
import { ShoukoClient } from "../../utils/shouko/client.js";
import { ShoukoEmbed } from "../../utils/shouko/embed.js";
import { CreateErrorMessage } from "../../utils/logging.js";

const sortByOptions = (commands: Command[]) => {
  return commands.sort((a: Command, b: Command) => {
    const opt_a = a.options ? a.options.length : 0;
    const opt_b = b.options ? b.options.length : 0;
    return opt_b - opt_a;
  });
};

const HelpCommand: Command = {
  name: "help",
  description: "Display all commands in a neat list",
  contexts: UniversalContextType,
  integrationTypes: UniversalIntegrationType,
  options: [
    {
      name: "name",
      autocomplete: true,
      description: "The command to show",
      type: ApplicationCommandOptionType.String,
    },
    EphmeralCommandOption,
  ],
  run: async (client: ShoukoClient, interaction: ShoukoInteraction) => {
    const helpEmbed = new ShoukoEmbed()
      .setTitle(`${NAME} — Help Page`)
      .setDescription(
        `${EMOJI.ICON_SLASH} **Prefix:** \`\`${PREFIX}\`\`\n` +
          `${EMOJI.ICON_HAMBURGER} **Categories:** ` +
          client.commandCategories.map((c) => `\`\`${c}\`\``).join(" "),
      );
    const selectedCommand = interaction.getOption<string>("name");
    if (!selectedCommand) {
      client.commandCategories.map((category) => {
        const commandList: string[] = [];
        sortByOptions(client.getCommands().filter((c) => c.category === category)).map(
          (command: Command) => {
            let commandString = `</${command.name}:${command.id}>`;
            let noSubcommand = false;
            if ((command.options && command.options.length <= 0) || !command.options)
              return commandString;

            command.options?.map((option: ApplicationCommandOptionData) => {
              if (option.name === "ephmeral") return;
              if (option.type === ApplicationCommandOptionType.Subcommand) {
                commandString = `</${command.name} ${option.name}:${command.id}> ${
                  option.options
                    ?.map((opt2: ApplicationCommandOptionData) => {
                      return `\`(${opt2.name})\``;
                    })
                    .join(" ") ?? ""
                }`;
                commandList.push(commandString);
              } else {
                noSubcommand = true;
                commandString += ` \`(${option.name})\``;
              }
            });
            if (noSubcommand) commandList.push(commandString);
          },
        );
        helpEmbed.addFields([
          {
            name: EMOJI.ARROW_RIGHT + " " + category + " Commands",
            value: commandList.map((cl) => `${EMOJI.ARROW_BRANCH} ${cl}`).join("\n"),
          },
        ]);
      });
    } else {
      const command = client.getCommands().find((c) => c.name === selectedCommand);
      if (!command) {
        return interaction.reply({
          content: CreateErrorMessage(
            new Error(`No matching command found by the name '${selectedCommand}'`),
          ),
          ephemeral: true,
        });
      }
      const subCommands =
        command.options?.filter((opt) => opt.type === ApplicationCommandOptionType.Subcommand) ??
        [];
      let usageString = `${PREFIX}${command.name} ${command.options?.map((opt) => `(${opt.name}:${TranslateApplicationCommandOptionType[opt.type - 1]})`).join(" ")}`;

      if (subCommands.length > 0)
        usageString = `${subCommands
          .map((sub) => {
            return `${PREFIX}${command.name} ${sub.name} ${sub.options?.map((opt) => `(${opt.name}:${TranslateApplicationCommandOptionType[opt.type - 1]})`).join(" ")}`;
          })
          .join("\n")}`;

      helpEmbed
        .setTitle(`Showing command '${command.name}'`)
        .setDescription(
          `${EMOJI.ICON_SLASH} **Usage:**
          \`\`\`${usageString}\`\`\`
          ${EMOJI.ICON_INFO} **Description:** ${command.description}
          ${EMOJI.ICON_HAMBURGER} **Category:** ${command.category}`,
        )
        .addFields(
          command.options?.map((opt) => {
            return {
              name: `${EMOJI.ARROW_RIGHT} **${opt.name}:** \`${TranslateApplicationCommandOptionType[opt.type - 1]}\``,
              value: `${EMOJI.ARROW_BRANCH} *${opt.description}*`,
            };
          }) as APIEmbedField[],
        );
    }

    await interaction.reply({
      content: client.getString("COMMANDRESPONSE_HELP"),
      embeds: [helpEmbed],
      ephemeral: interaction.getOption<boolean>("ephmeral") ?? false,
    });
  },
  autocomplete: async (client: ShoukoClient, interaction: AutocompleteInteraction) => {
    const focusedOption = interaction.options.getFocused(true);
    let choices: string[] = [];

    switch (focusedOption.name) {
      case "name":
        choices = client.getCommands().map((c) => c.name);
        break;
    }

    const filtered = choices.filter((choice) => choice.startsWith(focusedOption.value));
    await interaction.respond(filtered.map((choice) => ({ name: choice, value: choice })));
  },
};

export default [HelpCommand];
