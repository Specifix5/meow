use poise::{ serenity_prelude::{ CreateEmbed, EmbedField, User }, CreateReply };

use crate::{
  convert_to_timestamp,
  core::{
    constants::{ URL_CDN_GUILD_MEMBER, URL_USER_ID },
    nyan::{
      embed::MeowEmbed,
      user::{
        fetch_raw_member_data,
        fetch_raw_user_data,
        get_user_badges_string,
        is_guild_install,
      },
    },
    utils::error_handler::send_cmd_error,
  },
  Context,
  Error,
};

pub async fn profile_command(
  ctx: Context<'_>,
  target: Option<User>,
  user_profile: Option<bool>,
  ephemeral: Option<bool>
) -> Result<(), Error> {
  let guild_id = ctx.guild_id();
  let is_guild = guild_id.is_some();
  let use_guild_profile = !user_profile.unwrap_or(false) && is_guild_install(&ctx);

  let target_user = target.as_ref().unwrap_or_else(|| ctx.author());

  if is_guild && use_guild_profile {
    if let Some(guild_id) = guild_id {
      // Fetch member data
      let (raw_json, member_opt) = fetch_raw_member_data(
        ctx.http(),
        &target_user.id,
        &guild_id
      ).await;

      if let Some(member) = member_opt {
        // Build embed fields
        let fields: [EmbedField; 4] = [
          EmbedField::new("Display Name", member.display_name(), true),
          EmbedField::new(
            "Account Created",
            convert_to_timestamp!(member.user.created_at().unix_timestamp()),
            true
          ),
          EmbedField::new(
            "Joined at",
            member.joined_at
              .map(|time| convert_to_timestamp!(time.unix_timestamp()))
              .unwrap_or_else(|| "Unknown".to_string()),
            true
          ),
          EmbedField::new(
            "Roles",
            member.roles
              .iter()
              .map(|r| format!("<@&{}>", r.get()))
              .collect::<Vec<String>>()
              .join(", "),
            true
          ),
        ];

        // Get clan tag if available
        let clan_tag = raw_json
          .as_ref()
          .and_then(|json| json.get("user"))
          .and_then(|json| json.get("clan"))
          .and_then(|clan| clan.get("tag"))
          .and_then(|tag| tag.as_str())
          .map(|tag| format!("`` {} ``", tag))
          .unwrap_or_default();

        // Build embed
        let mut profile_embed = MeowEmbed::new()
          .title(format!("@{} ({})", member.user.tag(), member.user.id))
          .url(format!("{}{}", URL_USER_ID, member.user.id))
          .description(
            format!(
              "{} {}",
              get_user_badges_string(&member.user).await.unwrap_or_default(),
              clan_tag
            )
          )
          .fields(fields.to_vec())
          .thumbnail(
            member
              .avatar_url()
              .or_else(|| member.user.avatar_url())
              .unwrap_or_else(|| member.user.default_avatar_url())
          );

        // Add banner if available
        if
          let Some(banner_hash) = raw_json
            .as_ref()
            .and_then(|json| json.get("banner"))
            .and_then(|banner| banner.as_str())
        {
          profile_embed = profile_embed.image(
            format!(
              "{}{}/users/{}/banners/{}.{}?size=4096",
              URL_CDN_GUILD_MEMBER,
              guild_id,
              member.user.id,
              banner_hash,
              if banner_hash.contains("a_") {
                "gif"
              } else {
                "webp"
              }
            )
          );
        } else if let Some(banner_url) = member.user.banner_url() {
          profile_embed = profile_embed.image(banner_url);
        }

        ctx.send(
          CreateReply::default()
            .embed(CreateEmbed::from(profile_embed))
            .ephemeral(ephemeral.unwrap_or(false))
        ).await?;
      } else {
        send_cmd_error(ctx, "User not found".to_string()).await;
      }
    }
  } else {
    // Fetch user data
    let (raw_json, user_opt) = fetch_raw_user_data(ctx.http(), &target_user.id).await;

    if let Some(user) = user_opt {
      let fields: [EmbedField; 2] = [
        EmbedField::new("Display Name", user.display_name(), true),
        EmbedField::new(
          "Account Created",
          convert_to_timestamp!(user.created_at().unix_timestamp()),
          true
        ),
      ];

      // Get clan tag if available
      let clan_tag = raw_json
        .as_ref()
        .and_then(|json| json.get("clan"))
        .and_then(|clan| clan.get("tag"))
        .and_then(|tag| tag.as_str())
        .map(|tag| format!("`` {} ``", tag))
        .unwrap_or_default();

      let mut profile_embed = MeowEmbed::new()
        .title(format!("@{} ({})", user.tag(), user.id))
        .url(format!("{}{}", URL_USER_ID, user.id))
        .description(
          format!("{} {}", get_user_badges_string(&user).await.unwrap_or_default(), clan_tag)
        )
        .fields(fields.to_vec())
        .thumbnail(user.avatar_url().unwrap_or_else(|| user.default_avatar_url()));

      // Add banner if available
      if let Some(banner_url) = user.banner_url() {
        profile_embed = profile_embed.image(banner_url);
      }

      ctx.send(
        CreateReply::default()
          .embed(CreateEmbed::from(profile_embed))
          .ephemeral(ephemeral.unwrap_or(false))
      ).await?;
    } else {
      send_cmd_error(ctx, "User not found".to_string()).await;
    }
  }

  Ok(())
}

/// Gets a user's profile like avatar, banner, etc.
#[poise::command(
  slash_command,
  prefix_command,
  category = "General",
  install_context = "Guild|User",
  interaction_context = "Guild|PrivateChannel"
)]
pub async fn profile(
  ctx: Context<'_>,
  #[description = "Target to get profile"] target: Option<User>,
  #[description = "Gets the user profile instead of guild (even if available)"] user_profile: Option<bool>,
  #[description = "Incognito mode"] ephemeral: Option<bool>
) -> Result<(), Error> {
  profile_command(ctx, target, user_profile, ephemeral).await
}

#[poise::command(context_menu_command = "View Profile", install_context = "Guild|User")]
pub async fn user_profile(ctx: Context<'_>, target: User) -> Result<(), Error> {
  profile_command(ctx, Some(target), None, Some(true)).await
}
