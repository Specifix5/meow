use std::collections::HashSet;

use crate::{
  core::{
    constants::{ URL_MEOWBOT_BANNER, VERSION },
    utils::{ error_handler::send_cmd_error, ranime::get_random_anime },
  },
  random_message,
  Context,
  Error,
};
use poise::{ serenity_prelude::{ CreateEmbed, CreateEmbedFooter, EmbedField }, CreateReply };
use crate::core::nyan::embed::MeowEmbed;
use crate::core::constants::{ Emojis, Messages, APP_NAME, COMMAND_PREFIX };
use rand::prelude::SliceRandom;

/// Displays the list of available commands
#[poise::command(
  slash_command,
  prefix_command,
  category = "General",
  install_context = "Guild|User",
  interaction_context = "Guild|PrivateChannel"
)]
pub async fn help(
  ctx: Context<'_>,
  #[description = "Command name to show help page"] command: Option<String>,
  #[description = "Incognito mode"] ephemeral: Option<bool>
) -> Result<(), Error> {
  // Handle specific command help
  if let Some(cmd) = command {
    let cmd_name = cmd.to_lowercase();
    let command = ctx
      .framework()
      .options()
      .commands.iter()
      .find(|c| c.name == cmd_name);

    if let Some(command) = command {
      // Build and send the help embed for the specific command
      let mut help_embed = MeowEmbed::new().title(format!("Showing help for '{}'", command.name));
      let description: [String; 3] = [
        format!(
          "{} **Usage:** \n``` {} ```",
          Emojis::ICON_SLASH,
          format!(
            "{}{} {}",
            ctx.prefix(),
            command.name,
            command.parameters
              .iter()
              .map(|p| (
                if p.required {
                  format!("{}:required", p.name.to_string())
                } else {
                  p.name.to_string()
                }
              ))
              .collect::<Vec<String>>()
              .join(" ")
          )
        ),
        format!(
          "{} **Description:** {}",
          Emojis::ICON_INFO,
          command.description
            .as_ref()
            .unwrap_or(&Cow::from(Messages::COMMAND_NODESCRIPTION))
            .to_string()
        ),
        format!(
          "{} **Category:** {}",
          Emojis::ICON_HAMBURGER,
          command.category.as_ref().unwrap_or(&Cow::from(Messages::COMMAND_NOCATEGORY)).to_string()
        ),
      ];

      help_embed = help_embed.description(description.join("\n"));

      help_embed = help_embed.fields_raw(
        command.parameters
          .iter()
          .map(|p| {
            EmbedField::new(
              format!(
                "{} **``{}``: {}**",
                Emojis::ARROW_RIGHT,
                p.name,
                p.description
                  .as_ref()
                  .unwrap_or(&Cow::from(Messages::COMMAND_NODESCRIPTION))
                  .to_string()
              ),
              " ".to_string(),
              false
            )
          })
          .collect::<Vec<EmbedField>>()
      );
      ctx.send(
        CreateReply::default()
          .embed(CreateEmbed::from(help_embed))
          .ephemeral(ephemeral.unwrap_or(false))
      ).await?;
    } else {
      // Command not found error
      send_cmd_error(ctx, format!("Command '{}' not found", cmd)).await;
    }
    return Ok(());
  }

  // Generate a list of all commands and categories
  let commands = ctx
    .framework()
    .options()
    .commands.iter()
    .filter(|c| c.context_menu_action.is_none());

  let command_name_list: Vec<String> = commands
    .clone()
    .map(|c| {
      format!(
        "`{}: {}`",
        c.name,
        c.description.as_ref().unwrap_or(&Cow::from(Messages::COMMAND_NODESCRIPTION)).to_string()
      )
    })
    .collect();

  let category_name_list: Vec<String> = commands
    .clone()
    .map(|c| {
      c.category.as_ref().unwrap_or(&Cow::from(Messages::COMMAND_NOCATEGORY)).to_string()
    })
    .collect::<HashSet<String>>()
    .into_iter()
    .collect();

  let description: [String; 3] = [
    format!("{} **Prefix:** `{}`", Emojis::ICON_SLASH, COMMAND_PREFIX),
    format!("{} **Categories:** `{}`", Emojis::ICON_HAMBURGER, category_name_list.join(", ")),
    format!("{} **Commands:** `{}` total", Emojis::ICON_OPTIONS, command_name_list.len()),
  ];

  let mut fields: Vec<EmbedField> = category_name_list
    .iter()
    .map(|category| {
      let commands_in_category: Vec<String> = commands
        .clone()
        .filter(|c| {
          c.category.as_ref().unwrap_or(&Cow::from(Messages::COMMAND_NOCATEGORY)) == category
        })
        .map(|c| format!("`{}`", c.name))
        .collect();

      EmbedField::new(
        format!("**{} {} Commands**", Emojis::ARROW_RIGHT, category),
        commands_in_category.join(", "),
        true
      )
    })
    .collect();

  let mut help_embed = MeowEmbed::new()
    .title(format!("{} (v{}) — Help Page", APP_NAME, VERSION))
    .image(URL_MEOWBOT_BANNER)
    .description(&description.join("\n"))
    .footer(CreateEmbedFooter::new(""));

  if let Some((src, url)) = get_random_anime() {
    help_embed = help_embed.thumbnail(url);
    fields.push(
      EmbedField::new(
        format!("{} Others", Emojis::ARROW_RIGHT),
        format!("-# * [img source]({})", src),
        false
      )
    );
  }

  help_embed = help_embed.fields_raw(fields);

  let response_message = random_message!(Messages::COMMAND_HELP_REPLY);
  ctx.send(
    CreateReply::default()
      .embed(CreateEmbed::from(help_embed))
      .content(response_message.to_string())
      .ephemeral(ephemeral.unwrap_or(false))
  ).await?;

  Ok(())
}
