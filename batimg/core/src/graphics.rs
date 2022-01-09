/// graphics.rs - Manage escape code graphics
/// Author: Sofiane DJERBI (@Kugge)


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

