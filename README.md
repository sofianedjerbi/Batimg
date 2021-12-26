# batimg
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
    -d, --debug         Print debug stats
    -a, --audio         Play video audio (unstable)
    -h, --help          Print help information
    -l, --loop          Loop the video 
    -s, --size <u32>    Canvas size
    -r, --resolution    Disable high resolution mode (half pixel character)
    -p, --prerender     Export frames first (unstable)
    -t, --timesync      Disable realtime synchronization
    -V, --version       Print version information

EXAMPLES: 
    batimg img.png
    batimg img.jpg -s 100
    batimg video.mp4 -a
    batimg animation.gif
```

## batimg vs catimg

<div align="center">
	<table>
	<thead>
	  <tr>
	    <th></th>
	    <th>**batimg**</th>
	    <th>**catimg**</th>
	  </tr>
	</thead>
	<tbody>
	  <tr>
	    <td>**creation date**</td>
	    <td>2021</td>
	    <td>2013</td>
	  </tr>
	  <tr>
	    <td>**language**</td>
	    <td>rust</td>
	    <td>shell/c</td>
	  </tr>
	  <tr>
	    <td>**format**</td>
	    <td><a href="https://ffmpeg.org/ffmpeg-formats.html">[almost all](https://ffmpeg.org/ffmpeg-formats.html)</a></td>
	    <td>png/jpg/gif</td>
	  </tr>
	  <tr>
	    <td>**dependencies**</td>
	    <td>ffmpeg (videos)</td>
	    <td>imagemagick</td>
	  </tr>
	  <tr>
	    <td>**resize algorithm**</td>
	    <td>nearest neighbor</td>
	    <td>nearest color</td>
	  </tr>
	  <tr>
	    <td>**resolution**</td>
	    <td>▀ / █</td>
	    <td>▀ / ██</td>
	  </tr>
	  <tr>
	    <td>**video support**</td>
	    <td>yes</td>
	    <td>no</td>
	  </tr>
	  <tr>
	    <td>**audio support**</td>
	    <td>yes</td>
	    <td>no</td>
	  </tr>
	  <tr>
	    <td>**CPU usage**</td>
	    <td>too high</td>
	    <td>medium</td>
	  </tr>
	  <tr>
	    <td>**prerendering**</td>
	    <td>Disabled by default</td>
	    <td>Always enabled</td>
	  </tr>
	  <tr>
	    <td>**time sync**</td>
	    <td>Enabled by default</td>
	    <td>No</td>
	  </tr>
	</tbody>
	</table>
</div>

### Rendering comparison

<p align="center">
  <img src="./rendering.gif">
</p>

