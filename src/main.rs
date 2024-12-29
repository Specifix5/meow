use command_list::get_commands;
use poise::serenity_prelude as serenity;
use core::utils::{ error_handler::error_handler, event_handler::Handler };
use std::borrow::Cow;
use tokio::signal::ctrl_c;

pub struct Data {}
pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;

pub mod commands;

pub mod events;

pub mod core;

pub mod command_list;

#[tokio::main]
async fn main() -> Result<(), Error> {
  dotenv::dotenv().ok();
  let token = std::env::var("CLIENT_TOKEN").expect("missing CLIENT_TOKEN");
  let intents =
    serenity::GatewayIntents::non_privileged() | serenity::GatewayIntents::MESSAGE_CONTENT;

  let framework: poise::Framework<Data, Error> = poise::Framework
    ::builder()
    .options(poise::FrameworkOptions {
      commands: get_commands(),
      prefix_options: poise::PrefixFrameworkOptions {
        prefix: Some(Cow::from(core::constants::COMMAND_PREFIX.to_string())),
        case_insensitive_commands: true,
        ..Default::default()
      },
      on_error: |error: poise::FrameworkError<'_, Data, Error>| Box::pin(error_handler(error)),
      ..Default::default()
    })
    .setup(|ctx, _ready, framework| {
      Box::pin(async move {
        poise::builtins::register_globally(ctx, &framework.options().commands).await?;
        Ok(Data {})
      })
    })
    .build();

  let client: Result<serenity::Client, serenity::Error> = serenity::ClientBuilder
    ::new(token, intents)
    .event_handler(Handler)
    .framework(framework).await;

  let client_task = tokio::spawn(async move {
    if let Err(why) = client.unwrap().start().await {
      println!("Client error: {:?}", why);
    }
  });

  ctrl_c().await?;
  logger!("Received SIGINT Signal, Shutting down..");

  client_task.abort();
  Ok(())
}
