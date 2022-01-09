// Uses the api provided in graphics.rs
// To build a cli tool that get images
// Author: Sofiane DJERBI (@Kugge)
use std::path::Path;

use terminal_size::{Width, Height, terminal_size};

use ctrlc::set_handler;

use clap::Parser;

mod batimage;
mod batvideo;


/// Argument parser
#[derive(Parser)]
#[clap(name = "batimg")]
#[clap(author = "Sofiane DJERBI (@Kugge)")]
#[clap(version = "2.0.0")]
#[clap(about = "Fast image/video printing in your terminal")]
struct Args {
    /// Media path
    #[clap()]
    path: String,
    
    /// Media height in pixel
    #[clap(short, long, parse(try_from_str))]
    size: Option<u32>,

    /// Disable realtime synchronization
    #[clap(short, long)]
    timesync: bool,

    /// Repeat the video
    #[clap(short, long)]
    repeat: bool,

    /// Play video audio
    #[clap(short, long)]
    audio: bool,

    /// Disable high resolution rendering
    #[clap(short, long)]
    low: bool,
    
    /// Display debug info
    #[clap(short, long)]
    debug: bool,

}


fn main() {

    // Get args
    let args = Args::parse();
    let path = Path::new(&args.path);
    
    // To populate
    let mut height: u32;
    let mut video: bool;
    let mut repeat: bool = args.repeat;

    // Check if the file exists
    if !path.exists() {
        eprintln!("No such media: {}", args.path);
        std::process::exit(1);
    }
    
    // Determine the optimal size
    if let Some(h) = args.size {
        height = h;
    }
    else if let Some((Width(w), Height(h))) = terminal_size() {
        height = if h < w {h} else {w} as u32 - 1;
        println!("{:?} x {:?}: {:?}", w, h, height);
    }
    else { // Cannot get terminal size
        eprintln!("Unable to get canvas size, please use <--size> option.");
        std::process::exit(3);
    }

    // Check the extension
    match args.path.rsplit(".").next() {
        None      => {
            eprintln!("Unable to get file extension.");
            std::process::exit(11);
        },
        Some(ext) => {
            video = batvideo::SUPPORTED_VIDEOS.contains(&ext);
            repeat = &ext.eq("gif") ^ repeat // If is a gif
        }
    }

    // Handle CTRL + C
    ctrlc::set_handler(move || {
        print!("\x1b[?25h"); // Show cursor again
        print!("\x1b[0J"); // Clear everything
        println!("Exiting.");
        std::process::exit(0); // Exit process
    }).expect("Ctrl-C handler error");

    // Process video/picture
    if video {
        batvideo::process_video(path, height, args.audio, !args.low, 
                                repeat, !args.timesync, args.debug);
    }
    else {
        batimage::process_file(path, height, !args.low);
    }
}

