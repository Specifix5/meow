use poise::serenity_prelude as serenity;

use serenity::{ CreateEmbed, CreateEmbedFooter, EmbedField };

use crate::core::constants::{ APP_NAME, DEFAULT_EMBED_COLOR, VERSION };
pub struct MeowEmbed(CreateEmbed);

impl MeowEmbed {
  pub fn new() -> Self {
    let mut embed = CreateEmbed::default();
    embed = embed
      .color(DEFAULT_EMBED_COLOR) // Your default color
      .footer(CreateEmbedFooter::new(format!("{} v{}", APP_NAME, VERSION)));

    MeowEmbed(embed)
  }

  // Delegate methods to inner CreateEmbed
  pub fn title(mut self, title: impl ToString) -> Self {
    self.0 = self.0.title(title.to_string());
    self
  }

  pub fn color(mut self, color: serenity::Colour) -> Self {
    self.0 = self.0.color(color);
    self
  }

  pub fn description(mut self, desc: impl ToString) -> Self {
    self.0 = self.0.description(desc.to_string());
    self
  }

  pub fn footer(mut self, footer: CreateEmbedFooter) -> Self {
    self.0 = self.0.footer(footer);
    self
  }

  pub fn fields(mut self, fields: Vec<EmbedField>) -> Self {
    let mut result_fields = Vec::with_capacity(fields.len() * 2);

    for (i, field) in fields.into_iter().enumerate() {
      result_fields.push(field);
      if (i + 1) % 2 == 0 {
        result_fields.push(EmbedField::new("\u{200B}", "\u{200B}", true));
      }
    }

    self.0 = self.0.fields(
      result_fields
        .iter()
        .map(|f| (f.name.to_string(), f.value.to_string(), f.inline as bool))
        .collect::<Vec<(String, String, bool)>>()
    );
    self
  }

  pub fn fields_raw(mut self, fields: Vec<EmbedField>) -> Self {
    self.0 = self.0.fields(
      fields
        .iter()
        .map(|f| (f.name.to_string(), f.value.to_string(), f.inline as bool))
        .collect::<Vec<(String, String, bool)>>()
    );
    self
  }

  pub fn url(mut self, url: impl ToString) -> Self {
    self.0 = self.0.url(url.to_string());
    self
  }

  pub fn thumbnail(mut self, url: impl ToString) -> Self {
    self.0 = self.0.thumbnail(url.to_string());
    self
  }

  pub fn image(mut self, url: impl ToString) -> Self {
    self.0 = self.0.image(url.to_string());
    self
  }
}

impl From<MeowEmbed> for CreateEmbed {
  fn from(embed: MeowEmbed) -> Self {
    embed.0
  }
}

pub fn get_download_links(link: &str) -> String {
  let trimmed = link
    .strip_suffix("?size=1024")
    .unwrap_or(link.strip_suffix("?size=4096").unwrap_or(link));

  let base_link = trimmed
    .strip_suffix(".webp")
    .unwrap_or(trimmed.strip_suffix(".gif").unwrap_or(trimmed));

  let description = if base_link.contains("a_") {
    format!(
      "[GIF]({}{}) | [WebP]({}{}) | [PNG]({}{}) | [JPEG]({}{})",
      base_link,
      ".gif?size=4096",
      base_link,
      ".webp?size=4096",
      base_link,
      ".png?size=4096",
      base_link,
      ".jpeg?size=4096"
    )
  } else {
    format!(
      "[WebP]({}{}) | [PNG]({}{}) | [JPEG]({}{})",
      base_link,
      ".webp?size=4096",
      base_link,
      ".png?size=4096",
      base_link,
      ".jpeg?size=4096"
    )
  };

  description
}

pub fn add_download_links(embed: MeowEmbed, link: &str) -> MeowEmbed {
  embed.description(get_download_links(link))
}
