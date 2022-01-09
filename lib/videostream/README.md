# Videostream

A rust library that provided a simple iterator around video files.

Requires [ffmpeg] to be installed.

## Example

To read the first five frames of a video and write them to files is just:

```rust
extern crate videostream;

use videostream::VideoStream;

fn main() {
    let mut stream = VideoStream::new("file.mp4").unwrap();

    for (i, frame) in stream.iter().take(5).enumerate() {
        let image = frame.as_rgb().unwrap();
        image.save(&format!("{}.png", i));
    }
}
```

[ffmpeg]: https://ffmpeg.org