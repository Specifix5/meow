use poise::{ serenity_prelude::{ CreateEmbed, User }, CreateReply };

use crate::{
  core::nyan::{ embed::{ add_download_links, MeowEmbed }, user::is_guild_install },
  Context,
  Error,
};

pub async fn avatar_command(
  ctx: Context<'_>,
  target: Option<User>,
  guild_avatar: Option<bool>,
  ephemeral: Option<bool>
) -> Result<(), Error> {
  let target_user = target.as_ref().unwrap_or_else(|| ctx.author());
  let avatar_link;

  if let Some(guild_id) = ctx.guild_id() {
    if guild_avatar.unwrap_or(false) && is_guild_install(&ctx) {
      if let Ok(member) = guild_id.member(ctx.serenity_context(), target_user.id).await {
        avatar_link = member
          .avatar_url()
          .unwrap_or_else(|| {
            target_user.avatar_url().unwrap_or_else(|| target_user.default_avatar_url())
          });
      } else {
        avatar_link = target_user.avatar_url().unwrap_or_else(|| target_user.default_avatar_url());
      }
    } else {
      avatar_link = target_user.avatar_url().unwrap_or_else(|| target_user.default_avatar_url());
    }
  } else {
    avatar_link = target_user.avatar_url().unwrap_or_else(|| target_user.default_avatar_url());
  }

  let mut avatar_embed = MeowEmbed::new()
    .title(format!("{}'s avatar", target_user.tag()))
    .image(&avatar_link);

  avatar_embed = add_download_links(avatar_embed, &avatar_link);

  ctx.send(
    CreateReply::default()
      .embed(CreateEmbed::from(avatar_embed))
      .ephemeral(ephemeral.unwrap_or(false))
  ).await?;

  Ok(())
}

/// Gets a user's avatar.
#[poise::command(
  slash_command,
  prefix_command,
  category = "General",
  install_context = "Guild|User",
  interaction_context = "Guild|PrivateChannel"
)]
pub async fn avatar(
  ctx: Context<'_>,
  #[description = "Target to get avatar"] target: Option<User>,
  #[description = "Get guild avatar"] guild_avatar: Option<bool>,
  #[description = "Incognito mode"] ephemeral: Option<bool>
) -> Result<(), Error> {
  avatar_command(ctx, target, guild_avatar, ephemeral).await
}

#[poise::command(
  context_menu_command = "View Avatar",
  category = "General",
  install_context = "Guild|User",
  interaction_context = "Guild|PrivateChannel"
)]
pub async fn user_avatar(ctx: Context<'_>, target: User) -> Result<(), Error> {
  avatar_command(ctx, Some(target), None, Some(true)).await
}

#[poise::command(
  context_menu_command = "View Guild Avatar",
  category = "General",
  install_context = "Guild|User",
  interaction_context = "Guild|PrivateChannel"
)]
pub async fn user_guild_avatar(ctx: Context<'_>, target: User) -> Result<(), Error> {
  avatar_command(ctx, Some(target), Some(true), Some(true)).await
}
