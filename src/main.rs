// Uses the api provided in graphics.rs
// To build a cli tool that get images
// Author: Sofiane Djerbi (@sofianedjerbi)
use terminal_size::{Width, Height, terminal_size};
use clap::{App, Arg};
use ctrlc;
use regex::Regex;

use std::path::Path;
use std::process::{Command, Stdio};
use std::io::{BufRead, BufReader};
use std::thread;
use std::time::Duration;
use std::sync::{Arc, atomic::{AtomicBool, Ordering}};

mod graphics;


const SUPPORTED_VIDEOS: [&str; 23] = ["gif", "avi", "mp4", "mkv", "m2v",
                                      "ogg", "ogv", "aac", "aax",
                                      "mov", "wmv", "avchd", "m4p",
                                      "f4v", "swf", "mkv", "yuv", "webm",
                                      "amv", "m4v", "3gp", "3g2", "nsv"];

fn is_youtube_url(input: &str) -> bool {
    let youtube_regex = Regex::new(
        r"^(https?://)?(www\.)?(youtube\.com/(watch\?v=|shorts/)|youtu\.be/)[\w-]+"
    ).unwrap();
    youtube_regex.is_match(input)
}

fn download_youtube_video(url: &str) -> Result<String, String> {
    println!("Fetching YouTube video...");

    // Create a temporary directory for the video
    let temp_dir = tempfile::tempdir()
        .map_err(|e| format!("Failed to create temp directory: {}", e))?;
    let temp_path = temp_dir.path().join("video.mp4");

    // Start loading animation in a separate thread
    let loading = Arc::new(AtomicBool::new(true));
    let loading_clone = Arc::clone(&loading);

    let spinner_thread = thread::spawn(move || {
        let frames = vec!["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];
        let mut idx = 0;
        while loading_clone.load(Ordering::Relaxed) {
            print!("\r{} Downloading... ", frames[idx]);
            std::io::Write::flush(&mut std::io::stdout()).ok();
            idx = (idx + 1) % frames.len();
            thread::sleep(Duration::from_millis(80));
        }
        print!("\r\x1b[K"); // Clear the line
        std::io::Write::flush(&mut std::io::stdout()).ok();
    });

    // Use yt-dlp to download the video with progress output
    let child_result = Command::new("yt-dlp")
        .arg("-f")
        .arg("best[height<=720][ext=mp4]/best[height<=720]/best")
        .arg("--no-playlist")
        .arg("--quiet")
        .arg("--progress")
        .arg("--newline")
        .arg("-o")
        .arg(&temp_path)
        .arg(url)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn();

    let mut child = match child_result {
        Ok(c) => c,
        Err(e) => {
            loading.store(false, Ordering::Relaxed);
            spinner_thread.join().ok();
            return Err(format!("Failed to execute yt-dlp: {}. Make sure yt-dlp is installed.", e));
        }
    };

    // Read stderr in real-time for progress
    if let Some(stderr) = child.stderr.take() {
        let reader = BufReader::new(stderr);
        for line in reader.lines() {
            if let Ok(line) = line {
                // Show download progress percentage
                if line.contains("%") && line.contains("of") {
                    print!("\r\x1b[K{}", line.trim());
                    std::io::Write::flush(&mut std::io::stdout()).ok();
                }
            }
        }
    }

    let status = match child.wait() {
        Ok(s) => s,
        Err(e) => {
            loading.store(false, Ordering::Relaxed);
            spinner_thread.join().ok();
            return Err(format!("Failed to wait for yt-dlp: {}", e));
        }
    };

    // Stop the loading animation
    loading.store(false, Ordering::Relaxed);
    spinner_thread.join().ok();

    if !status.success() {
        return Err(format!("yt-dlp failed with exit code: {:?}", status.code()));
    }

    if !temp_path.exists() {
        return Err("Download failed: video file not created".to_string());
    }

    println!("\r\x1b[K✓ Download complete!");

    // Convert path to string and leak the temp dir to keep file alive
    let path_str = temp_path.to_str().unwrap().to_string();
    std::mem::forget(temp_dir); // Keep temp dir alive
    Ok(path_str)
}


fn main() {
    // Handle CTRL + C (on videos)
    ctrlc::set_handler(move || {
        print!("\x1b[2J");     // Clear entire screen
        print!("\x1b[H");      // Move cursor to home position
        print!("\x1b[?25h");   // Show cursor again
        print!("\x1b[0m");     // Reset all text attributes
        std::io::Write::flush(&mut std::io::stdout()).ok();
        graphics::clean_tmp_files(); // Remove tmp files
        std::process::exit(0); // Exit process cleanly
    }).expect("Error setting Ctrl-C handler");

    // Load cli config
    let matches = App::new("batimg")
        .version("1.1")
        .author("Sofiane Djerbi <@sofianedjerbi>")
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
        .arg(Arg::new("FILE")
            .help("Path to the media")
            .value_name("FILE")
            .required(true)
            .takes_value(true)
            .index(1))
        .get_matches();

    // Variables to populate
    let file: String;
    let height: u32;
    let width: u32;
    let is_video: bool;

    // Flag variables
    let debug: bool = matches.is_present("debug");
    let play_audio: bool = matches.is_present("audio");
    let timesync: bool = matches.is_present("timesync");
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
        width = height * 2; // Assume 2:1 ratio for characters
    }
    else if let Some((Width(w), Height(h))) = size { // In terminal
        height = (h as u32).saturating_sub(1);
        width = w as u32;
    }
    else { // Cannot get terminal size
        eprintln!("Unable to get canvas size, please use <--size> option.");
        std::process::exit(3);
    }

    // GET INPUT FILE OR URL
    let input = matches.value_of("FILE").unwrap_or_else(|| {
        eprintln!("No media specified.");
        std::process::exit(1);
    });

    // Check if input is a YouTube URL
    if is_youtube_url(input) {
        file = match download_youtube_video(input) {
            Ok(path) => path,
            Err(e) => {
                eprintln!("Error downloading YouTube video: {}", e);
                std::process::exit(2);
            }
        };
    } else {
        // Check if the file exists
        if !Path::new(input).exists() {
            eprintln!("{}: No such media.", input);
            std::process::exit(1);
        }
        file = input.to_string();
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
        graphics::process_image(&file, height, resolution);
    }
    // PROCESS VIDEO
    else {
        graphics::process_video(&file, width, height, play_audio,
                                resolution, loop_video,
                                !timesync, debug);
    }
}

