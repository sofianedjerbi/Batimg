/// batimage.rs - Load images
/// Author: Sofiane DJERBI (@Kugge)
use std::path::Path;

use image::imageops::FilterType;
use image::imageops::resize;
use image::io::Reader;
use image::ImageError;
use image::RgbaImage;

use batimg_core;

/// Load an image
/// # Parameters
/// - `path`: Path to the picture
pub fn load_image(path: &Path) -> Result<RgbaImage, ImageError> {
    let image = Reader::open(path)?.decode()?;
    return Ok(image.to_rgba8());
}

/// Resize an image
/// # Parameters
/// - `image`: RGBA image object
pub fn resize_image(image: &RgbaImage, w: u32, h: u32) -> RgbaImage {
    return resize(image, w, h, FilterType::Nearest);
}

/// Process and print an image
/// # Parameters:
/// - image: RgbaImage representing the image to be printed
/// - `height`: Height of the image in characters
/// - `res`: Are we using the half pixel mode ?
pub fn process_image(image: RgbaImage, height: u32, res: bool) {
    let w = image.width();
    let h = image.height();
    if res {
        let image = resize_image(&image, 2*w*height/h, 2*height);
        batimg_core::print_image_hpm(image);
    }
    else {
        let image = resize_image(&image, 2*w*height/h, height);
        batimg_core::print_image(image);
    }
}

/// Wrapper for **process_image**
/// # Parameters:
/// - `file`: Path to the image
/// - `height`: Height of the image in characters
/// - `res`: Are we using the half pixel mode ?
pub fn process_file(file: &Path, height: u32, res: bool) {
    let raw_img = load_image(file);
    let img = match raw_img {
        Ok(pic) => pic,
        Err(_err) => {
            eprintln!("{}: Unknown file format.", file.display());
            std::process::exit(4);
        },
    };
    process_image(img, height, res);
}

