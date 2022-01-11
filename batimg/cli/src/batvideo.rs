/// batvideo.rs - Video processing
/// Author: Sofiane DJERBI (@Kugge)
use std::process::Command;
use std::thread::sleep;
use std::time::{Duration, Instant};

use std::io::{stdout, Write};
use std::io::BufReader;

use std::path::Path;
use std::fs::File;
use std::str;
use std::fs;

use rodio::{Source, Sink, Decoder, OutputStream};
use image::RgbaImage;

use crate::batvideostream::VideoStream;
use crate::batimage;

/// Supported video formats
pub const SUPPORTED_VIDEOS: [&str; 23] = ["gif", "avi", "mp4", "mkv", "m2v",
                                          "ogg", "ogv", "aac", "aax",
                                          "mov", "wmv", "avchd", "m4p",
                                          "f4v", "swf", "mkv", "yuv", "webm",
                                          "amv", "m4v", "3gp", "3g2", "nsv"];


/// Get SPF (second per frame)
/// # Parameters
/// - `file`: File path
/// # Returns
/// Second per frame (f64)
fn get_frame_info(path: &Path) -> f64 {
    let file = path.to_str().unwrap();
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
    // dividende
    let fps1 = raw_fps[0].parse::<f64>().unwrap();
    // divisor
    let fps2 = raw_fps[1].parse::<f64>().unwrap();
    // Second per frame
    let spf = fps2/fps1;
    return spf
}

/// Extract an audio source
/// # Parameters
/// - `file`: Path to the file
fn extract_audio(path: &Path) -> Decoder<BufReader<File>> {
    let file = path.to_str().unwrap();
    // Extract audio to mp3
    Command::new("ffmpeg")
        .arg("-y")
        .arg("-i")
        .arg(file)
        .arg("-q:a")
        .arg("0")
        .arg("-map")
        .arg("a")
        .arg("/tmp/batimgtmp.mp3")
        .output()
        .expect("Failed to extract audio with FFmpeg.");
    // Deconding mp3
    let filemp3 = BufReader::new(
        match File::open("/tmp/batimgtmp.mp3") {
            Ok(obj)   => obj,
            Err(_err) => {
                eprintln!("Cannot read audio file.");
                std::process::exit(8);
            }
        }
    );
    return Decoder::new(filemp3).unwrap();
}

/// Clean temp files (/tmp/batimgtmp.mp3)
fn clean_tmp_files() {
    match fs::remove_file("/tmp/batimgtmp.mp3") {
        Ok(_) => (),
        Err(e) => {
            eprintln!("Cannot delete temp files: {:?}", e);
            std::process::exit(15);
        }
    };
}

/// Print a video using ffmpeg
/// # Parameters
/// - `path`: Path to the image
/// - `height`: Height of the image
/// - `audio`: Are we playing the audio ?
/// - `res`: Are we using the half pixel mode ?
/// - `loop_video`: Loop the video ?
/// - `sync`: Activate realtime syncing ?
/// - `debug`: Print debug info ?
pub fn process_video(path: &Path, height: u32, audio: bool,
                     res: bool, loop_video: bool, sync: bool,
                     debug: bool) {
    // Getting second per frame
    let spf = get_frame_info(path);
    // Duration per frame
    let dpf = Duration::from_secs_f64(spf);
    // Setting default frameskip
    let mut frameskip: usize = 0;
    // Parse video stream
    let mut stream = VideoStream::new(path);
    let mut iter = stream.iter();
    // Using rodio to play audio
    let (_, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();
    // Extract and play audio
    if audio {
        let source = extract_audio(path);
        sink.append(source.repeat_infinite());
    }
    // Hide cursor
    print!("\x1b[?25l");
    // Iter frames
    while let Some(frame) = iter.next() {
        // Convert to RGBA
        let argb_frame = match frame.as_rgba() {
            Ok(image) => image,
            Err(e) => {
                eprintln!("Cannot convert frame to rgba: {:?}", e);
                std::process::exit(30);
            }
        };
        // Show image
        batimage::process_image(path, height, res);
        // Apply frameskip
        iter.skip(frameskip);
        // Show debug infos
        if debug {
            println!("Frameskip: {:?}", frameskip);
        }
        stdout().flush().unwrap();
        print!("\x1b[{}F", height); // Goto beginning
    }
    print!("\x1b[?2J"); // Clean
    print!("\x1b[?25h"); // Show cursor
}

