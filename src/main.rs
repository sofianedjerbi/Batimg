// Uses the api provided in graphics.rs
// To build a cli tool that get images
// Author: Sofiane DJERBI (@Kugge)
use terminal_size::{Width, Height, terminal_size};
use clap::{App, Arg};
use ctrlc;

use std::fs::create_dir;
use std::path::Path;
use std::cmp::min;

mod graphics;


const SUPPORTED_VIDEOS: [&str; 23] = ["gif", "avi", "mp4", "mkv", "m2v", 
                                      "ogg", "ogv", "aac", "aax", 
                                      "mov", "wmv", "avchd", "m4p", 
                                      "f4v", "swf", "mkv", "yuv", "webm", 
                                      "amv", "m4v", "3gp", "3g2", "nsv"];


fn main() {
    // Handle CTRL + C (on videos)
    ctrlc::set_handler(move || {
        print!("\x1b[?25h"); // Show cursor again
        print!("\x1b[0J"); // Clear everything
        graphics::clean_tmp_files(); // Remove tmp files
        println!("Exiting.");
        std::process::exit(0); // Exit process
    }).expect("Error setting Ctrl-C handler");
    
    // Load cli config
    let matches = App::new("batimg")
        .version("1.1")
        .author("Sofiane D. <@Kugge>")
        .about("Graphic content on your tty")
        .arg(Arg::new("size")
            .short('s')
            .long("size")
            .help("Canvas size")
            .value_name("u32")
            .takes_value(true))
        .arg(Arg::new("timesync")
            .short('t')
            .long("timesync")
            .help("Disable realtime synchronization")
            .takes_value(false))
        .arg(Arg::new("debug")
            .short('d')
            .long("debug")
            .help("Print debug stats")
            .takes_value(false))
        .arg(Arg::new("audio")
            .short('a')
            .long("audio")
            .help("Play video audio")
            .takes_value(false))
        .arg(Arg::new("loop")
            .short('l')
            .long("loop")
            .help("Loop the video")
            .takes_value(false))
        .arg(Arg::new("resolution")
            .short('r')
            .long("resolution")
            .help("Disable high resolution mode (half pixel character)")
            .takes_value(false))
        .arg(Arg::new("prerender")
            .short('p')
            .long("prerender")
            .help("Export frames first (unstable)")
            .takes_value(false))
        .arg(Arg::new("FILE")
            .help("Path to the media")
            .value_name("FILE")
            .required(true)
            .takes_value(true)
            .index(1))
        .get_matches();

    // Variables to populate
    let file: &str;
    let height: u32;
    let is_video: bool;

    // Flag variables
    let debug: bool = matches.is_present("debug");
    let play_audio: bool = matches.is_present("audio");
    let timesync: bool = matches.is_present("timesync");
    let prerender: bool = matches.is_present("prerender");
    let resolution: bool = !matches.is_present("resolution");
    let mut loop_video: bool = matches.is_present("loop");

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
    }
    else if let Some((Width(w), Height(h))) = size { // In terminal
        height = min(h, w) as u32 - 1;
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

    // Check if the file exists
    if !Path::new(file).exists() {
        eprintln!("{}: No such media.", file);
        std::process::exit(1);
    }

    // Check for video
    match file.rsplit(".").next() {
        None      => std::process::exit(11),
        Some(ext) => {
            is_video = SUPPORTED_VIDEOS.contains(&ext);
            loop_video = &ext.eq("gif") ^ loop_video // If is a gif
        }
    }

    // PROCESS PICTURE
    if !is_video {
        graphics::process_image(file, height, resolution);
    }
    // PROCESS VIDEO
    else {
        create_dir(".adplaytmp").ok();
        if prerender {
            graphics::process_video_prerender(file, height, play_audio,
                                              resolution, loop_video,
                                              !timesync, debug);
        }
        else {
            graphics::process_video(file, height, play_audio,
                                    resolution, loop_video,
                                    !timesync, debug);
        }
    }
}

