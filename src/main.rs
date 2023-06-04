use std::env;

mod vim_status_line;
mod bash_ansi;

const DELIMETER: &str = "/";

fn main() {
    let args: Vec<String> = env::args().collect();
    let is_vim_status_line = args.len() > 1 && args[1] == "--vim-status-line";
    if is_vim_status_line {
        vim_status_line::colorize_string(DELIMETER);
    } else {
        let is_ps1_escape = args.len() > 1 && args[1] == "--ps1-escape";
        bash_ansi::colorize_strings(is_ps1_escape, DELIMETER);
    }
}
