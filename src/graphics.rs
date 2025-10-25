/// graphics.rs - Load images and generate ascii data
/// Author: Sofiane Djerbi (@sofianedjerbi)
use std::fs::File;
use std::time::{Duration, Instant};
use std::io::{stdout, Write, BufReader};
use std::thread::sleep;

use rodio::{Source, Sink, Decoder, OutputStream};

use image::imageops::FilterType;
use image::imageops::resize;
use image::{ImageError, RgbaImage, ImageBuffer};
use image::io::Reader;

use ffmpeg_next as ffmpeg;
use ffmpeg::{format, media, codec, software::scaling};
use ffmpeg::util::frame::video::Video as VideoFrame;


/// Print with colors (r, g, b) on the foreground
#[macro_export]
macro_rules! printcf {
    ($t: expr, $r: expr, $g: expr, $b: expr) => {
        print!("\x1b[0m\x1b[38;2;{};{};{}m{}", $r, $g, $b, $t);
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

/// Half-pixel resolution: Print two pixels r, g, b(f/b)
#[macro_export]
macro_rules! printhp {
     ($rf: expr, $gf: expr, $bf: expr,
      $rb: expr, $gb: expr, $bb: expr) => {
        print!("\x1b[38;2;{};{};{}m\x1b[48;2;{};{};{}m▀",
               $rf, $gf, $bf, $rb, $gb, $bb)
    }
}


/// Print a space (empty character)
#[macro_export]
macro_rules! printe {
    () => {
    print!("\x1b[0m ")
    }
}


/// Load an image
/// # Parameters
/// - `path`: Path to the image
pub fn load_image(path: &str) -> Result<RgbaImage, ImageError> {
    let image = Reader::open(path)?.decode()?;
    return Ok(image.to_rgba8());
}

/// Resize an image
/// # Parameters
/// - `image`: RGBA image object
pub fn resize_image(image: &RgbaImage, w: u32, h: u32) -> RgbaImage {
    return resize(image, w, h, FilterType::Nearest);
}

/// Show an image
/// # Parameters
/// - `image`: RGBA image object
pub fn print_image(image: RgbaImage) {
    for i in 0..image.height() {
        for j in 0..image.width() {
            let px = image.get_pixel(j, i);
            if (*px)[3] == 0 { // Transparent
                printe!()
            }
            else {
                printc!((*px)[0], (*px)[1], (*px)[2]);
            }
        }
        print!("\x1b[0m\n");
    }
}

/// Show an image: Half pixel mode
/// # Parameters
/// - `image`: RGBA image object
pub fn print_image_hpm(image: RgbaImage) {
    for i in (0..image.height()-1).step_by(2) {
        for j in 0..image.width() {
            let pxu = image.get_pixel(j, i);   // Upper pixel
            let pxl = image.get_pixel(j, i+1); // Lower pixel
            if (*pxu)[3] == 0 && (*pxl)[3] == 0 { // Both transparent
                printe!()
            }
            else if (*pxu)[3] == 0 { // Upper transparent
                printcf!("▄", (*pxl)[0], (*pxl)[1], (*pxl)[2]);
            }
            else if (*pxl)[3] == 0 { // Lower transparent
                printcf!("▀", (*pxu)[0], (*pxu)[1], (*pxu)[2]);
            }
            else {
                printhp!((*pxu)[0], (*pxu)[1], (*pxu)[2],
                         (*pxl)[0], (*pxl)[1], (*pxl)[2]);
            }
        }
        print!("\x1b[0m\n");
    }
}

/// Process and print an image
/// # Parameters:
/// - `file`: Path to the image
/// - `height`: Height of the image in characters
/// - `res`: Are we using the half pixel mode ?
pub fn process_image(file: &str, height: u32, res: bool){
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
    if res {
        let img = resize_image(&img, 2*w*height/h, 2*height);
        print_image_hpm(img);
    }
    else {
        let img = resize_image(&img, 2*w*height/h, height);
        print_image(img);
    }
}

/// Convert FFmpeg video frame to RgbaImage
/// # Parameters
/// - `frame`: FFmpeg video frame
/// - `scaler`: FFmpeg scaler context
fn frame_to_rgba(frame: &VideoFrame, scaler: &mut scaling::Context) -> Result<RgbaImage, String> {
    let mut rgb_frame = VideoFrame::empty();
    scaler.run(frame, &mut rgb_frame)
        .map_err(|e| format!("Scaling error: {}", e))?;

    let width = rgb_frame.width();
    let height = rgb_frame.height();
    let data = rgb_frame.data(0);
    let stride = rgb_frame.stride(0);

    let mut img_data = Vec::with_capacity((width * height * 4) as usize);

    for y in 0..height {
        let row_start = (y * stride as u32) as usize;
        for x in 0..width {
            let pixel_start = row_start + (x * 4) as usize;
            img_data.push(data[pixel_start]);     // R
            img_data.push(data[pixel_start + 1]); // G
            img_data.push(data[pixel_start + 2]); // B
            img_data.push(data[pixel_start + 3]); // A
        }
    }

    ImageBuffer::from_raw(width, height, img_data)
        .ok_or_else(|| "Failed to create image buffer".to_string())
}

/// Extract audio source from video using FFmpeg decoder
/// # Parameters
/// - `file`: Path to the file
fn extract_audio(file: &str) -> Result<Decoder<BufReader<File>>, String> {
    // Create temp audio file
    let temp_audio = format!("/tmp/batimg_audio_{}.mp3", std::process::id());

    // Extract audio using FFmpeg
    let output = std::process::Command::new("ffmpeg")
        .arg("-y")
        .arg("-i")
        .arg(file)
        .arg("-q:a")
        .arg("0")
        .arg("-map")
        .arg("a")
        .arg(&temp_audio)
        .output()
        .map_err(|e| format!("Failed to execute FFmpeg: {}", e))?;

    if !output.status.success() {
        return Err("Video does not contain audio".to_string());
    }

    // Open and decode the audio file
    let audio_file = File::open(&temp_audio)
        .map_err(|_| "Failed to open extracted audio".to_string())?;

    Decoder::new(BufReader::new(audio_file))
        .map_err(|e| format!("Failed to decode audio: {}", e))
}

/// Cleanup temp audio file
pub fn clean_tmp_files(){
    std::fs::remove_file(format!("/tmp/batimg_audio_{}.mp3", std::process::id())).ok();
}

/// Print a video using native FFmpeg decoder (no disk I/O, no subprocess spawning)
/// # Parameters
/// - `file`: Path to the video file
/// - `width`: Width of the terminal in characters
/// - `height`: Height of the terminal in characters
/// - `audio`: Are we playing the audio?
/// - `res`: Are we using the half pixel mode?
/// - `loop_video`: Loop the video?
/// - `sync`: Activate realtime syncing?
/// - `debug`: Print debug info?
pub fn process_video(file: &str, width: u32, height: u32, audio: bool,
                     res: bool, loop_video: bool, sync: bool,
                     debug: bool) {
    // Clear screen and hide cursor FIRST (before any processing)
    print!("\x1b[2J");        // Clear entire screen
    print!("\x1b[H");         // Move cursor to home position
    print!("\x1b[?25l");      // Hide cursor
    stdout().flush().unwrap();

    // Initialize FFmpeg and suppress log output
    ffmpeg::init().expect("Failed to initialize FFmpeg");
    ffmpeg::util::log::set_level(ffmpeg::util::log::Level::Quiet);

    /*** OPEN VIDEO FILE ***/
    let mut ictx = format::input(&file)
        .expect("Failed to open video file");

    // Find video stream
    let video_stream_index = ictx.streams()
        .best(media::Type::Video)
        .expect("No video stream found")
        .index();

    let video_stream = ictx.stream(video_stream_index).unwrap();

    // Get video metadata
    let frame_rate = video_stream.avg_frame_rate();
    let spf = frame_rate.1 as f64 / frame_rate.0 as f64; // seconds per frame
    let dpf = Duration::from_secs_f64(spf);

    // Get total frames (if available)
    let total_frames = video_stream.frames() as f64;

    /*** SETUP DECODER ***/
    let context_decoder = codec::context::Context::from_parameters(video_stream.parameters())
        .expect("Failed to create codec context");
    let mut decoder = context_decoder.decoder().video()
        .expect("Failed to create video decoder");

    // Setup scaler to convert to RGBA
    let mut scaler = scaling::Context::get(
        decoder.format(),
        decoder.width(),
        decoder.height(),
        ffmpeg::format::Pixel::RGBA,
        decoder.width(),
        decoder.height(),
        scaling::Flags::BILINEAR,
    ).expect("Failed to create scaler");

    /*** AUDIO ***/
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();
    if audio {
        if let Ok(source) = extract_audio(file) {
            sink.append(source.repeat_infinite());
        }
    }

    /*** PROCESSING ***/
    let mut frame_num: f64 = 0.;
    let mut incr: f64 = 1.;

    'main_loop: loop {
        let mut reached_end = false;

        for (stream, packet) in ictx.packets() {
            if stream.index() != video_stream_index {
                continue;
            }

            let now = Instant::now();

            // Decode packet
            decoder.send_packet(&packet).ok();

            let mut decoded = VideoFrame::empty();
            while decoder.receive_frame(&mut decoded).is_ok() {
                // Convert frame to RgbaImage
                if let Ok(rgba_img) = frame_to_rgba(&decoded, &mut scaler) {
                    // Calculate dimensions to fit terminal while maintaining aspect ratio
                    let video_w = rgba_img.width() as f32;
                    let video_h = rgba_img.height() as f32;
                    let aspect_ratio = video_w / video_h;

                    // Calculate target dimensions based on resolution mode
                    let (target_h, target_w) = if res {
                        // In high-res mode, we use half-pixel characters (2 pixels per char vertically)
                        let max_h = height * 2;
                        let calc_w = (max_h as f32 * aspect_ratio) as u32;

                        if calc_w <= width {
                            (max_h, calc_w)
                        } else {
                            // Width constraint is tighter
                            let fit_h = (width as f32 / aspect_ratio) as u32;
                            (fit_h, width)
                        }
                    } else {
                        // Normal mode: 1 pixel per character vertically
                        let calc_w = (height as f32 * aspect_ratio * 2.0) as u32; // *2 for char width/height ratio

                        if calc_w <= width {
                            (height, calc_w)
                        } else {
                            // Width constraint is tighter
                            let fit_h = (width as f32 / (aspect_ratio * 2.0)) as u32;
                            (fit_h, width)
                        }
                    };

                    let resized_img = resize_image(&rgba_img, target_w, target_h);

                    // Calculate actual display height (in terminal lines)
                    let display_height = if res {
                        (target_h + 1) / 2  // Half-pixel mode uses 2 pixels per line
                    } else {
                        target_h
                    };

                    // Print the frame
                    if res {
                        print_image_hpm(resized_img);
                    } else {
                        print_image(resized_img);
                    }

                    // FPS sync
                    if sync {
                        match dpf.saturating_mul(incr as u32).checked_sub(now.elapsed()) {
                            Some(duration) => sleep(duration),
                            None => incr += 1., // Frame skip if can't keep up
                        };
                    }

                    frame_num += incr;

                    // Debug info
                    if debug {
                        print!("Frame: {} | Frameskip: {}", frame_num, incr);
                    }

                    stdout().flush().unwrap();
                    print!("\x1b[{}F", display_height); // Move cursor to beginning
                }

                // Check if we've reached the end
                if total_frames > 0.0 && frame_num >= total_frames {
                    reached_end = true;
                    break;
                }
            }

            if reached_end {
                break;
            }
        }

        // Handle end of video
        if reached_end || !loop_video {
            if loop_video && reached_end {
                // Seek back to beginning for loop
                ictx.seek(0, ..0).ok();
                decoder.flush();
                frame_num = 0.;
                continue 'main_loop;
            }
            break 'main_loop;
        }

        // If packets ended naturally and we're looping
        if loop_video {
            ictx.seek(0, ..0).ok();
            decoder.flush();
            frame_num = 0.;
        } else {
            break;
        }
    }

    print!("\x1b[?2J"); // Clean screen
    print!("\x1b[?25h"); // Show cursor
    clean_tmp_files();
}


