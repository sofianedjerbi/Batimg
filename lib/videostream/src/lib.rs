//! A rust library that provided a simple iterator around video files.
//! **Author:** @expenses
//! **Forked by** Sofiane DJERBI (@Kugge)
//!
//! Requires [ffmpeg] to be installed.
//!
//! ## Example
//!
//! To read the first five frames of a video and write them to files is just:
//!
//! ```no_run
//! extern crate videostream;
//!
//! use videostream::VideoStream;
//!
//! fn main() {
//!     let mut stream = VideoStream::new("file.mp4").unwrap();
//!
//!     for (i, frame) in stream.iter().take(5).enumerate() {
//!         let image = frame.as_rgb().unwrap();
//!         image.save(&format!("{}.png", i));
//!     }
//! }
//! ```
//!
//! [ffmpeg]: https://ffmpeg.org

extern crate ffmpeg;
extern crate image;

use std::path::Path;

pub use ffmpeg::util::format::pixel::Pixel as PixelFormat;
use ffmpeg::util::frame::video::Video as InnerFrame;
use ffmpeg::codec::decoder::video::Video as Decoder;
use ffmpeg::media::Type;
use ffmpeg::format::context::Input;
use ffmpeg::format::context::input::PacketIter;
use ffmpeg::software::converter;
use image::{GrayImage, RgbImage, RgbaImage};

#[derive(Debug)]
pub enum Error {
    Static(&'static str),
    Ffmpeg(ffmpeg::Error),
}

impl From<ffmpeg::Error> for Error {
    fn from(error: ffmpeg::Error) -> Self {
        Error::Ffmpeg(error)
    }
}

impl From<&'static str> for Error {
    fn from(error: &'static str) -> Self {
        Error::Static(error)
    }
}

/// A videostream.
pub struct VideoStream {
    input: Input,
    stream: usize,
    decoder: Decoder,
}

impl VideoStream {
    /// Create a new videostream from a path (can be a file or a url).
    ///
    /// Returns an error if ffmpeg fails to initialise or a stream cannot be found in the input.
    pub fn new<P: AsRef<Path> + std::convert::AsRef<std::ffi::OsStr>>(path: P) -> Result<Self, Error> {
        ffmpeg::init()?;

        let input = ffmpeg::format::input(&path)?;

        let (stream, decoder) = {
            let stream = input
                .streams()
                .best(Type::Video)
                .ok_or("Failed to get stream")?;
            (stream.index(), stream.codec().decoder().video()?)
        };

        Ok(Self {
            input,
            stream,
            decoder,
        })
    }

    /// Create an iterator of frames in the video.
    pub fn iter(&mut self) -> Frames {
        Frames {
            packets: self.input.packets(),
            stream: self.stream,
            decoder: &mut self.decoder,
        }
    }
}

impl<'a> IntoIterator for &'a mut VideoStream {
    type Item = Frame;
    type IntoIter = Frames<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

/// An iterator of frames in a video.
pub struct Frames<'a> {
    decoder: &'a mut Decoder,
    stream: usize,
    packets: PacketIter<'a>,
}

impl<'a> Iterator for Frames<'a> {
    type Item = Frame;

    /// Return the next frame in the video.
    ///
    /// Stops and `eprintln`s an error if one occurred in decoding.
    fn next(&mut self) -> Option<Self::Item> {
        match self.packets.next()? {
            Ok((stream, packet)) => if stream.index() == self.stream {
                let mut output = InnerFrame::empty();

                match self.decoder.decode(&packet, &mut output) {
                    Ok(_) => if output.format() != PixelFormat::None {
                        Some(Frame::new(output))
                    } else {
                        self.next()
                    },
                    Err(error) => {
                        eprintln!("{}", error);
                        None
                    }
                }
            } else {
                self.next()
            },
            Err(_) => None,
        }
    }
}

/// A frame in the video, wrapping around a ffmpeg frame.
pub struct Frame {
    inner: InnerFrame,
}

impl Frame {
    fn new(inner: InnerFrame) -> Self {
        Self { inner }
    }

    /// Get the width of the frame.
    pub fn width(&self) -> u32 {
        self.inner.width()
    }

    /// Get the height of the frame.
    pub fn height(&self) -> u32 {
        self.inner.height()
    }

    /// Convert the frame to a rgba image.
    pub fn as_rgba(&self) -> Result<RgbaImage, Error> {
        let vec = self.as_vec(4, PixelFormat::RGBA)?;
        RgbaImage::from_raw(self.width(), self.height(), vec)
            .ok_or_else(|| "Failed to convert image".into())
    }

    /// Convert the frame to a rgb image.
    pub fn as_rgb(&self) -> Result<RgbImage, Error> {
        let vec = self.as_vec(3, PixelFormat::RGB24)?;
        RgbImage::from_raw(self.width(), self.height(), vec)
            .ok_or_else(|| "Failed to convert image".into())
    }

    /// Convert the frame to a luma (greyscale) image.
    pub fn as_luma(&self) -> Result<GrayImage, Error> {
        let vec = self.as_vec(1, PixelFormat::GRAY8)?;
        GrayImage::from_raw(self.width(), self.height(), vec)
            .ok_or_else(|| "Failed to convert image".into())
    }

    fn convert(&self, format: PixelFormat) -> Result<InnerFrame, Error> {
        let mut output = InnerFrame::empty();

        converter((self.width(), self.height()), self.inner.format(), format)?
            .run(&self.inner, &mut output)
            .map_err(Error::Ffmpeg)
            .map(|_| output)
    }

    /// Convert the frame to a vec of channels with a given pixel format.
    ///
    /// For example, to convert to rgb you would use:
    ///
    /// ```text
    /// frame.as_vec(3, PixelFormat::RGB24)
    /// ```
    pub fn as_vec(&self, channels: u32, format: PixelFormat) -> Result<Vec<u8>, Error> {
        let output = self.convert(format)?;

        let index = 0;
        let stride = output.stride(index);
        let width = (output.width() * channels) as usize;

        // If the stride and width are equal, just convert to a vec
        if stride == width {
            Ok(output.data(index).to_vec())
        // If they aren't, because the data has some garbage at the end of each line, skip over it
        } else {
            let mut offset = 0;
            let mut vec = Vec::with_capacity((self.width() * self.height() * channels) as usize);
            let data = output.data(index);

            while offset < data.len() {
                vec.extend_from_slice(&data[offset..offset + width]);
                offset += stride;
            }

            Ok(vec)
        }
    }
}

