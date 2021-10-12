use image::Reader;

mod graphics {
    /// Print with colors (r, g, b) on the foreground
    /// Works on true colors terminal
    #[macro_export]
    macro_rules! printcf {
        ($t: expr, $r: expr, $g: expr, $b: expr) => {
            println!("\x1b[38;2;{};{};{}m{}", $r, $g, $b, $t);
        }
    }
    /// Print with colors (r, g, b) on the background
    /// Works on true colors terminall
    #[macro_export]
    macro_rules! printcb {
        ($t: expr, $r: expr, $g: expr, $b: expr) => {
            println!("\x1b[48;2;{};{};{}m{}", $r, $g, $b, $t);
        }
    }

    fn load_image(path: str) -> Reader {
        let image: Reader = Reader::open("path/to/image.png")?.decode()?;
        return image;
    }
}
