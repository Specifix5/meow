use poise::serenity_prelude as serenity;
use serenity::{ async_trait, Context, EventHandler, Ready };

use crate::events::ready;

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
  async fn ready(&self, ctx: Context, ready: Ready) {
    ready::ready(ctx, ready).await;
  }
}
