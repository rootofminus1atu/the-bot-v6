use std::io::Cursor;
use serde::Deserialize;
use crate::{Context, Error};
use image::{ImageBuffer, Rgba};
use poise::{serenity_prelude::{ComponentInteractionCollector, CreateActionRow, CreateAttachment, CreateButton, CreateEmbed, CreateEmbedAuthor, CreateEmbedFooter, CreateInteractionResponse, CreateInteractionResponseMessage, EditMessage}, CreateReply};



/// the width will be 5 times this value
const PIXEL_HEIGHT: usize = 100;

/// Get a random color palette!
#[poise::command(prefix_command, slash_command, category = "Randomizer")]
pub async fn palette(ctx: Context<'_>) -> Result<(), Error> {
    // generating an image
    let mut attachment = CreateAttachment::bytes(
        get_palette_bytes().await?, 
        "the_palette.png".to_string()
    );


    let ctx_id = ctx.id();
    let button_id = format!("{}button", ctx_id);


    // the componenst that will be reused
    let embed = CreateEmbed::default()
        .image("attachment://the_palette.png")
        .color(poise::serenity_prelude::Color::BLURPLE)
        .author(CreateEmbedAuthor::new("Here's your random color palette:")
            .icon_url("https://media.discordapp.net/attachments/1060711805028155453/1061825040716402731/logo_beter.png")
        )
        .footer(CreateEmbedFooter::new("Generated with colormind.io"));

    let button = CreateButton::new(&button_id).label("Generate again");

    let action_row = CreateActionRow::Buttons(vec![button.clone()]);

    let mut message = ctx.send(CreateReply::default()
        .embed(embed.clone())
        .components(vec![action_row.clone()])
        .attachment(attachment.clone())
    )
    .await?
    .into_message()
    .await?;



    while let Some(interaction) = ComponentInteractionCollector::new(ctx)
        .filter(move |interaction| interaction.data.custom_id.starts_with(&ctx_id.to_string()))
        .timeout(std::time::Duration::from_secs(60))
        .await
    {
        // generating another image
        attachment = CreateAttachment::bytes(
            get_palette_bytes().await?, 
            "the_palette.png".to_string()
        );


        interaction.create_response(
            ctx, 
            CreateInteractionResponse::UpdateMessage(
                CreateInteractionResponseMessage::new()
                    .add_file(attachment.clone())
                    .embed(embed.clone())
                    .components(vec![action_row.clone()])
            )
        ).await?;
    }


    message.edit(ctx, EditMessage::new()
        .components(vec![CreateActionRow::Buttons(vec![
            button.disabled(true)
        ])])
        .embed(embed)
    )
    .await?;


    Ok(())
}



async fn get_palette_bytes() -> Result<Vec<u8>, Error> {
    let res = reqwest::Client::new()
        .post("http://colormind.io/api/")
        .json(&serde_json::json!({"model": "default"}))
        .send()
        .await?
        .json::<ColorData>()
        .await?;

    let colors = res.result
        .iter()
        .map(|c| {
            Rgba([c.r, c.g, c.b, 255])
        })
        .collect::<Vec<_>>();

    let img_bytes = img_to_bytes(create_image(&colors))?;

    Ok(img_bytes)
}

fn img_to_bytes(img: ImageBuffer<Rgba<u8>, Vec<u8>>) -> Result<Vec<u8>, Error> {
    let mut img_cursor = Cursor::new(Vec::new());
    img.write_to(&mut img_cursor, image::ImageOutputFormat::Png)?;

    Ok(img_cursor.into_inner())
}

fn create_image(colors: &[Rgba<u8>]) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    let height = PIXEL_HEIGHT;
    let count = colors.len();
    let width = height * count;

    // create an ImageBuffer with the specified width and height
    let mut img = ImageBuffer::<Rgba<u8>, Vec<u8>>::new(width as u32, height as u32);

    // draw the squares with different colors
    for (i, color) in colors.iter().enumerate() {
        let x_start = i * (width / count);
        let x_end = (i + 1) * (width / count);

        for x in x_start..x_end {
            for y in 0..height {
                img.put_pixel(x as u32, y as u32, *color);
            }
        }
    }

    img
}

#[derive(Debug, Deserialize)]
struct Color {
    r: u8,
    g: u8,
    b: u8,
}

#[derive(Debug, Deserialize)]
struct ColorData {
    result: Vec<Color>,
}