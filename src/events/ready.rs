use poise::serenity_prelude as serenity;
use serenity::{ Context, Ready, OnlineStatus };

use crate::logger;

pub async fn ready(ctx: Context, ready: Ready) {
  ctx.set_presence(None, OnlineStatus::Idle);
  logger!(format!("Logged in as {} press CTRL + C to shut down!", ready.user.tag()));
}
