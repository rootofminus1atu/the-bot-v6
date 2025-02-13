use std::num::NonZeroU32;
use image::GenericImageView;
use image::DynamicImage;
use fast_image_resize as fr;
use image::RgbaImage;


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