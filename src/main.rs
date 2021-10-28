use terminal_size::{Width, Height, terminal_size};
use clap::{App, Arg};

mod graphics;


fn main() {
    // Load cli config
    let matches = App::new("Ascii DRIP")
        .version("0.0")
        .author("Sofiane D. <@Kugge>")
        .about("Graphic content on your tty")
        //.arg(Arg::new("picture")
        //    .short('p')
        //    .long("picture")
        //    .about("Show a picture"))
        .arg(Arg::new("size")
            .short('s')
            .long("size")
            .value_name("u32")
            .about("Canvas size")
            .takes_value(true))
        .arg(Arg::new("FILE")
            .about("Sets the input file to use")
            .required(true)
            .index(1))
        .get_matches();

    // Variables to populate
    let file: &str;
    //let width: u32;
    let height: u32;
    //let is_video: bool;
    //let is_picture: bool;

    // GET CANVAS SIZE
    let size = terminal_size(); // Request term size
    if let Some(h) = matches.value_of("size") { // In options
        height = h.parse().unwrap();
    }
    else if let Some((Width(_w), Height(h))) = size { // In terminal
        //width=w as u32;
        height=h as u32;
    }
    else { // Cannot get terminal size
        println!("Unable to get canvas size, please use <--size WxH>.");
        return
    }

    // GET INPUT FILE
    if let Some(f) = matches.value_of("FILE") {
        file = &f;
    } else {
        println!("No input files.");
        return
    }

    //is_video = matches.is_present("video");
    //is_picture = matches.is_present("picture");

    /*
    if is_picture && is_video {
        println!("error: Incompatible flags --video and --picture");
        return
    }
    else if !is_picture && !is_video {
        println!("error: Please specify either --video or --picture");
        return
    }

    // PROCESS PICTURE
    if is_picture {
    */
    let raw_img = graphics::load_image(file);
    let img = raw_img.unwrap();
    let w = img.width();
    let h = img.height();
    let img = graphics::resize_image(&img, w*height/h, height/2);
    graphics::print_image(img);
}

