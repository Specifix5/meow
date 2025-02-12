use std::sync::RwLock;

use crate::{ commands::fun::interact::Action, core::constants::{ self, URL_BASE_ACTION }, Error };
use lazy_static::lazy_static;
use reqwest::{ self, Client };
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct InteractAPIResponse {
  pub url: String,
  pub anime_name: String,
}

#[derive(Deserialize, Debug)]
struct NekosBestAPIResponse {
  results: Vec<InteractAPIResponse>,
}

lazy_static! {
  static ref reqwest_client: RwLock<Client> = RwLock::new(
    Client::builder()
      .cookie_store(true)
      .user_agent(format!("{}/{}", constants::APP_NAME, constants::VERSION))
      .build()
      .unwrap()
  );
}

pub async fn get(action: Action) -> Result<InteractAPIResponse, Error> {
  let req = reqwest_client
    .read()
    .unwrap()
    .get(format!("{}{}", URL_BASE_ACTION, action.to_string()))
    .send();
  let res = req.await?.json::<NekosBestAPIResponse>().await?;
  Ok(InteractAPIResponse {
    url: res.results
      .iter()
      .map(|result| result.url.to_owned())
      .collect::<String>(),
    anime_name: res.results
      .iter()
      .map(|result| result.anime_name.to_owned())
      .collect::<String>(),
  })
}
