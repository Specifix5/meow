use crate::{ Data, Error };
use crate::commands::*;
use poise::Command;

pub fn get_commands() -> Vec<Command<Data, Error>> {
  vec![
    // General Category
    general::help::help(),

    general::profile::profile(),
    general::profile::user_profile(),

    general::avatar::avatar(),
    general::avatar::user_avatar(),
    general::avatar::user_guild_avatar(),

    general::banner::banner(),
    general::banner::user_banner(),
    general::banner::user_guild_banner(),

    general::guild::guild(),

    general::ping::ping(),

    fun::interact::interact()
  ]
}
