use crate::{Context, Error};
use poise::{serenity_prelude::{Color, CreateEmbed}, CreateReply};
use reqwest;
use serde_json;


/// Get a random fluffy fox!
#[poise::command(prefix_command, slash_command, category = "Randomizer")]
pub async fn fox(ctx: Context<'_>) -> Result<(), Error> {
    let url = "https://randomfox.ca/floof";
    let img = fetch_animal_img(url, "image").await?;

    ctx.send(CreateReply::default()
        .embed(CreateEmbed::new()
            .title("Here's your fox!")
            .color(Color::RED)
            .image(img)
        )
    ).await?;

    Ok(())
}


async fn fetch_animal_img(url: &str, field_name: &str) -> Result<String, Error> {
    let img: String = reqwest::get(url)
        .await?
        .json::<serde_json::Value>()
        .await?
        .get(field_name)
        .and_then(|value| value.as_str())
        .ok_or("Field not found or not a string")?
        .to_string();

    Ok(img)
}