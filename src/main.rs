use terminal_size::{Width, Height, terminal_size};
use clap::{App, Arg};

use std::path::Path;
use std::cmp::min;

mod graphics;


const SUPPORTED_VIDEOS: [&str; 23] = ["gif", "avi", "mp4", "mkv", "m2v", 
                                      "ogg", "ogv", "aac", "aax", 
                                      "mov", "wmv", "avchd", "m4p", 
                                      "f4v", "swf", "mkv", "yuv", "webm", 
                                      "amv", "m4v", "3gp", "3g2", "nsv"];


fn main() {
    // Load cli config
    let matches = App::new("Ascii DRIP")
        .version("1.0")
        .author("Sofiane D. <@Kugge>")
        .about("Graphic content on your tty")
        .arg(Arg::new("size")
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

    // GET CANVAS SIZE
    let size = terminal_size(); // Request term size
    if let Some(h) = matches.value_of("size") { // In options
        height = match h.parse::<u32>() {
            Ok(num)   => num,
            Err(_err) => {
                eprintln!("<--size> should be an unsigned integer.");
                std::process::exit(7);
            }
        };
        println!("{}", height);
    }
    else if let Some((Width(w), Height(h))) = size { // In terminal
        height = min(h, w/2) as u32;
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


    if !Path::new(file).exists() {
        eprintln!("{}: No such media.", file);
        std::process::exit(1);
    }

    match file.rsplit(".").next() {
        None      => std::process::exit(11),
        Some(ext) => {
            is_video = SUPPORTED_VIDEOS.contains(&ext);
        }
    }

    // PROCESS PICTURE
    if !is_video {
        let raw_img = graphics::load_image(file);
        let img = match raw_img {
            Ok(pic) => pic,
            Err(_err) => {
                eprintln!("{}: Unknown file format.", file);
                std::process::exit(4);
            },
        };
        let w = img.width();
        let h = img.height();
        let img = graphics::resize_image(&img, w*height/h, height/2);
        graphics::print_image(img);
    }
}

