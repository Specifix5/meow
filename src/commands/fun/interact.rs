use poise::{
  serenity_prelude::{
    CreateActionRow,
    CreateButton,
    CreateEmbed,
    CreateEmbedFooter,
    CreateInteractionResponse,
    CreateInteractionResponseMessage,
    Mentionable,
    User,
  },
  CreateReply,
};

use crate::{
  core::{ constants, nyan::{ api, embed::MeowEmbed }, utils::error_handler::send_cmd_error },
  Context,
  Error,
};
use sqlx::Row;

#[derive(Debug, poise::ChoiceParameter, Clone)]
pub enum Action {
  Hug,
  Kiss,
  Slap,
  Bite,
  Cuddle,
  Pat,
}

impl Action {
  pub fn to_string(&self) -> String {
    format!("{:#?}", self).to_lowercase()
  }
  pub fn to_past_string(&self) -> String {
    match self {
      Action::Hug => { "hugged".to_owned() }
      Action::Kiss => { "kissed".to_owned() }
      Action::Slap => { "slapped".to_owned() }
      Action::Bite => { "bit".to_owned() }
      Action::Cuddle => { "cuddled".to_owned() }
      Action::Pat => { "petted".to_owned() }
    }
  }
  pub fn to_plural(&self) -> String {
    match self {
      Action::Kiss => { format!("{}es", self.to_string()) }
      _ => { format!("{}s", self.to_string()) }
    }
  }
}

pub async fn get_embed_interact_command(
  ctx: Context<'_>,
  action: Action,
  author: &User,
  target: User
) -> Result<Option<MeowEmbed>, Error> {
  let l_action = action.to_string();
  let action_cloned = action.clone();
  let action_string = action.to_past_string();
  match api::get(action).await {
    Ok(res) => {
      let action_took = &ctx.data().db.perform_transaction::<_, i32>(|mut transaction|
        Box::pin(async move {
          let target_id_string = target.id.to_string();
          sqlx
            ::query("INSERT OR IGNORE INTO interaction (user_id, hug) VALUES (?, 0)")
            .bind(&target_id_string)
            .execute(&mut *transaction).await?;

          sqlx
            ::query(
              format!(
                "UPDATE interaction SET {} = {} + 1 WHERE user_id = ?",
                &l_action,
                &l_action
              ).as_str()
            )
            .bind(&target_id_string)
            .execute(&mut *transaction).await?;

          let interaction_count_row = sqlx
            ::query(format!("SELECT {} FROM interaction WHERE user_id = ?", &l_action).as_str())
            .bind(&target_id_string)
            .fetch_one(&mut *transaction).await?;

          let interaction_count: i32 = interaction_count_row.get(&l_action.as_str());
          transaction.commit().await?;
          Ok(interaction_count)
        })
      ).await?;

      let description: [String; 2] = [
        format!("### {} **{}** {}", &author.mention(), action_string, target.mention()),
        format!(
          "{} **{}** now has ``{}`` {}",
          constants::Emojis::ARROW_BRANCH_END,
          target.name,
          action_took,
          action_cloned.to_plural()
        ),
      ];
      let interact_embed = MeowEmbed::new()
        .description(description.join("\n"))
        .image(res.url)
        .footer(
          CreateEmbedFooter::new(
            format!("{} | {} v{}", res.anime_name, constants::APP_NAME, constants::VERSION)
          )
        );

      Ok(Some(interact_embed))
    }

    Err(err) => { Err(err) }
  }
}

pub async fn interact_command(
  ctx: Context<'_>,
  action: Action,
  target: User,
  ephemeral: Option<bool>
) -> Result<(), Error> {
  if ephemeral.unwrap_or(false) {
    ctx.defer_ephemeral().await?;
  } else {
    ctx.defer().await?;
  }

  match get_embed_interact_command(ctx, action.clone(), ctx.author(), target.clone()).await {
    Ok(interact_embed) => {
      match interact_embed {
        Some(interact_embed) => {
          let msg = ctx
            .send(
              CreateReply::default()
                .embed(CreateEmbed::from(interact_embed))
                .ephemeral(ephemeral.unwrap_or(false))
                .components(
                  vec![
                    CreateActionRow::Buttons(
                      vec![
                        CreateButton::new("response_interaction_back")
                          .style(poise::serenity_prelude::ButtonStyle::Primary)
                          .label(format!("{} back", &action.to_string()))
                      ]
                    )
                  ]
                )
            ).await?
            .into_message().await?;

          match
            msg
              .await_component_interaction(&ctx.serenity_context().shard)
              .author_id(target.id)
              .timeout(std::time::Duration::from_secs(10)).await
          {
            Some(interaction) => {
              match get_embed_interact_command(ctx, action, &target, ctx.author().clone()).await {
                Ok(interact_embed) => {
                  match interact_embed {
                    Some(interact_embed) => {
                      interaction.create_response(
                        &ctx,
                        CreateInteractionResponse::Message(
                          CreateInteractionResponseMessage::default()
                            .embed(CreateEmbed::from(interact_embed))
                            .ephemeral(ephemeral.unwrap_or(false))
                        )
                      ).await?;
                    }
                    None => {}
                  }
                }
                Err(err) => {
                  send_cmd_error(ctx, err.to_string()).await;
                }
              }
            }
            None => {}
          }

          Ok(())
        }

        None => { Ok(()) }
      }
    }
    Err(err) => {
      send_cmd_error(ctx, err.to_string()).await;
      Ok(())
    }
  }
}

/// Do interactions like hug, etc!
#[poise::command(
  slash_command,
  prefix_command,
  category = "Fun",
  install_context = "Guild|User",
  interaction_context = "Guild|PrivateChannel"
)]
pub async fn interact(
  ctx: Context<'_>,
  #[description = "The action to perform"] action: Action,
  #[description = "The target user"] target: User,
  #[description = "Incognito mode"] ephemeral: Option<bool>
) -> Result<(), Error> {
  interact_command(ctx, action, target, ephemeral).await?;
  Ok(())
}
