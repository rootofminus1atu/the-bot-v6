use std::{io::Cursor, num::NonZeroU32};
use crate::{helpers::images::fast_resize, Context, Error};
use image::{GenericImage, GenericImageView, ImageOutputFormat, Rgba};
use poise::{serenity_prelude::{self as serenity, CreateAttachment}, CreateReply};
use lazy_static::lazy_static;
use tracing::info;
use image::Pixel;

const SPEECHBUBBLE_HEIGHT: f64 = 0.3;

#[poise::command(prefix_command, slash_command, category = "Fun")]
pub async fn speechbubble(
    ctx: Context<'_>,
    attachment: serenity::Attachment,
    speechbubble_position: Option<SpeechBubblePosition>
) -> Result<(), Error> {
    ctx.defer().await?;

    info!("started speechbubbling");

    let mime = attachment.content_type.as_ref().ok_or("That's some wacky ahh attachment you got there, can't work with that")?;

    if !mime.starts_with("image") {
        return Err("I can only add speechbubbles to images".into())
    }

    let img_bytes = attachment.download().await?;

    let new_img = speechbubbleify(&img_bytes, speechbubble_position.unwrap_or_default()).await?;

    let attachment = CreateAttachment::bytes(new_img, format!("speechbubbleified.gif"));

    ctx.send(CreateReply::default()
        .attachment(attachment)
    ).await?;

    info!("finished speechbubbling");

    Ok(())
}

lazy_static! {
    static ref TAIL_CENTER: Result<image::DynamicImage, image::ImageError> = {
        image::open("assets/tail_center.png")
    };
}

lazy_static! {
    static ref TAIL_LEFT: Result<image::DynamicImage, image::ImageError> = {
        image::open("assets/tail_left.png")
    };
}

lazy_static! {
    static ref TAIL_RIGHT: Result<image::DynamicImage, image::ImageError> = {
        image::open("assets/tail_right.png")
    };
}

#[derive(Debug, Clone, poise::ChoiceParameter, Default)]
pub enum SpeechBubblePosition {
    Left,
    #[default]
    Center,
    Right
}

impl SpeechBubblePosition {
    fn get_img(&self) -> &'static Result<image::DynamicImage, image::ImageError> {
        match self {
            Self::Left => &TAIL_LEFT,
            Self::Center => &TAIL_CENTER,
            Self::Right => &TAIL_RIGHT
        }
    }
}

async fn speechbubbleify(img: &[u8], speechbubble_position: SpeechBubblePosition) -> Result<Vec<u8>, Error> {
    let mut base_image = image::load_from_memory(img)?.into_rgba8();

    let chatbubble = speechbubble_position.get_img().as_ref()?;

    let chatbubble = fast_resize(
        chatbubble, 
        NonZeroU32::new(base_image.width()).unwrap(), 
        NonZeroU32::new((base_image.height() as f64 * SPEECHBUBBLE_HEIGHT).floor() as u32).unwrap(),
        fast_image_resize::ResizeAlg::Nearest
    )?;

    apply_cutout_mask(&mut base_image, &chatbubble);

    let mut img_buffer= Cursor::new(Vec::new());
    base_image.write_to(&mut img_buffer, ImageOutputFormat::Gif)?;

    Ok(img_buffer.into_inner())
}

fn apply_cutout_mask<I: GenericImage<Pixel = Rgba<u8>>, J: GenericImageView<Pixel = I::Pixel>>(base: &mut I, mask: &J) {
    let (width, height) = mask.dimensions();

    for y in 0..height {
        for x in 0..width {
            let mask_pixel = mask.get_pixel(x, y).to_rgba();
            let [r, g, b, _a] = mask_pixel.0;

            if r >= 240 && g >= 240 && b >= 240 {
                base.put_pixel(x, y, Rgba([0, 0, 0, 0]));
            }
        }
    }
}
