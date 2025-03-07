import {
  APIEmbedField,
  ApplicationCommandOptionData,
  ApplicationCommandOptionType,
  AutocompleteInteraction,
} from "discord.js";
import {
  COMMAND_PADDED_BOX_LENGTH,
  EMOJI,
  EphmeralCommandOption,
  NAME,
  PREFIX,
  UniversalContextType,
  UniversalIntegrationType,
} from "../../utils/constants.js";
import { TranslateApplicationCommandOptionType } from "../../utils/types.js";
import { Command, MeowInteraction } from "../../utils/nyan/command.js";
import { MeowClient } from "../../utils/nyan/client.js";
import { MeowEmbed } from "../../utils/nyan/embed.js";
import { CreateErrorMessage } from "../../utils/logging.js";

const sortByOptions = (commands: Command[]) => {
  return commands.sort((a: Command, b: Command) => {
    const opt_a = a.options ? a.options.length : 0;
    const opt_b = b.options ? b.options.length : 0;
    return opt_b - opt_a;
  });
};

const paddedBox = (text: string): string => {
  return `\` ${text.padEnd(COMMAND_PADDED_BOX_LENGTH, " ")} \``;
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
  run: async (client: MeowClient, interaction: MeowInteraction) => {
    const helpEmbed = new MeowEmbed()
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
            if ((command.options && command.options.length < 1) || !command.options)
              return commandList.push(commandString);

            command.options?.map((option: ApplicationCommandOptionData) => {
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
                if (option.name === "ephmeral") return;
                commandString += ` \`(${option.name})\``;
              }
            });
            if (noSubcommand) commandList.push(commandString);
          },
        );
        helpEmbed.addFields([
          {
            name: EMOJI.ARROW_RIGHT + " " + category + " Commands",
            value: commandList
              .map(
                (cl, index) =>
                  `${index < commandList.length - 1 ? EMOJI.ARROW_BRANCH : EMOJI.ARROW_BRANCH_END} ${cl}`,
              )
              .join("\n"),
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

      helpEmbed.setTitle(`Showing command '${command.name}'`).setDescription(
        `${EMOJI.ICON_SLASH} **Usage:**
          \`\`\`${usageString}\`\`\`
          ${EMOJI.ICON_INFO} **Description:** ${command.description}
          ${EMOJI.ICON_HAMBURGER} **Category:** ${command.category}`,
      );
      const fields: APIEmbedField[] =
        command.options?.map((opt) => {
          const optionField = {
            name: `${EMOJI.ARROW_RIGHT} **\`${opt.name}\`:** ${opt.description}`,
            value: `${
              opt.type === ApplicationCommandOptionType.Subcommand &&
              opt.options &&
              opt.options.length > 0
                ? `${opt.options.map((opt2, index) => `${index < opt.options!.length - 1 ? EMOJI.ARROW_BRANCH : EMOJI.ARROW_BRANCH_END} ${paddedBox(opt2.name)} ${opt2.description}`).join("\n")}`
                : " "
            }`,
          };

          return optionField;
        }) ?? [];

      if (fields.length > 0) helpEmbed.setFields(fields);
    }

    await interaction.reply({
      content: client.getString("COMMAND_RESPONSE_HELP"),
      embeds: [helpEmbed],
      ephemeral: interaction.getOption<boolean>("ephmeral") ?? false,
    });
  },
  autocomplete: async (client: MeowClient, interaction: AutocompleteInteraction) => {
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
