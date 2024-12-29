use poise::{ serenity_prelude::{ json, CreateEmbed, Http, User }, CreateReply };

use crate::{
  core::{
    constants::URL_CDN_GUILD_MEMBER,
    nyan::{
      embed::{ add_download_links, MeowEmbed },
      user::{ fetch_raw_member_data, is_guild_install },
    },
    utils::error_handler::send_cmd_error,
  },
  Context,
  Error,
};

pub async fn banner_command(
  ctx: Context<'_>,
  target: Option<User>,
  guild_banner: Option<bool>,
  ephemeral: Option<bool>
) -> Result<(), Error> {
  let target_user = target.as_ref().unwrap_or_else(|| ctx.author());
  let mut banner_embed = MeowEmbed::new().title(format!("{}'s banner", target_user.tag()));
  let mut banner_link = String::new();

  if let Some(guild_id) = ctx.guild_id() {
    if guild_banner.unwrap_or(false) && is_guild_install(&ctx) {
      if let Ok(member) = guild_id.member(ctx.serenity_context(), target_user.id).await {
        let raw_json = fetch_raw_member_data(ctx.http(), &target_user.id, &guild_id).await.0;

        if
          let Some(banner_hash) = raw_json
            .as_ref()
            .and_then(|raw| raw.get("banner"))
            .and_then(|banner| banner.as_str())
        {
          banner_link = format!(
            "{}{}/users/{}/banners/{}.{}?size=1024",
            URL_CDN_GUILD_MEMBER,
            guild_id,
            member.user.id,
            banner_hash,
            if banner_hash.contains("a_") {
              "gif"
            } else {
              "webp"
            }
          );
        } else if let Some(raw_user_json) = raw_json.as_ref().and_then(|raw| raw.get("user")) {
          if let Ok(user) = json::from_value::<User>(raw_user_json.clone()) {
            banner_link = user.banner_url().unwrap_or_default();
          }
        }
      } else {
        banner_link = fetch_user_banner(ctx.http(), target_user).await.unwrap_or_default();
      }
    } else {
      banner_link = fetch_user_banner(ctx.http(), target_user).await.unwrap_or_default();
    }
  } else {
    banner_link = fetch_user_banner(ctx.http(), target_user).await.unwrap_or_default();
  }

  if !banner_link.is_empty() {
    banner_embed = add_download_links(banner_embed.image(&banner_link), &banner_link);

    ctx.send(
      CreateReply::new()
        .embed(CreateEmbed::from(banner_embed))
        .ephemeral(ephemeral.unwrap_or(false))
    ).await?;
  } else {
    send_cmd_error(ctx, "Specified user/member has no banner!".to_string()).await;
  }

  Ok(())
}

async fn fetch_user_banner(http: &Http, user: &User) -> Option<String> {
  if let Ok(user) = http.get_user(user.id).await { user.banner_url() } else { None }
}

/// Gets a user's banner.
#[poise::command(
  slash_command,
  prefix_command,
  category = "General",
  install_context = "Guild|User",
  interaction_context = "Guild|PrivateChannel"
)]
pub async fn banner(
  ctx: Context<'_>,
  #[description = "Target to get banner"] target: Option<User>,
  #[description = "Get guild banner"] guild_banner: Option<bool>,
  #[description = "Incognito mode"] ephemeral: Option<bool>
) -> Result<(), Error> {
  banner_command(ctx, target, guild_banner, ephemeral).await
}

#[poise::command(
  context_menu_command = "View banner",
  category = "General",
  install_context = "Guild|User",
  interaction_context = "Guild|PrivateChannel"
)]
pub async fn user_banner(ctx: Context<'_>, target: User) -> Result<(), Error> {
  banner_command(ctx, Some(target), None, Some(true)).await
}

#[poise::command(
  context_menu_command = "View Guild banner",
  category = "General",
  install_context = "Guild|User",
  interaction_context = "Guild|PrivateChannel"
)]
pub async fn user_guild_banner(ctx: Context<'_>, target: User) -> Result<(), Error> {
  banner_command(ctx, Some(target), Some(true), Some(true)).await
}
