/// graphics.rs - Load images and generate ascii data
use image::imageops::FilterType;
use image::imageops::resize;
use image::ImageError;
use image::io::Reader;
use image::RgbImage;

/// Print with colors (r, g, b) on the foreground
#[macro_export]
macro_rules! printcf {
    ($t: expr, $r: expr, $g: expr, $b: expr) => {
        print!("\x1b[38;2;{};{};{}m{}", $r, $g, $b, $t);
    }
}
/// Print with colors (r, g, b) on the background
#[macro_export]
macro_rules! printcb {
    ($t: expr, $r: expr, $g: expr, $b: expr) => {
        print!("\x1b[48;2;{};{};{}m{}", $r, $g, $b, $t);
    }
}

/// Print with colors (r, g, b) on both background and foreground
#[macro_export]
macro_rules! printca {
     ($t: expr, $r: expr, $g: expr, $b: expr) => {
        print!("\x1b[48;2;{r};{g};{b}m\x1b[38;2;{r};{g};{b}m{t}",
            r=$r, g=$g, b=$b, t=$t);
    }
}

/// Print a square of a single (r, g, b) color
#[macro_export]
macro_rules! printc {
     ($r: expr, $g: expr, $b: expr) => {
        printca!("X", $r, $g, $b)
    }
}

// Load an image
pub fn load_image(path: &str) -> Result<RgbImage, ImageError> {
    let image = Reader::open(path)?.decode()?;
    return Ok(image.to_rgb8());
}

// Resize image
pub fn resize_image(img: &RgbImage, w: u32, h: u32) -> RgbImage {
    return resize(img, w, h, FilterType::Lanczos3);
}

// Show image
pub fn print_image(image: RgbImage){
    for i in 0..image.height() {
        for j in 0..image.width() {
            let px = image.get_pixel(j, i);
            printc!((*px)[0], (*px)[1], (*px)[2]);
        }
        print!("\x1b[0m\n");
    }
}
