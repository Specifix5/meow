use poise::{ serenity_prelude::{ CreateEmbed, EmbedField, GuildId, ImageHash }, CreateReply };

use crate::{
  convert_to_timestamp,
  core::{
    constants::{ Emojis, URL_CDN_GUILD_ICON, URL_CDN_GUILD_SPLASH },
    nyan::{ embed::{ get_download_links, MeowEmbed }, user::is_guild_install },
    utils::error_handler::send_cmd_error,
  },
  Context,
  Error,
};

pub async fn guild_command(
  ctx: Context<'_>,
  guild_id: GuildId,
  guild_name: String,
  approximate_member_count: u64,
  approximate_presence_count: u64,
  premium_subscription_string: Option<String>,
  icon: Option<ImageHash>,
  splash: Option<ImageHash>,
  banner: Option<String>,
  description: Option<String>,
  ephemeral: Option<bool>
) -> Result<(), Error> {
  let mut fields = vec![
    EmbedField::new("Guild Id", guild_id.to_string(), true),
    EmbedField::new(
      "Guild Created",
      convert_to_timestamp!(guild_id.created_at().unix_timestamp()),
      true
    )
  ];

  let mut guild_embed = MeowEmbed::new()
    .title(guild_name)
    .description(
      format!(
        "**{} {} members • {} {} online {}**{}",
        Emojis::ICON_USER,
        approximate_member_count,
        Emojis::ICON_GREEN_CIRCLE,
        approximate_presence_count,
        premium_subscription_string.unwrap_or_default(),
        match description {
          Some(description) => format!("\n-# *{}*", description),
          None => String::new(),
        }
      )
    );

  if let Some(icon) = icon {
    let icon_url = format!(
      "{}{}/{}.{}?size=4096",
      URL_CDN_GUILD_ICON,
      guild_id,
      icon.to_string(),
      if icon.is_animated() {
        "gif"
      } else {
        "webp"
      }
    );
    guild_embed = guild_embed.thumbnail(&icon_url);
    fields.push(EmbedField::new("Guild Icon", get_download_links(&icon_url), true));
  }

  if let Some(splash) = splash {
    let splash_url = format!(
      "{}{}/{}.{}?size=4096",
      URL_CDN_GUILD_SPLASH,
      guild_id,
      splash.to_string(),
      if splash.is_animated() {
        "gif"
      } else {
        "webp"
      }
    );
    guild_embed = guild_embed.image(&splash_url);
    fields.push(EmbedField::new("Guild Splash", get_download_links(&splash_url), true));
  }

  if let Some(banner) = banner {
    guild_embed = guild_embed.image(&banner);
    fields.push(EmbedField::new("Guild Banner", get_download_links(&banner), true));
  }

  guild_embed = guild_embed.fields(fields);

  ctx.send(
    CreateReply::new().embed(CreateEmbed::from(guild_embed)).ephemeral(ephemeral.unwrap_or(false))
  ).await?;

  Ok(())
}

/// Displays information about the current guild
#[poise::command(
  slash_command,
  prefix_command,
  category = "General",
  install_context = "Guild",
  interaction_context = "Guild"
)]
pub async fn guild(
  ctx: Context<'_>,
  #[description = "Target guild id"] guild_id: Option<String>,
  #[description = "Incognito mode"] ephemeral: Option<bool>
) -> Result<(), Error> {
  if let Some(guild_id) = guild_id {
    let guild_id = match guild_id.parse::<u64>() {
      Ok(id) if id != 0 => id,
      _ => {
        send_cmd_error(ctx, "Invalid guild id".to_owned()).await;
        return Ok(());
      }
    };

    let guild_result = ctx.http().get_guild_preview(GuildId::from(guild_id)).await;

    match guild_result {
      Ok(guild_preview) => {
        guild_command(
          ctx,
          guild_preview.id,
          guild_preview.name,
          guild_preview.approximate_member_count,
          guild_preview.approximate_presence_count,
          None,
          guild_preview.icon,
          guild_preview.splash,
          None,
          guild_preview.description,
          ephemeral
        ).await?;
      }
      Err(_) => {
        send_cmd_error(
          ctx,
          "Failed to get guild, make sure guild is a discoverable server!".to_owned()
        ).await;
      }
    }
  } else if let Some(_guild_id) = ctx.guild_id() {
    if is_guild_install(&ctx) {
      let guild_result = ctx.http().get_guild_with_counts(_guild_id).await;

      match guild_result {
        Ok(partial_guild) => {
          let banner = partial_guild.banner_url();
          guild_command(
            ctx,
            partial_guild.id,
            partial_guild.name,
            partial_guild.approximate_member_count.unwrap_or_default(),
            partial_guild.approximate_presence_count.unwrap_or_default(),
            if partial_guild.premium_subscription_count.unwrap_or_default() > 0 {
              let guild_tier = u8::from(partial_guild.premium_tier);
              let emoji = match guild_tier {
                1 => Emojis::ICON_BOOST_LEVEL_1,
                2 => Emojis::ICON_BOOST_LEVEL_2,
                _ => Emojis::ICON_BOOST_LEVEL_3,
              };

              Some(
                format!(
                  "• {} {} boost(s) (Lvl. {})",
                  emoji,
                  partial_guild.premium_subscription_count.unwrap_or_default(),
                  guild_tier
                )
              )
            } else {
              Some("".to_owned())
            },
            partial_guild.icon,
            partial_guild.splash,
            banner,
            partial_guild.description,
            ephemeral
          ).await?;
        }
        Err(_) => {
          send_cmd_error(ctx, "Failed to get guild".to_owned()).await;
        }
      }
    } else {
      send_cmd_error(ctx, "Must be a guild install to use this command".to_owned()).await;
    }
  } else {
    send_cmd_error(ctx, "Must be in a guild to use this command".to_owned()).await;
  }

  Ok(())
}
