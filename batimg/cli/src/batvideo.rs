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

use rodio::{Source, Sink, Decoder, OutputStream};
use image::RgbaImage;

use crate::batimage;


/// Supported video formats
pub const SUPPORTED_VIDEOS: [&str; 23] = ["gif", "avi", "mp4", "mkv", "m2v",
                                          "ogg", "ogv", "aac", "aax",
                                          "mov", "wmv", "avchd", "m4p",
                                          "f4v", "swf", "mkv", "yuv", "webm",
                                          "amv", "m4v", "3gp", "3g2", "nsv"];

/// Extract an audio source
/// # Parameters
/// - `file`: Path to the file
fn extract_audio(file: &str) -> Decoder<BufReader<File>> {
    Command::new("ffmpeg")
        .arg("-y")
        .arg("-i")
        .arg(file)
        .arg("-q:a")
        .arg("0")
        .arg("-map")
        .arg("a")
        .arg(".adplaytmp.mp3")
        .output()
        .expect("Failed to extract audio with FFmpeg.");
    // Deconding mp3
    let filemp3 = BufReader::new(
        match File::open(".adplaytmp.mp3") {
            Ok(obj)   => obj,
            Err(_err) => {
                eprintln!("Video does not contain any audio.");
                std::process::exit(8);
            }
        }
    );
    return Decoder::new(filemp3).unwrap();
}

/// Print a video using ffmpeg
/// # Parameters
/// - `file`: Path to the image
/// - `height`: Height of the image
/// - `audio`: Are we playing the audio ?
/// - `res`: Are we using the half pixel mode ?
/// - `loop_video`: Loop the video ?
/// - `sync`: Activate realtime syncing ?
/// - `debug`: Print debug info ?
pub fn process_video(file: &Path, height: u32, audio: bool,
                     res: bool, loop_video: bool, sync: bool,
                     debug: bool) {
    // NOT IMPLEMENTED YET
}
