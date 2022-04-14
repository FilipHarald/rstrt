use md5::Digest;
use std::io::{self, BufRead};

const COLOR_BASE: u8 = 128;
const DELIMETER: &str = "/";

fn main() {
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = line.expect("¿No?");
        let colored_line = get_color(line);
        println!("{}", colored_line);
    }
}

fn get_color(str: String) -> String {
    let colored_str = str.clone();
    let digest: [u8; 16] = md5::compute(str).into();
    let palette = get_palette(digest);
    print!("{:?}", palette);
    return colored_str;
}

fn get_palette(dig: [u8; 16]) -> [u8; 3] {
    let red = dig[0] % 128;
    let green = dig[1] % 128;
    let blue = dig[2] % 128;
    let palette_nbr = dig[0] % 3;
    match palette_nbr {
        0 => {
            return [
                COLOR_BASE + red,
                COLOR_BASE + green,
                COLOR_BASE
            ]
        }
        1 => {
            return [
                COLOR_BASE + red,
                COLOR_BASE,
                COLOR_BASE + blue
            ]
        }
        2 => {
            return [
                COLOR_BASE,
                COLOR_BASE + green,
                COLOR_BASE + blue
            ]
        }
        _ => {
            return [
                COLOR_BASE + 64,
                COLOR_BASE + 64,
                COLOR_BASE + 64
            ]
        }
    }
}
