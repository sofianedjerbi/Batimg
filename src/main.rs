use terminal_size::{Width, Height, terminal_size};
use clap::{App, Arg};
use std::path::Path;
use std::cmp::min;

mod graphics;


fn main() {
    // Load cli config
    let matches = App::new("Ascii DRIP")
        .version("0.1")
        .author("Sofiane D. <@Kugge>")
        .about("Graphic content on your tty")
        .arg(Arg::new("SIZE")
            .short('s')
            .long("size")
            .value_name("u32")
            .about("Canvas size")
            .takes_value(true))
        .arg(Arg::new("FILE")
            .about("Path to the media")
            .value_name("FILE")
            .required(true)
            .takes_value(true)
            .index(1))
        .get_matches();

    // Variables to populate
    let file: &str;
    let height: u32;
    let is_video: bool;
    let is_picture: bool;

    // GET CANVAS SIZE
    let size = terminal_size(); // Request term size
    if let Some(h) = matches.value_of("size") { // In options
        height = h.parse().unwrap();
    }
    else if let Some((Width(w), Height(h))) = size { // In terminal
        //width=w as u32;
        height=min(h, w/2) as u32;
    }
    else { // Cannot get terminal size
        eprintln!("Unable to get canvas size, please use <--size> option.");
        std::process::exit(3);
    }

    // GET INPUT FILE
    if let Some(f) = matches.value_of("FILE") {
        file = &f;
    } else {
        eprintln!("No media specified.");
        std::process::exit(1);
    }

    //is_video = matches.is_present("video");
    //is_picture = matches.is_present("picture");

    if !Path::new(file).exists() {
        eprintln!("{}: No such media.", file);
        std::process::exit(1);
    }

    // PROCESS PICTURE
    //if is_picture {
    let raw_img = graphics::load_image(file);
    let img = match raw_img {
        Ok(pic) => pic,
        Err(err) => {
            eprintln!("There was an error reading media {}", file);
            eprintln!("Error: {:?}", err);
            std::process::exit(4);
        },
    };
    let w = img.width();
    let h = img.height();
    let img = graphics::resize_image(&img, w*height/h, height/2);
    graphics::print_image(img);
}

