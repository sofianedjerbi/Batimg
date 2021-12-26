# ADPlay
<p align="center">
  <img src="./demo.gif">
</p>

`batimg` is a small program written in Rust, designed to be fast and compatible with every terminal.   
It can print images/videos in almost [every format](https://ffmpeg.org/ffmpeg-formats.html) in your terminal.

## Build
Build bin and install dependencies: `cargo build --release`  
Bin location: `./target/release/adplay`

## Usage
```
USAGE:
    batimg [OPTIONS] <FILE>

ARGS:
    <FILE>    Path to the media

OPTIONS:
    -a, --audio         Play video audio (unstable)
    -h, --help          Print help information
    -l, --loop          Loop the video 
    -s, --size <u32>    Canvas size
    -r, --resolution    Disable high resolution mode (half pixel character)
    -p, --prerender     Export frames first (unstable)
    -V, --version       Print version information

EXAMPLES: 
    batimg img.png
    batimg img.jpg -s 100
    batimg video.mp4 -a
    batimg animation.gif
```

