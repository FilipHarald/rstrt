use std::{io::{self, BufRead, BufWriter, Write}, env};

const COLOR_BASE: u8 = 128;
const DELIMETER: &str = "/";
const PS1_ESCPAE_SEQ_START: &str = "\\[";
const PS1_ESCPAE_SEQ_END: &str = "\\]";
const BASH_ESCPAE_SEQ: &str = "\x1b[";

fn get_color(str: &String) -> [u8; 3] {
    let digest: [u8; 16] = md5::compute(str).into();
    let red = digest[0] >> 1;
    let green = digest[1] >> 1;
    let blue = digest[2] >> 1;
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




fn colorize(str: &&str, color: [u8; 3], escape_ps1: bool) -> String {
    let r = color[0];
    let g = color[1];
    let b = color[2];
    let mut res = String::new();
    if escape_ps1 {
        res.push_str(PS1_ESCPAE_SEQ_START);
    }
    res.push_str(BASH_ESCPAE_SEQ);
    res.push_str(format!("0;38;2;{};{};{}m", r, g, b).as_str());
    if escape_ps1 {
        res.push_str(PS1_ESCPAE_SEQ_END);
    }
    res.push_str(str);
    if escape_ps1 {
        res.push_str(PS1_ESCPAE_SEQ_START);
    }
    res.push_str(BASH_ESCPAE_SEQ);
    res.push_str("0m");
    if escape_ps1 {
        res.push_str(PS1_ESCPAE_SEQ_END);
    }
    return res;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let is_ps1_escape = args.len() > 1 && args[1] == "--ps1-escape";
    let mut stream = BufWriter::new(io::stdout());
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = line.expect("¿No?");
        let words: Vec<&str> = line.split(DELIMETER).collect();
        let mut colored_word: Vec<String> = Vec::new();
        for (index, word) in words.iter().enumerate() {
            let ancestors_and_me = &words[..index+1].join("");
            let color = get_color(ancestors_and_me);
            colored_word.push(colorize(word, color, is_ps1_escape));
        }
        writeln!(&mut stream, "{}", colored_word.join(DELIMETER)).unwrap();
    }
}
