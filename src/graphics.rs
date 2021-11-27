/// graphics.rs - Load images and generate ascii data
use std::process::Command;
use std::time::{Duration, Instant};
use std::thread::sleep;
use std::str;

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
    return resize(img, w, h, FilterType::Nearest);
}

// Show image
pub fn print_image(image: RgbImage) {
    for i in 0..image.height() {
        for j in 0..image.width() {
            let px = image.get_pixel(j, i);
            printc!((*px)[0], (*px)[1], (*px)[2]);
        }
        print!("\x1b[0m\n");
    }
}

// Process and print an image
pub fn process_image(file: &str, height: u32){
    let raw_img = load_image(file);
    let img = match raw_img {
        Ok(pic) => pic,
        Err(_err) => {
            eprintln!("{}: Unknown file format.", file);
            std::process::exit(4);
        },
    };
    let w = img.width();
    let h = img.height();
    let img = resize_image(&img, w*height/h, height/2);
    print_image(img);
}

pub fn process_video(file: &str, height: u32){
    /*** PREPROCESSING ***/
    // Setting default incrementation (ideal)
    let mut incr: f64 = 1.;
    // Current frame
    let mut frame: f64 = 0.;
    // Getting total number of frames
    let raw_frames = Command::new("ffprobe")
        .arg("-v")
        .arg("error")
        .arg("-select_streams")
        .arg("v:0")
        .arg("-count_packets")
        .arg("-show_entries")
        .arg("stream=nb_read_packets")
        .arg("-of")
        .arg("csv=p=0")
        .arg(file)
        .output()
        .expect("Failed to execute FFprobe process.");
    let total_frames = str::from_utf8(&raw_frames.stdout)
        .unwrap()
        .replace("\n", "")
        .parse::<u32>()
        .unwrap() as f64;
    // Getting fps
    let ffprobe_fps = Command::new("ffprobe")
        .arg("-v")
        .arg("error")
        .arg("-select_streams")
        .arg("v")
        .arg("-show_entries")
        .arg("stream=r_frame_rate")
        .arg("-of")
        .arg("default=noprint_wrappers=1:nokey=1")
        .arg(file)
        .output()
        .expect("Failed to execute FFprobe process.");
    let str_fps = str::from_utf8(&ffprobe_fps.stdout)
        .unwrap()
        .replace("\n", "");
    let raw_fps: Vec<&str> = str_fps.split("/").collect();
    // dividende
    let fps1 = raw_fps[0].parse::<f64>().unwrap();
    // divisor
    let fps2 = raw_fps[1].parse::<f64>().unwrap();
    // Second per frame
    let spf = fps2/fps1;
    // Duration per frame
    let dpf = Duration::from_secs_f64(spf);
    // Hide cursor
    print!("\x1b[?25l");
    Command::new("clear").status().unwrap(); // Clear term
    /*** PROCESSING ***/
    while frame < total_frames {
        let now = Instant::now();
        // Get frame
        print!("\x1b[2H");
        Command::new("ffmpeg")
            .arg("-ss")
            .arg((spf*frame).to_string())
            .arg("-y")
            .arg("-i")
            .arg(file)
            .arg("-vf")
            .arg("select=eq(n\\,1)")
            .arg(".advideo.tmp.bmp")
            .output()
            .expect("Failed to execute FFmpeg process.");
        // Print frame
        process_image(&".advideo.tmp.bmp", height);
        // Check fps, and sleep if needed
        match dpf.saturating_mul(incr as u32)
                 .checked_sub(now.elapsed()) {
            Some(duration) => sleep(duration),
            None => incr += 1. // Incr frameskip if cant keep up
        };
        frame += incr;
    }
    Command::new("clear").status().unwrap(); // Clear term
    print!("\x1b[?25h");
}

