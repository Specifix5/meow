use crate::{ Data, Error };
use crate::commands::*;
use poise::Command;

macro_rules! command_list {
  ($($cmd:expr),* $(,)?) => {
      vec![$($cmd),*]
  };
}

pub fn get_commands() -> Vec<Command<Data, Error>> {
  command_list![
    help::help(),

    profile::profile(),
    profile::user_profile(),

    avatar::avatar(),
    avatar::user_avatar(),
    avatar::user_guild_avatar(),

    banner::banner(),
    banner::user_banner(),
    banner::user_guild_banner(),

    guild::guild()
  ]
}
