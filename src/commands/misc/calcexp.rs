use poise::CreateReply;
use std::io::Cursor;
use murmur3::murmur3_32;

use crate::{ core::utils::error_handler::send_cmd_error, logger, Context, Error };

/// Calculate Discord Experiment Rollouts with Murmurhash3
#[poise::command(
  slash_command,
  prefix_command,
  category = "Misc",
  install_context = "Guild|User",
  interaction_context = "Guild|PrivateChannel"
)]
pub async fn calcexp(
  ctx: Context<'_>,
  #[description = "<exp_name>:<resource_id>"] source: String,
  #[description = "Incognito mode"] ephemeral: Option<bool>
) -> Result<(), Error> {
  let calculated_hash = murmur3_32(&mut Cursor::new(source), 0);

  match calculated_hash {
    Ok(hash) => {
      ctx.send(
        CreateReply::default()
          .content(format!("Result is: **{}**", hash % 10_000))
          .ephemeral(ephemeral.unwrap_or(false))
      ).await?;
    }
    Err(e) => {
      send_cmd_error(ctx, "Something went wrong while calculating the hash".to_owned()).await;
      logger!(format!("Calcexp error: {}", e));
    }
  }

  Ok(())
}
