/// graphics.rs - Load images and generate ascii data
use std::fs::{File, remove_dir_all};
use std::time::{Duration, Instant};
use std::process::Command;
use std::thread::sleep;
use std::io::BufReader;
use std::str;

use rodio::{Sink, Decoder, OutputStream};

use rayon::prelude::*;

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

// Returns (total frame number, second per frame)
fn get_frame_info(file: &str) -> (f64, f64) {
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
        .split("\n")
        .next()
        .unwrap();
    let raw_fps: Vec<&str> = str_fps.split("/").collect();
    println!("{:?}", raw_fps);
    // dividende
    let fps1 = raw_fps[0].parse::<f64>().unwrap();
    // divisor
    let fps2 = raw_fps[1].parse::<f64>().unwrap();
    // Second per frame
    let spf = fps2/fps1;
    return (total_frames, spf)
}

pub fn clean_tmp_files(){
    remove_dir_all(".adplaytmp").ok();
}

pub fn process_video(file: &str, height: u32, audio: bool){
    /*** PREPROCESSING ***/
    // Setting default incriementation (ideal)
    let mut incr: f64 = 1.;
    // Current frame
    let mut frame: f64 = 0.;
    // Getting total number of frames and frames per sec
    let (total_frames, spf) = get_frame_info(file);
    // Duration per frame
    let dpf = Duration::from_secs_f64(spf);
    // Hide cursor
    print!("\x1b[?25l");
    Command::new("clear").status().unwrap(); // Clear term
    
    /*** AUDIO ***/
    // Using rodio to play audio
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();
    // Extract and play audio
    if audio {
        Command::new("ffmpeg")
            .arg("-y")
            .arg("-i")
            .arg(file)
            .arg("-q:a")
            .arg("0")
            .arg("-map")
            .arg("a")
            .arg(".adplaytmp/tmp.mp3")
            .output()
            .expect("Failed to extract audio with FFmpeg.");
        // Deconding mp3
        let filemp3 = BufReader::new(
            match File::open(".adplaytmp/tmp.mp3") {
                Ok(obj)   => obj,
                Err(_err) => {
                    println!("Video does not contain any audio.");
                    std::process::exit(8);
                }
            }
        );
        let source = Decoder::new(filemp3).unwrap();
        sink.append(source);
    }
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
            .arg("-frames:v")
            .arg("1")
            .arg(".adplaytmp/tmp.bmp")
            .output()
            .expect("Failed to execute FFmpeg process.");
        // Print frame
        process_image(&".adplaytmp/tmp.bmp", height);
        // Check fps, and sleep if needed
        match dpf.saturating_mul(incr as u32)
                 .checked_sub(now.elapsed()) {
            Some(duration) => sleep(duration),
            None => incr += 1. // Incr frameskip if cant keep up
        };
        frame += incr;
    }
    Command::new("clear").status().unwrap(); // Clear term
    print!("\x1b[?25h"); // Show cursor
    clean_tmp_files();
}

pub fn process_video_prerender(file: &str, height: u32, audio: bool){
    /*** PREPROCESSING ***/
    // Setting default incriementation (ideal)
    let mut incr: f64 = 1.;
    // Current frame
    let mut frame: f64 = 0.;
    // Getting total number of frames and frames per sec
    let (total_frames, spf) = get_frame_info(file);
    // Duration per frame
    let dpf = Duration::from_secs_f64(spf);
    // Hide cursor
    print!("\x1b[?25l");
    Command::new("clear").status().unwrap(); // Clear term
    
    /*** PRERENDERING ***/
    // Let the user know we're not dead
    println!("Extracting frames... (Might take a while)");
    // Extracting every frame (Multithreaded)
    (0..(total_frames as u64)).into_par_iter().for_each(|i| {
        Command::new("ffmpeg")
            .arg("-ss")
            .arg((spf*(i as f64)).to_string())
            .arg("-y")
            .arg("-i")
            .arg(file)
            .arg("-frames:v")
            .arg("1")
            .arg(format!(".adplaytmp/{}.bmp", i))
            .output()
            .expect("Failed to execute FFmpeg process.");
    });

    /*** AUDIO ***/
    // Using rodio to play audio
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();
    // Extract and play audio
    if audio {
        Command::new("ffmpeg")
            .arg("-y")
            .arg("-i")
            .arg(file)
            .arg("-q:a")
            .arg("0")
            .arg("-map")
            .arg("a")
            .arg(".adplaytmp/tmp.mp3")
            .output()
            .expect("Failed to extract audio with FFmpeg.");
        // Deconding mp3
        let filemp3 = BufReader::new(
            match File::open(".adplaytmp/tmp.mp3") {
                Ok(obj)   => obj,
                Err(_err) => {
                    println!("Video does not contain any audio.");
                    std::process::exit(8);
                }
            }
        );
        let source = Decoder::new(filemp3).unwrap();
        sink.append(source);
    }
    /*** PROCESSING ***/
    while frame < total_frames {
        let now = Instant::now();
        // Get frame
        print!("\x1b[2H");
        // Print frame
        process_image(&format!(".adplaytmp/{}.bmp", frame), height);
        // Check fps, and sleep if needed
        match dpf.saturating_mul(incr as u32)
                 .checked_sub(now.elapsed()) {
            Some(duration) => sleep(duration),
            None => incr += 1. // Incr frameskip if cant keep up
        };
        frame += incr;
    }
    Command::new("clear").status().unwrap(); // Clear term
    print!("\x1b[?25h"); // Show cursor
    clean_tmp_files();
}

