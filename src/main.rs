use command_list::get_commands;
use poise::serenity_prelude as serenity;
use sqlx::{ Executor, SqlitePool };
use core::{
  nyan::database::{ self, Database },
  utils::{
    error_handler::error_handler,
    event_handler::Handler,
    system_info::{ get_app_uptime, refresh_system_info },
  },
};
use serenity::cache::Settings;
use std::{ borrow::Cow, time::Duration };
use tokio::signal::ctrl_c;

pub struct Data {
  db: Database,
}
pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;

pub mod commands;

pub mod events;

pub mod core;

pub mod command_list;

#[tokio::main]
async fn main() -> Result<(), Error> {
  logger!(format!("Starting meowbot - v{}", env!("CARGO_PKG_VERSION")));
  dotenv::dotenv().ok();
  let token = std::env::var("CLIENT_TOKEN").expect("missing CLIENT_TOKEN");
  let intents =
    serenity::GatewayIntents::non_privileged() | serenity::GatewayIntents::MESSAGE_CONTENT;

  let mut settings = Settings::default();
  settings.time_to_live = Duration::from_secs(60);
  settings.cache_channels = false;

  // Database Initialization
  let db_pool = SqlitePool::connect("sqlite://database.db").await.expect(
    "Failed to connect to database"
  );
  let db = database::Database { pool: db_pool };
  let schema = include_str!("./core/sql/schema.sql");

  db.perform_transaction(|mut transaction| {
    Box::pin(async move {
      transaction.execute(schema).await?;
      transaction.commit().await?;
      logger!("Database initialized");
      Ok(())
    })
  }).await?;

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
        Ok(Data {
          db,
        })
      })
    })
    .build();

  let client: Result<serenity::Client, serenity::Error> = serenity::ClientBuilder
    ::new(token, intents)
    .event_handler(Handler)
    .cache_settings(settings)
    .framework(framework).await;

  let system_info_task = tokio::spawn(async {
    get_app_uptime();
    refresh_system_info().await;
  });

  let client_task = tokio::spawn(async move {
    if let Err(why) = client.unwrap().start().await {
      println!("Client error: {:?}", why);
    }
  });

  ctrl_c().await?;
  logger!("Received SIGINT Signal, Shutting down..");

  client_task.abort();
  system_info_task.abort();
  Ok(())
}
