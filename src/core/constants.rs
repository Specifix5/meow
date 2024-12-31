use poise::serenity_prelude::{ model::colour, Colour, UserPublicFlags };

pub const DEFAULT_EMBED_COLOR: Colour = colour::Color::new(0x121212);

pub const URL_USER_ID: &str = "https://discord.com/users/";
pub const URL_CDN_GUILD_MEMBER: &str = "https://cdn.discordapp.com/guilds/";
pub const URL_CDN_USER: &str = "https://cdn.discordapp.com/guilds/";

pub const URL_CDN_GUILD_ICON: &str = "https://cdn.discordapp.com/icons/";
pub const URL_CDN_GUILD_SPLASH: &str = "https://cdn.discordapp.com/splashes/";

pub const URL_MEOWBOT_BANNER: &str =
  "https://cdn.discordapp.com/attachments/1306245076300861450/1322888760836493343/meowbot_banner.png";

pub const APP_NAME: &str = env!("CARGO_PKG_NAME");

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

pub const REPOSITORY: &str = env!("CARGO_PKG_REPOSITORY");

pub const COMMAND_PREFIX: &str = "m!";

pub struct Messages;
pub struct Emojis;

pub struct Badges;

impl Messages {
  pub const ERROR_GENERIC: [&str; 5] = [
    "uh oh! something went wrong! sorry! :c",
    "oops! i seem to have stumbled upon a little problem! :T",
    "whoops! a tiny error has occurred! >.<",
    "something went awry! sorry about that! =w=",
    "a tiny hiccup! please forgive me! TwT",
  ];
  pub const COMMAND_NODESCRIPTION: &str = "No Description";
  pub const COMMAND_NOCATEGORY: &str = "No Category";
  pub const COMMAND_HELP_REPLY: [&str; 3] = [
    "i'm here to assist! :3",
    "here you go! hope this helps! ^w^",
    "confused? don't worry, i'm here to guide you! c:",
  ];
}

impl Emojis {
  pub const ARROW_RIGHT: &str = "<:_:1305532650836070420>";
  pub const ARROW_LEFT: &str = "<:_:1305533338701922324>";
  pub const ARROW_BRANCH_END: &str = " ╰";
  pub const ARROW_BRANCH: &str = "┠";
  pub const ICON_HAMBURGER: &str = "<:_:1306990190899757149>";
  pub const ICON_SLASH: &str = "<:_:1306989298444402698>";
  pub const ICON_DENY: &str = "<:_:1306994045872115712>";
  pub const ICON_ALLOW: &str = "<:_:1306994631094833286>";
  pub const ICON_INFO: &str = "<:_:1307647568184082522>";
  pub const ICON_EMPTY: &str = "ㅤ";
  pub const ICON_OPTIONS: &str = "<:_:1307669903221854299>";
  pub const ICON_USER: &str = "<:_:1321411020996218931>";
  pub const ICON_GREEN_CIRCLE: &str = "<:_:1321415740410302464>";
  pub const ICON_BOOST_LEVEL_1: &str = "<:_:1322530540212981812>";
  pub const ICON_BOOST_LEVEL_2: &str = "<:_:1322530588078637089>";
  pub const ICON_BOOST_LEVEL_3: &str = "<:_:1322530608735588422>";
  pub const ICON_IMAGE_FILE: &str = "<:_:1322898336507887677>";
  pub const ICON_IMAGE_FILE_WITH_PADDED_BOX: &str = "<:_:1322898380246089772>";
  pub const ICON_GITHUB: &str = "<:_:1322898821432475740>";
}

impl Badges {
  pub const DEV_ACTIVE: &str =
    "[<:_:1316610999331065876>](<https://support-dev.discord.com/hc/en-us/articles/10113997751447-Active-Developer-Badge>)";
  pub const HYPESQUAD_BRILLIANCE: &str =
    "[<:_:1317756045468241941>](<https://discord.com/settings/hypesquad-online>)";
  pub const HYPESQUAD_BRAVERY: &str =
    "[<:_:1317756763411451945>](<https://discord.com/settings/hypesquad-online>)";
  pub const HYPESQUAD_BALANCED: &str =
    "[<:_:1317756591662960761>](<https://discord.com/settings/hypesquad-online>)";
  pub const HYPESQUAD_EVENTS: &str =
    "[<:_:1317756875944624158>](<https://discord.com/settings/hypesquad-online>)";
  pub const DISCORD_STAFF: &str =
    "[<:_:1317757343160733716>](<https://support.discord.com/hc/en-us/articles/360035962891-Profile-Badges-101>)";
  pub const BUGHUNTER_LEVEL_1: &str =
    "[<:_:1317757510035439656>](<https://support.discord.com/hc/en-us/articles/360046057772-Discord-Bugs>)";
  pub const BUGHUNTER_LEVEL_2: &str =
    "[<:_:1317757564779368458>](<https://support.discord.com/hc/en-us/articles/360046057772-Discord-Bugs>)";
  pub const PREMIUM_EARLY_SUPPORTER: &str =
    "[<:_:1317757817821593681>](<https://support.discord.com/hc/en-us/articles/360017949691-Legacy-Nitro-Classic-FAQ>)";
  pub const VERIFIED_BOT: &str =
    "[<:_:1317764374424981554><:_:1317764400131735613>](<https://discord.com/developers/docs/intro>)";
  pub const BOT: &str =
    "[<:_:1317764441680642068><:_:1317764459137204294>](<https://discord.com/developers/docs/intro>)";
  pub const VERIFIED_BOT_DEVELOPER: &str =
    "[<:_:1317762099459526746>](<https://support.discord.com/hc/en-us/articles/360035962891-Profile-Badges-101>)";
  pub const MODERATOR_ALUMNI: &str =
    "[<:_:1317762276463214652>](<https://support.discord.com/hc/en-us/articles/360035962891-Profile-Badges-101>)";
  pub const NITRO: &str = "[<:_:1317766795737960500>](<https://discord.com/nitro>)";
  pub const MIKU: &str =
    " [<:_:1321113479016480879>](<https://discord.com/users/485749822322769920>) ";
}

#[derive(Debug, Clone)]
pub enum UserBadge {
  Staff = UserPublicFlags::DISCORD_EMPLOYEE.bits() as isize, // Discord Employee
  Partner = UserPublicFlags::PARTNERED_SERVER_OWNER.bits() as isize, // Partnered Server Owner
  HypeSquad = UserPublicFlags::HYPESQUAD_EVENTS.bits() as isize, // HypeSquad Events Member
  BugHunterLevel1 = UserPublicFlags::BUG_HUNTER_LEVEL_1.bits() as isize, // Bug Hunter Level 1
  HypeSquadOnlineHouse1 = UserPublicFlags::HOUSE_BRAVERY.bits() as isize, // House Bravery Member
  HypeSquadOnlineHouse2 = UserPublicFlags::HOUSE_BRILLIANCE.bits() as isize, // House Brilliance Member
  HypeSquadOnlineHouse3 = UserPublicFlags::HOUSE_BALANCE.bits() as isize, // House Balance Member
  PremiumEarlySupporter = UserPublicFlags::EARLY_SUPPORTER.bits() as isize, // Early Nitro Supporter
  TeamPseudoUser = UserPublicFlags::TEAM_USER.bits() as isize, // User is a team
  BugHunterLevel2 = UserPublicFlags::BUG_HUNTER_LEVEL_2.bits() as isize, // Bug Hunter Level 2
  VerifiedBot = UserPublicFlags::VERIFIED_BOT.bits() as isize, // Verified Bot
  VerifiedDeveloper = UserPublicFlags::EARLY_VERIFIED_BOT_DEVELOPER.bits() as isize, // Early Verified Bot Developer
  CertifiedModerator = UserPublicFlags::DISCORD_CERTIFIED_MODERATOR.bits() as isize, // Moderator Programs Alumni
  BotHttpInteractions = UserPublicFlags::BOT_HTTP_INTERACTIONS.bits() as isize, // Bot uses only HTTP interactions and is shown in the online member list
  ActiveDeveloper = UserPublicFlags::ACTIVE_DEVELOPER.bits() as isize, // User is an Active Developer

  // Anything past this point is our own implemented badges
  Placeholder = 1 << 24, // Placeholder, must be ignored, only to make sure a vector is not empty.
  Bot = 1 << 25, // User is a bot
  Nitro = 1 << 26, // User has Nitro
  Miku = 1 << 27, // User is specifix :3
}

impl ToString for UserBadge {
  fn to_string(&self) -> String {
    match self {
      UserBadge::Staff => "Discord Staff".to_string(),
      UserBadge::Partner => "Partnered Server Owner".to_string(),
      UserBadge::HypeSquad => "HypeSquad Events Member".to_string(),
      UserBadge::HypeSquadOnlineHouse1 => "House Bravery Member".to_string(),
      UserBadge::HypeSquadOnlineHouse2 => "House Brilliance Member".to_string(),
      UserBadge::HypeSquadOnlineHouse3 => "House Balance Member".to_string(),
      UserBadge::BugHunterLevel1 => "Bug Hunter Level 1".to_string(),
      UserBadge::BugHunterLevel2 => "Bug Hunter Level 2".to_string(),
      UserBadge::PremiumEarlySupporter => "Early Nitro Supporter".to_string(),
      UserBadge::VerifiedBot => "Verified Bot".to_string(),
      UserBadge::VerifiedDeveloper => "Early Verified Bot Developer".to_string(),
      UserBadge::CertifiedModerator => "Moderator Programs Alumni".to_string(),
      UserBadge::BotHttpInteractions => "HTTP Interactions".to_string(),
      UserBadge::ActiveDeveloper => "Active Developer".to_string(),
      UserBadge::TeamPseudoUser => "Team".to_string(),
      UserBadge::Bot => "Bot".to_string(),
      UserBadge::Nitro => "Nitro".to_string(),
      UserBadge::Placeholder => "".to_string(),
      UserBadge::Miku => "ミク".to_string(),
    }
  }
}

impl PartialEq for UserBadge {
  fn eq(&self, other: &Self) -> bool {
    self.to_string() == other.to_string()
  }
}

impl UserBadge {
  pub fn to_string_emoji(&self) -> String {
    match self {
      UserBadge::ActiveDeveloper => Badges::DEV_ACTIVE.to_string(),
      UserBadge::HypeSquad => Badges::HYPESQUAD_EVENTS.to_string(),
      UserBadge::HypeSquadOnlineHouse1 => Badges::HYPESQUAD_BRAVERY.to_string(),
      UserBadge::HypeSquadOnlineHouse2 => Badges::HYPESQUAD_BRILLIANCE.to_string(),
      UserBadge::HypeSquadOnlineHouse3 => Badges::HYPESQUAD_BALANCED.to_string(),
      UserBadge::PremiumEarlySupporter => Badges::PREMIUM_EARLY_SUPPORTER.to_string(),
      UserBadge::VerifiedBot => Badges::VERIFIED_BOT.to_string(),
      UserBadge::VerifiedDeveloper => Badges::VERIFIED_BOT_DEVELOPER.to_string(),
      UserBadge::CertifiedModerator => Badges::MODERATOR_ALUMNI.to_string(),
      UserBadge::BugHunterLevel1 => Badges::BUGHUNTER_LEVEL_1.to_string(),
      UserBadge::BugHunterLevel2 => Badges::BUGHUNTER_LEVEL_2.to_string(),
      UserBadge::Bot => Badges::BOT.to_string(),
      UserBadge::Nitro => Badges::NITRO.to_string(),
      UserBadge::Miku => Badges::MIKU.to_string(),
      _ => "".to_string(),
    }
  }
}

#[macro_export]
macro_rules! random_message {
  ($category:expr, $with_prefix:literal) => {
    {
      let msg = $category.choose(&mut rand::thread_rng()).unwrap();
      format!("{} {}", $with_prefix, msg)
    }
  };
  ($category:expr) => {
      $category.choose(&mut rand::thread_rng()).unwrap()
  };
}

#[macro_export]
macro_rules! convert_to_timestamp {
  ($timestamp:expr, $type:literal) => {
    if type == "relative" {
      format!("<t:{}:R>", $timestamp)
    }
  };
  ($timestamp:expr) => {
    format!("<t:{}>", $timestamp)
  };
}
