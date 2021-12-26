# ADPlay
<p align="center">
  <img src="./demo.gif">
</p>

`adplay` is a small program written in Rust, designed to be fast and compatible with every terminal.   
It can show images/videos in almost [every format](https://ffmpeg.org/ffmpeg-formats.html).

## Build
Build bin and install dependencies: `cargo build --release`  
Bin location: `./target/release/adplay`

## Usage
```
USAGE:
    adplay [OPTIONS] <FILE>

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
    adplay img.png
    adplay img.jpg -s 100
    adplay video.mp4 -a
    adplay animation.gif
```

