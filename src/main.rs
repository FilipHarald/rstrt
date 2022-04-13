use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = line.expect("¿No?");
        let colored_line = get_color(line);
        println!("{:x}", colored_line);
    }
}

fn get_color(str : String) -> md5::Digest {
    let digest = md5::compute(str);
    return digest;
}
