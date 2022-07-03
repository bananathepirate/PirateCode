use std::{env::var, fs::File, io::Write, path::PathBuf};

fn main() {
    let chars = ('a'..='z').chain('A'..='Z').chain('0'..='9').chain([
        '!', '?', '"', '£', '$', '%', '&', '(', ')', '[', ']', '{', '}', '=', '#', '@', '^', '<',
        '>', '*', '+', '-', '_', ':', ',', ';', '|'
        ]);
    let len = chars.clone().count();
    let path = PathBuf::from(var("OUT_DIR").unwrap()).join("table.rs");
    let mut file = File::create(path).unwrap();
    write!(file, "const PIRATE_LANG: [char; {len}] = [").unwrap();
    for ch in chars {
        write!(file, "\'{ch}\', ").unwrap();
    }
    write!(file, "];").unwrap();
}
