use std::io::{self, BufRead, BufWriter, Write};

use rainbow_street::get_true_color;

const PS1_ESCPAE_SEQ_START: &str = "\\[";
const PS1_ESCPAE_SEQ_END: &str = "\\]";
const BASH_ESCPAE_SEQ: &str = "\x1b[";

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

pub fn colorize_strings(is_ps1_escape: bool, delimiter: &str) {
    let mut stream = BufWriter::new(io::stdout());
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = line.expect("Error when expecting string");
        let words: Vec<&str> = line.split(delimiter).collect();
        let mut colored_word: Vec<String> = Vec::new();
        for (index, word) in words.iter().enumerate() {
            let ancestors_and_me = &words[..index + 1].join("");
            let color = get_true_color(ancestors_and_me);
            colored_word.push(colorize(word, color, is_ps1_escape));
        }
        writeln!(&mut stream, "{}", colored_word.join(delimiter)).unwrap();
    }
}
