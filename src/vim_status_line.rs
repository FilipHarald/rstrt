use std::io::{self, BufRead, BufWriter, Write};

use rainbow_street::get_true_color;

struct XtermDefinition {
//    background: String,
//    bold: bool,
//    ctermbg: String,
//    ctermfg: u8,
    foreground: String,
//    italic: bool,
//    reverse: bool,
//    special: String,
//    strikethrough: bool,
//    underline: bool,
}

impl std::fmt::Debug for XtermDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("")
            .field("\"foreground\"", &self.foreground)
            .finish()
    }
}

struct StatusLine {
    filename_string: String,
    definitions: Vec<XtermDefinition>,
}

impl std::fmt::Debug for StatusLine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("")
            .field("\"filename_string\"", &self.filename_string)
            .field("\"definitions\"", &self.definitions)
            .finish()
    }
}

pub fn colorize_string(delimiter: &str) {
    let mut stream = BufWriter::new(io::stdout());
    let stdin = io::stdin();
    let line = stdin.lock().lines().next().unwrap().unwrap();

    let mut status_line_obj: StatusLine = StatusLine {
        filename_string: String::new(),
        definitions: Vec::new(),
    };

    let words: Vec<&str> = line.split(delimiter).skip(1).collect();

    let colorize_from_index = words.len().saturating_sub(9);
    for (index, word) in words.iter().enumerate() {
        let ancestors_and_me = &words[..index + 1].join("");
        if index >= colorize_from_index {
            let color = get_true_color(ancestors_and_me);
            let xterm_definition = XtermDefinition {
//                background: String::from("none"),
//                bold: false,
//                ctermbg: String::from("none"),
//                ctermfg: 120, // TODO: actually calculate this
                foreground: format!("#{:02x}{:02x}{:02x}", color[0], color[1], color[2]),
//                italic: false,
//                reverse: false,
//                special: String::from("none"),
//                strikethrough: false,
//                underline: false,
            };
            status_line_obj.definitions.push(xterm_definition);
            status_line_obj.filename_string.push_str(&format!(
                "%{}*",
                (index.saturating_sub(colorize_from_index) + 1)
            ));
        }
        status_line_obj
            .filename_string
            .push_str(&format!("/{}", word));
    }
    writeln!(&mut stream, "{:?}", status_line_obj).unwrap();
}
