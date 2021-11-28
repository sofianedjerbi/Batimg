# ADPlay
ADPlay: Pics & videos on your terminal  
(FFmpeg required for videos)

<p align="center">
  <img src="./demo.png">
</p>

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
    -s, --size <u32>    Canvas size
    -p, --prerender     Export frames first (unstable)
    -V, --version       Print version information

EXAMPLES: 
    adplay img.png
    adplay img.jpg -s 100
```

