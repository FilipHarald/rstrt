use std::env;

mod vim_status_line;
mod bash_ansi;

const DELIMETER: &str = "/";

fn main() {
    let args: Vec<String> = env::args().collect();
    let is_dark_text = args.contains(&String::from("--dark-text"));
    let is_vim_status_line = args.contains(&String::from("--vim-status-line"));
    
    if is_vim_status_line {
        let pwd_index = args.iter().position(|arg| arg == "--vim-status-line").map(|i| i + 1).unwrap_or(2);
        let branch_index = pwd_index + 1;
        if pwd_index < args.len() && branch_index < args.len() {
            vim_status_line::build_status_line(DELIMETER, &args[pwd_index], &args[branch_index], is_dark_text);
        } else {
            eprintln!("Not enough arguments for vim status line mode");
        }
    } else {
        let is_ps1_escape = args.contains(&String::from("--ps1-escape"));
        bash_ansi::colorize_strings(is_ps1_escape, DELIMETER, is_dark_text);
    }
}
