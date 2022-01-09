/// lib.rs - High level wrapper for escape codes graphics API
/// Author: Sofiane DJERBI (@Kugge)
use image::RgbaImage;

mod graphics;

/// Show an image
/// # Parameters
/// - `image`: RGBA image object
pub fn print_image(image: RgbaImage) {
    for i in 0..image.height() {
        for j in 0..image.width() {
            let px = image.get_pixel(j, i);
            if (*px)[3] == 0 { // Transparent
                printe!()
            }
            else {
                printc!((*px)[0], (*px)[1], (*px)[2]);
            }
        }
        print!("\x1b[0m\n");
    }
}

/// Show an image: Half pixel mode
/// # Parameters
/// - `image`: RGBA image object
pub fn print_image_hpm(image: RgbaImage) {
    for i in (0..image.height()-1).step_by(2) {
        for j in 0..image.width() {
            let pxu = image.get_pixel(j, i);   // Upper pixel
            let pxl = image.get_pixel(j, i+1); // Lower pixel
            if (*pxu)[3] == 0 && (*pxl)[3] == 0 { // Both transparent
                printe!()
            }
            else if (*pxu)[3] == 0 { // Upper transparent
                printcf!("▄", (*pxl)[0], (*pxl)[1], (*pxl)[2]);
            }
            else if (*pxl)[3] == 0 { // Lower transparent
                printcf!("▀", (*pxu)[0], (*pxu)[1], (*pxu)[2]);
            }
            else {
                printhp!((*pxu)[0], (*pxu)[1], (*pxu)[2],
                         (*pxl)[0], (*pxl)[1], (*pxl)[2]);
            }
        }
        print!("\x1b[0m\n");
    }
}

