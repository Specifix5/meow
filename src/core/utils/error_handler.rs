use poise::serenity_prelude::{ self, Message };
use poise::{ serenity_prelude::CreateEmbed, CreateReply };
use rand::prelude::SliceRandom;
use crate::core::constants::{ Emojis, Messages, DEFAULT_EMBED_COLOR };

use crate::{ random_message, Data, Error, Context };

pub async fn send_cmd_error(ctx: Context<'_>, error: String) {
  let response = format!("{} {}", Emojis::ICON_DENY, error);
  if
    let Err(e) = ctx.send(
      CreateReply::default().content(response).ephemeral(true).reply(true)
    ).await
  {
    println!("Error while sending error message: {}", e);
  }
}

pub async fn serenity_send_cmd_error(
  ctx: &serenity_prelude::Context,
  message: &Message,
  error: String
) {
  let response = format!("{} {}", Emojis::ICON_DENY, error);
  if let Err(e) = message.reply(ctx, response).await {
    println!("Error while sending error message: {}", e);
  }
}

pub(crate) async fn error_handler(error: poise::FrameworkError<'_, Data, Error>) {
  match error {
    poise::FrameworkError::Setup { error, .. } => panic!("Failed to start bot: {:?}", error),
    poise::FrameworkError::CommandPanic { payload, ctx, .. } => {
      let embed = CreateEmbed::new()
        .color(DEFAULT_EMBED_COLOR)
        .title("unexpected error occured")
        .description(format!("\n```diff\n- {}\n```", payload.unwrap()));

      let message = format!("{} {}", Emojis::ICON_DENY, random_message!(Messages::ERROR_GENERIC));

      if
        let Err(e) = ctx.send(
          CreateReply::default().embed(embed).content(message).ephemeral(true).reply(true)
        ).await
      {
        println!("Error while sending error message: {}", e);
      }
    }
    poise::FrameworkError::UnknownCommand { framework, msg, msg_content, prefix, .. } => {
      let response = format!(
        "command '{}' not found, type `{}help` to view all commands",
        msg_content,
        prefix
      );
      serenity_send_cmd_error(framework.serenity_context, msg, response).await
    }

    poise::FrameworkError::ArgumentParse { error, input, ctx, .. } => {
      let embed = CreateEmbed::new()
        .color(DEFAULT_EMBED_COLOR)
        .title("argument error occurred")
        .description(
          format!("\n```diff\n- {}{}\n```", error.to_string(), if input.is_some() {
            format!("\n= argument: {}", input.unwrap_or("None".to_string()).to_string())
          } else {
            String::new()
          })
        );

      let message = format!("{} {}", Emojis::ICON_DENY, random_message!(Messages::ERROR_GENERIC));

      if
        let Err(e) = ctx.send(
          CreateReply::default().embed(embed).content(message).ephemeral(true).reply(true)
        ).await
      {
        println!("Error while sending error message: {}", e);
      }
    }
    error => {
      println!("Other error: {:?}", error.to_string());
    }
  }
}
