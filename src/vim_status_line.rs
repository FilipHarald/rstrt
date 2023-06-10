use std::io::{self, BufWriter, Write};

use rainbow_street::get_true_color;

struct VimHighlight {
    foreground: String,
    // TODO: add support for these
    //    background: String,
    //    bold: bool,
    //    ctermbg: u8,
    // NOTE: if vim is not using termguicolors, then ctermfg will be used. Inspiration here: https://github.com/m00qek/baleia.nvim/blob/main/lua/baleia/colors/xterm.lua
    //    ctermfg: u8,
    //    italic: bool,
    //    reverse: bool,
    //    special: String,
    //    strikethrough: bool,
    //    underline: bool,
}

impl std::fmt::Debug for VimHighlight {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("")
            .field("\"foreground\"", &self.foreground)
            .finish()
    }
}

struct VimStatusLine {
    filename_string: String,
    highlights: Vec<VimHighlight>,
}

impl std::fmt::Debug for VimStatusLine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("")
            .field("\"filename_string\"", &self.filename_string)
            .field("\"definitions\"", &self.highlights)
            .finish()
    }
}

pub fn build_status_line(delimiter: &str, file_path: &str, cwd: &str) {
    let mut stream = BufWriter::new(io::stdout());

    let mut status_line_obj: VimStatusLine = VimStatusLine {
        filename_string: String::new(),
        highlights: Vec::new(),
    };

    let words: Vec<&str> = file_path.split(delimiter).skip(1).collect();
    let cwd_words: Vec<&str> = cwd.split(delimiter).skip(1).collect();

    let build_path_from_index = if words.starts_with(&cwd_words) {
        cwd_words.len() - 1
    } else {
        status_line_obj.filename_string.push_str("/");
        0
    };
    let colorize_from_index = if words.len() > 9 {
        words.len()
        .saturating_sub(9)
        .saturating_add(build_path_from_index)
    } else {
        build_path_from_index
    };

    for (index, word) in words.iter().enumerate() {
        let ancestors_and_me = &words[..index + 1].join("");
        if index >= build_path_from_index {
            if index >= colorize_from_index {
                let color = get_true_color(ancestors_and_me);
                let xterm_definition = VimHighlight {
                    foreground: format!("#{:02x}{:02x}{:02x}", color[0], color[1], color[2]),
                };
                status_line_obj.highlights.push(xterm_definition);
                status_line_obj.filename_string.push_str(&format!(
                    "%{}*",
                    (index.saturating_sub(colorize_from_index) + 1)
                ));
            }
            status_line_obj.filename_string.push_str(word);
            if index != words.len() - 1 {
                status_line_obj.filename_string.push_str(delimiter);
            }
        }
    }
    writeln!(&mut stream, "{:?}", status_line_obj).unwrap();
}
