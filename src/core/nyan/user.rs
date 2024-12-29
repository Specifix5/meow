use poise::serenity_prelude::{
  json,
  AuthorizingIntegrationOwner,
  GuildId,
  Http,
  LightMethod,
  Member,
  Request,
  Route,
  User,
  UserId,
};

use crate::{ core::constants::UserBadge, logger, Context, Error };

/// Gets a list of all badges that a user has from their public flags.
///
/// The returned list will be empty if the user has no public flags set.
///
/// The badges that are checked are:
///
/// - Staff
/// - Partner
/// - HypeSquad
/// - HypeSquadOnlineHouse1
/// - HypeSquadOnlineHouse2
/// - HypeSquadOnlineHouse3
/// - PremiumEarlySupporter
/// - BugHunterLevel1
/// - BugHunterLevel2
/// - ActiveDeveloper
/// - VerifiedBot
/// - VerifiedDeveloper
/// - CertifiedModerator
/// - BotHttpInteractions
pub fn get_user_flags(user: &User) -> Vec<UserBadge> {
  let mut badges: Vec<UserBadge> = Vec::new();
  for badge in [
    UserBadge::Staff,
    UserBadge::Partner,
    UserBadge::HypeSquad,
    UserBadge::HypeSquadOnlineHouse1,
    UserBadge::HypeSquadOnlineHouse2,
    UserBadge::HypeSquadOnlineHouse3,
    UserBadge::PremiumEarlySupporter,
    UserBadge::BugHunterLevel1,
    UserBadge::BugHunterLevel2,
    UserBadge::ActiveDeveloper,
    UserBadge::VerifiedBot,
    UserBadge::VerifiedDeveloper,
    UserBadge::CertifiedModerator,
    UserBadge::BotHttpInteractions,
  ].iter() {
    if let Some(flags) = user.public_flags {
      let _badge = (*badge).clone() as u32;
      if (flags.bits() & _badge) != 0 {
        badges.push(badge.clone());
      }
    }
  }

  badges
}

/// Fetches the raw JSON data of a user and deserializes it into a `User` object.
///
/// This function will return `None` if the request fails or if the JSON fails to parse.
///
/// # Errors
///
/// If the request fails or if the JSON fails to parse, an error will be logged and `None` will be returned.
pub async fn fetch_raw_user_data(
  http: &Http,
  &user_id: &UserId
) -> (Option<json::Value>, Option<User>) {
  match
    http.request(
      Request::new(Route::User { user_id: UserId::from(user_id) }, LightMethod::Get)
    ).await
  {
    Ok(response) => {
      let raw_json: json::Value = json
        ::from_str(&response.text().await.unwrap())
        .expect("Failed to parse user json");

      // Deserialize the raw JSON into a `User` object
      let user: User = json::from_value(raw_json.clone()).expect("Failed to deserialize user");
      (Some(raw_json), Some(user))
    }
    Err(err) => {
      logger!(format!("Error fetching user data: {:?}", err));
      (None, None)
    }
  }
}

/// Checks if the current command invocation was triggered by a guild slash command install.
///
/// Returns `true` if the command was triggered by a guild slash command install, `false` otherwise.
///
/// If the context is not a slash command context, this function will return `true` because then it's
/// ran with prefix.
pub fn is_guild_install(ctx: &Context<'_>) -> bool {
  if let poise::Context::Application(app_ctx) = ctx {
    app_ctx.interaction.authorizing_integration_owners.0
      .iter()
      .any(|owner| matches!(owner, AuthorizingIntegrationOwner::GuildInstall(_)))
  } else {
    // Not a slash command context, return true bcs then it's ran with prefix
    true
  }
}

/// Fetches the raw JSON data of a guild member and deserializes it into a `Member` object.
///
/// This function will return `None` if the request fails or if the JSON fails to parse.
///
/// # Errors
///
/// If the request fails or if the JSON fails to parse, an error will be logged and `None` will be returned.
pub async fn fetch_raw_member_data(
  http: &Http,
  &user_id: &UserId,
  &guild_id: &GuildId
) -> (Option<json::Value>, Option<Member>) {
  match
    http.request(Request::new(Route::GuildMember { guild_id, user_id }, LightMethod::Get)).await
  {
    Ok(response) => {
      let raw_json: json::Value = json
        ::from_str(&response.text().await.unwrap())
        .expect("Failed to parse user json");

      // Deserialize the raw JSON into a `Member` object
      let mut member: Member = json
        ::from_value(raw_json.clone())
        .expect("Failed to deserialize user");
      member.guild_id = guild_id;

      (Some(raw_json), Some(member))
    }
    Err(err) => {
      logger!(format!("Error fetching user data: {:?}", err));
      (None, None)
    }
  }
}

/// Returns a string of all the badges a user has, formatted as a string of emojis.
///
/// # Errors
///
/// If the function fails to format the string, an error will be returned.
pub async fn get_user_badges_string(user: &User) -> Result<String, Error> {
  let mut initial_badges = get_user_flags(user);
  initial_badges.push(UserBadge::Placeholder); // Makes sure there's at least one badge so for loop runs...

  let mut badges = Vec::<UserBadge>::new();
  for b in initial_badges.iter() {
    badges.push((*b).clone());
  }

  if user.banner_url().is_some() && !user.bot {
    badges.push(UserBadge::Nitro);
  }

  if user.bot && !badges.contains(&UserBadge::VerifiedBot) {
    badges.push(UserBadge::Bot);
  }

  if user.id == 485749822322769920 {
    badges.push(UserBadge::Miku);
  }

  Ok(
    format!(
      "{}",
      badges
        .iter()
        .map(|b| b.to_string_emoji())
        .collect::<Vec<String>>()
        .join("")
    )
  )
}
