extern crate atty;
use atty::Stream;

extern crate syntect;
use syntect::dumps;
use syntect::easy::HighlightLines;
use syntect::highlighting::Style;
use syntect::parsing::SyntaxSet;
use syntect::util::{as_24_bit_terminal_escaped, LinesWithEndings};

fn main() {
    // Load these once at the start of your program
    let ps = SyntaxSet::load_defaults_newlines();
    let th = dumps::from_binary(include_bytes!("../Monokai.bin"));

    let syntax = ps.find_syntax_by_extension("json").unwrap();
    let mut h = HighlightLines::new(syntax, &th);
    let s = r#"{"a":"b", "c":true, "d":[1,2,3]}"#;
    for line in LinesWithEndings::from(s) {
        // LinesWithEndings enables use of newlines mode
        if atty::is(Stream::Stdout) {
            let ranges: Vec<(Style, &str)> = h.highlight(line, &ps);
            let escaped = as_24_bit_terminal_escaped(&ranges[..], false);
            println!("{}", escaped);
        } else {
            println!("{}", line);
        }
    }
}
