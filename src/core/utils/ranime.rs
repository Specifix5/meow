use std::fs::File;
use std::error::Error;
use poise::serenity_prelude::json::from_reader;
use serde::Deserialize;
use lazy_static::lazy_static;
use std::sync::RwLock;

#[derive(Debug, Deserialize, Clone)]
pub struct Anime {
  pub src: String,
  pub url: String,
}

pub struct AnimeStore {
  pub animes: Vec<Anime>,
  current_index: usize,
}

impl AnimeStore {
  pub fn new(file_path: &str) -> Result<Self, Box<dyn Error>> {
    let file = File::open(file_path)?;
    let animes: Vec<Anime> = from_reader(file)?;
    Ok(AnimeStore { animes, current_index: 0 })
  }

  pub fn get_random_anime(&mut self) -> Option<(String, String)> {
    if self.animes.is_empty() {
      return None; // Return None if there are no animes
    }

    let anime = &self.animes[self.current_index];
    self.current_index = (self.current_index + 1) % self.animes.len(); // Increment and wrap around
    Some((anime.src.clone(), anime.url.clone()))
  }
}

lazy_static! {
  static ref ANIME_STORE: RwLock<AnimeStore> = {
    let store = AnimeStore::new("random_anime.json").expect("Failed to load anime images");
    RwLock::new(store)
  };
}

/// Function to access random anime from the global store
///
/// The global store is a RwLock that is lazily initialized with
/// the contents of the "random_anime.json" file. This file should
/// contain an array of objects with "src" and "url" attributes.
///
/// The function returns a tuple of the form (src, url) where src
/// is the source of the anime image and url is the URL of the
/// image. If the store is empty or there is an error reading the
/// store, the function returns None.
pub fn get_random_anime() -> Option<(String, String)> {
  let mut store = ANIME_STORE.write().unwrap();
  store.get_random_anime()
}
