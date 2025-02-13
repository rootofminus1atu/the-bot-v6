use std::io::Cursor;
use std::num::NonZeroU32;
use image::GenericImage;
use image::GenericImageView;
use image::DynamicImage;
use fast_image_resize as fr;
use image::ImageFormat;
use image::ImageOutputFormat;
use image::Rgba;
use image::RgbaImage;
use image::Pixel;
use lazy_static::lazy_static;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    let test_img = std::fs::read("assets/test.png")?;

    println!("Processing image...");
    
    // Process the image with the speech bubble
    let result = speechbubbleify(
        &test_img,
        "image/png",
        SpeechBubblePosition::Center
    ).await?;

    std::fs::write("output.png", result)?;

    println!("Done! Check output.png");

    Ok(())
}

const SPEECHBUBBLE_HEIGHT: f64 = 0.3;

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

#[derive(Debug, Clone)]
pub enum SpeechBubblePosition {
    Left,
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

fn apply_cutout_mask<I: GenericImage<Pixel = Rgba<u8>>, J: GenericImageView<Pixel = I::Pixel>>(base: &mut I, mask: &J) {
    let (width, height) = mask.dimensions();
    let mut non_transparent_pixels = 0;

    for y in 0..height {
        for x in 0..width {
            let mask_pixel = mask.get_pixel(x, y).to_rgba();
            let [r, g, b, a] = mask_pixel.0;

            if r >= 240 && g >= 240 && b >= 240 && a == 255 {
                non_transparent_pixels += 1;
                base.put_pixel(x, y, Rgba([0, 0, 0, 0]));  // Make the pixel fully transparent
            }
        }
    }

    println!("Applied mask to {} pixels out of {} total pixels", non_transparent_pixels, width * height);
}

async fn speechbubbleify(img: &[u8], mime: &str, speechbubble_position: SpeechBubblePosition) -> Result<Vec<u8>, Box<dyn std::error::Error + Send + Sync>> {
    let img_format = ImageFormat::from_mime_type(mime).ok_or("Got an invalid image format")?;
    // let img_output_format = ImageOutputFormat::from(img_format);

    let mut base_image = image::load_from_memory(img)?.into_rgba8();

    let chatbubble = speechbubble_position.get_img().as_ref()?;

    let chatbubble = fast_resize(
        chatbubble, 
        NonZeroU32::new(base_image.width()).unwrap(), 
        NonZeroU32::new((base_image.height() as f64 * SPEECHBUBBLE_HEIGHT).floor() as u32).unwrap(),
        fast_image_resize::ResizeAlg::Nearest
    )?;
    
    // save chatbubble to file for inspection
    let chatbubble_path = std::path::Path::new("chatbubble.png");
    let mut chatbubble_file = std::fs::File::create(&chatbubble_path)?;
    chatbubble.write_to(&mut chatbubble_file, ImageOutputFormat::Png)?;

    apply_cutout_mask(&mut base_image, &chatbubble);

    let mut img_buffer = Cursor::new(Vec::new());
    base_image.write_to(&mut img_buffer, ImageOutputFormat::Png)?;

    Ok(img_buffer.into_inner())
}


pub fn fast_resize(image: &DynamicImage, new_w: NonZeroU32, new_h: NonZeroU32, resize_alg: fr::ResizeAlg) -> Result<DynamicImage, Box<dyn std::error::Error + Send + Sync>> {
    let (w, h) = image.dimensions();
    let mut src_img = fr::Image::from_vec_u8(
        NonZeroU32::new(w).unwrap(),
        NonZeroU32::new(h).unwrap(),
        image.clone().into_rgba8().into_raw(),
        fast_image_resize::PixelType::U8x4,
    )?;

    // Multiple RGB channels of source image by alpha channel 
    // (not required for the Nearest algorithm)
    let alpha_mul_div = fr::MulDiv::default();
    alpha_mul_div.multiply_alpha_inplace(&mut src_img.view_mut())?;

    // Create container for data of destination image
    let mut dst_image = fr::Image::new(
        new_w,
        new_h,
        src_img.pixel_type(),
    );

    // Get mutable view of destination image data
    let mut dst_view = dst_image.view_mut();

    // Create Resizer instance and resize source image
    // into buffer of destination image
    let mut resizer = fr::Resizer::new(
        resize_alg,
    );
    resizer.resize(&src_img.view(), &mut dst_view)?;

    // Divide RGB channels of destination image by alpha
    alpha_mul_div.divide_alpha_inplace(&mut dst_view)?;

    // save the new image
    let new_img = RgbaImage::from_raw(
        new_w.get(), 
        new_h.get(), 
        dst_image.into_vec()
    )
    .ok_or("Could not create image from raw u8 bytes after resizing")?;
    
    Ok(new_img.into())
}



// fn apply_cutout_mask2<I: GenericImage<Pixel = Rgba<u8>>, J: GenericImageView<Pixel = I::Pixel>>(base: &mut I, mask: &J) {
//     let (width, height) = mask.dimensions();
//     let mut white_pixels_found = 0;

//     for y in 0..height {
//         for x in 0..width {
//             let mask_pixel = mask.get_pixel(x, y).to_rgba();
//             let [r, g, b, a] = mask_pixel.0;
            
//             // Debug log for middle row to see what values we're getting
//             if y == height/2 && x % 100 == 0 {
//                 println!("Mask pixel at ({}, {}): r={}, g={}, b={}, a={}", x, y, r, g, b, a);
//             }

//             if r == 255 && g == 255 && b == 255 && a == 255 {
//                 white_pixels_found += 1;
//                 base.put_pixel(x, y, Rgba([0, 0, 0, 0]));  // Let's try full transparency first
//             }
//         }
//     }

//     println!("Found {} white pixels out of {} total pixels", white_pixels_found, width * height);
// }