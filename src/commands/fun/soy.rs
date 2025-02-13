use std::{io::Cursor, num::NonZeroU32, time::Instant};
use image::{GenericImageView, ImageFormat, ImageOutputFormat};
use crate::{helpers::images::fast_resize, Context, Error};
use image::{imageops, DynamicImage};
use poise::{serenity_prelude::{self as serenity, CreateAttachment}, CreateReply};
use tracing::{debug, info};
use lazy_static::lazy_static;


const MIN_W_GAP_PERCENTAGE: f64 = 0.3;
const MIN_H_GAP_TOP_PERCENTAGE: f64 = 0.15;

/// Add soy to an image
#[poise::command(prefix_command, slash_command, category = "Fun")]
pub async fn soy(
    ctx: Context<'_>,
    attachment: serenity::Attachment
) -> Result<(), Error> {
    ctx.defer().await?;

    let Some(ref mime) = attachment.content_type else {
        ctx.say("That's some wacky ahh attachment you got there, can't work with that").await?;
        return Ok(())
    };

    if !mime.starts_with("image") {
        ctx.say("I can only add soy to images").await?;
    }

    let ext = mime.split('/').last().unwrap_or("png");

    info!("starting download");
    let rn = Instant::now();
    let img_bytes = attachment.download().await?;
    info!("downloaded img, elapsed: {:?}", rn.elapsed());

    let rn = Instant::now();
    let new_img = soyjakify(&img_bytes, MIN_W_GAP_PERCENTAGE, MIN_H_GAP_TOP_PERCENTAGE, mime).await?;
    info!("soyjakified img, elapsed: {:?}", rn.elapsed());

    let attachment = CreateAttachment::bytes(new_img, format!("soyjakified.{}", ext));

    ctx.send(CreateReply::default()
        .attachment(attachment)
    ).await?;

    Ok(())
}


lazy_static! {
    static ref SOY_LEFT: Result<image::DynamicImage, image::ImageError> = {
        image::open("assets/soy-left.png")
    };
}

lazy_static! {
    static ref SOY_RIGHT: Result<image::DynamicImage, image::ImageError> = {
        image::open("assets/soy-right.png")
    };
}


/// The docs below are incomprehensible if you are hovering over the function name, so please check the source code instead.
/// 
/// =================================
/// |                               | <- min_h_gap_top (here the percentage is around 20%)
/// |-------------------------------|
/// |        |          |           |
/// |        |          |           |
/// |soy_left|          | soy_right |
/// |        |          |           |
/// |        |          |           |
/// =================================
///               ^
///               min-w-gap (here the percentage is around 33%)
async fn soyjakify(img: &[u8], min_w_gap_percentage: f64, min_h_gap_top_percentage: f64, mime: &str) -> Result<Vec<u8>, Box<dyn std::error::Error + Send + Sync>> {
    let img_format = ImageFormat::from_mime_type(mime).ok_or("Got an invalid image format")?;
    let img_output_format = ImageOutputFormat::from(img_format);

    debug!("time stats for soyjakifying");

    let rn = Instant::now();
    let base_image = image::load_from_memory(img)?;
    debug!("reading base image {:?}", rn.elapsed());

    let rn = Instant::now();
    let soy_left: &DynamicImage = SOY_LEFT.as_ref()?;
    debug!("reading soy_left {:?}", rn.elapsed());

    let rn = Instant::now();
    let soy_right: &DynamicImage = SOY_RIGHT.as_ref()?;
    debug!("reading soy_right {:?}", rn.elapsed());

    
    let rn = Instant::now();
    let scale_factor = get_scale_factor(
        ImgDimF64::from_img(&base_image), 
        ImgDimF64::from_img(soy_left), 
        ImgDimF64::from_img(soy_right), 
        min_w_gap_percentage, 
        min_h_gap_top_percentage
    )?;
    debug!("calculated scale factor {:?}", rn.elapsed());

    let rn = Instant::now();
    let soy_left = fast_resize_with_scale_factor(soy_left, scale_factor)?;
    debug!("resized soy_left {:?}", rn.elapsed());

    let rn = Instant::now();
    let soy_right = fast_resize_with_scale_factor(soy_right, scale_factor)?;
    debug!("resized soy_right {:?}", rn.elapsed());

    let rn = Instant::now();
    let new_image = overlay_soyjaks(&base_image, &soy_right, &soy_left).await;
    debug!("overlaid soys {:?}", rn.elapsed());

    // finally, saving the img
    let rn = Instant::now();
    let mut img_buffer = Cursor::new(Vec::new());
    new_image.write_to(&mut img_buffer, img_output_format)?;
    debug!("survived writing to img_buffer {:?}", rn.elapsed());

    Ok(img_buffer.into_inner())
}



async fn overlay_soyjaks(base_img: &DynamicImage, soy_right: &DynamicImage, soy_left: &DynamicImage) -> DynamicImage {
    let mut new_img = base_img.clone();

    imageops::overlay(&mut new_img, soy_left, 0, base_img.height() as i64 - soy_left.height() as i64);
    imageops::overlay(&mut new_img, soy_right, base_img.width() as i64 - soy_right.width() as i64, base_img.height() as i64 - soy_right.height() as i64);

    new_img
}


fn fast_resize_with_scale_factor(image: &DynamicImage, scale_factor: f64) -> Result<DynamicImage, Box<dyn std::error::Error + Send + Sync>> {
    let new_w = (image.width() as f64 * scale_factor).round() as u32;
    let new_h = (image.height() as f64 * scale_factor).round() as u32;

    let new_image = fast_resize(
        image, 
        NonZeroU32::new(new_w).unwrap(), 
        NonZeroU32::new(new_h).unwrap(),
        fast_image_resize::ResizeAlg::default()
    )?;

    Ok(new_image)
}

#[derive(Debug, Clone)]
struct ImgDimF64 {
    w: f64,
    h: f64
}

impl ImgDimF64 {
    fn new(w: u32, h: u32) -> Self {
        Self { w: w as f64, h: h as f64 }
    }

    fn from_tuple(tuple: (u32, u32)) -> Self {
        Self::new(tuple.0, tuple.1)
    }

    fn from_img(img: &DynamicImage) -> Self {
        Self::from_tuple(img.dimensions())
    }

    fn rescale(&self, scale_factor: f64) -> Self {
        Self {
            w: self.w * scale_factor,
            h: self.h * scale_factor
        }
    }
}

/// TODO: Write tests for this
fn get_scale_factor(base_dim: ImgDimF64, soy_left_dim: ImgDimF64, soy_right_dim: ImgDimF64, min_w_gap_percentage: f64, min_h_gap_top_percentage: f64) -> Result<f64, Box<dyn std::error::Error + Send + Sync>> {
    // safety checks for the percentages
    if min_w_gap_percentage > 1.0 || min_w_gap_percentage < 0.0 {
        return Err(format!("{} is not a valid width gap percentage", min_w_gap_percentage).into());
    }

    if min_h_gap_top_percentage > 1.0 || min_h_gap_top_percentage < 0.0 {
        return Err(format!("{} is not a valid height gap percentage", min_h_gap_top_percentage).into());
    }

    debug!("base img size: {:?}", base_dim);

    // prearing the available widths and heights
    let min_w_gap = base_dim.w * min_w_gap_percentage;
    let available_w = base_dim.w - min_w_gap;

    let min_h_gap_top = base_dim.h * min_h_gap_top_percentage;
    let available_h = base_dim.h - min_h_gap_top;

    // first, we need to guarantee that there's gonna be enough of a width gap between the 2 soyjaks
    let scale_factor = available_w / (soy_left_dim.w + soy_right_dim.w); 

    // we create our rescale candidates
    let new_soy_left_dim = soy_left_dim.rescale(scale_factor);
    let new_soy_right_dim = soy_right_dim.rescale(scale_factor);


    // pick out the higher ORIGINAL soyjak img (although they SHOULD BE the same height)
    let higher_one = if soy_left_dim.h > soy_right_dim.h {
        soy_left_dim.clone()
    } else {
        soy_right_dim.clone()
    };

    
    // second, we check if the scale factor that we got will result in the height fitting under the required height gap
    let final_scale_factor = if new_soy_left_dim.h.max(new_soy_right_dim.h) < available_h {
        // if yes, we keep the scale factor as is
        scale_factor
    } else {
        // otherwise, we scale it down further, to match the required height gap
        let fixed_factor = available_h / higher_one.h;

        fixed_factor
    };

    Ok(final_scale_factor)
}