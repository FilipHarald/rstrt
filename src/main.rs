use std::io::{self, BufRead};

const COLOR_BASE: u8 = 128;
const DELIMETER: &str = "/";

fn get_color(str: &String) -> [u8; 3] {
    let digest: [u8; 16] = md5::compute(str).into();
    let red = digest[0] % 128;
    let green = digest[1] % 128;
    let blue = digest[2] % 128;
    let palette_nbr = digest[0] % 3;
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

fn colorize(str: &&str, color: [u8; 3]) -> String {
    let r = color[0];
    let g = color[1];
    let b = color[2];
    return format!("\u{001B}[0;38;2;{};{};{}m{}\x1b[00m", r, g, b, str);
}

fn colorize_prompt(str: &&str, color: [u8; 3]) -> String {
    let r = color[0];
    let g = color[1];
    let b = color[2];
    return format!("\\[\u{001B}[0;38;2;{};{};{}m\\]{}\\[\x1b[00m\\]", r, g, b, str);
}

fn main() {
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = line.expect("¿No?");
        let words: Vec<&str> = line.split(DELIMETER).collect();
        let mut colored_word: Vec<String> = Vec::new();
        for (index, word) in words.iter().enumerate() {
            let ancestors_and_me = &words[..index+1].join("");
            let color = get_color(ancestors_and_me);
            colored_word.push(colorize(word, color));
        }
        println!("{}", colored_word.join(DELIMETER));
    }
}
