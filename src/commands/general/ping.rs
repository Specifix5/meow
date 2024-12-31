use poise::{ serenity_prelude::CreateEmbed, CreateReply };
use rand::prelude::SliceRandom;
use tokio::time::Instant;

use crate::{
  core::{
    constants::{ Emojis, Messages, APP_NAME },
    nyan::embed::MeowEmbed,
    utils::system_info::{ get_app_uptime_string, get_memory_usage },
  },
  random_message,
  Context,
  Error,
};

/// Gets the bot's current status
#[poise::command(
  slash_command,
  prefix_command,
  category = "General",
  install_context = "Guild|User",
  interaction_context = "Guild|PrivateChannel"
)]
pub async fn ping(
  ctx: Context<'_>,
  #[description = "Incognito mode"] ephemeral: Option<bool>
) -> Result<(), Error> {
  let response_message = random_message!(Messages::COMMAND_PING_REPLY);
  let ping_start_time = Instant::now();
  let reply_handle = ctx.send(
    CreateReply::default().content("Pinging..").ephemeral(ephemeral.unwrap_or(false))
  ).await?;

  let ping_end_time = ping_start_time.elapsed().as_millis();

  ctx.cache().guilds().len();

  let description: [String; 4] = [
    format!("**{} API/Heartbeat:** `{} ms`", Emojis::ICON_HEART_BEAT, ctx.ping().await.as_millis()),
    format!("**{} Client Latency:** `{} ms`", Emojis::ICON_PING_PONG, ping_end_time),
    format!(
      "**{} Memory Usage:** `{} MB`",
      Emojis::ICON_IMAGE_FILE_WITH_PADDED_BOX,
      get_memory_usage()
    ),
    format!("**{} Uptime:** `{}`", Emojis::ICON_UPTIME, get_app_uptime_string()),
  ];
  let ping_embed = MeowEmbed::new()
    .title(format!("{} — Status Page", APP_NAME))
    .description(description.join("\n"));

  reply_handle.edit(
    ctx,
    CreateReply::default()
      .content(response_message.to_string())
      .embed(CreateEmbed::from(ping_embed))
  ).await?;

  Ok(())
}
