mod graphics;

fn main() {
    let raw_img = graphics::load_image("diams.png");
    let img = raw_img.unwrap();
    let w = img.width();
    let h = img.height();
    let img = graphics::resize_image(&img, w/2, h/4);
    graphics::print_image(img);
}
