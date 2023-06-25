use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

pub fn main() {
    let f = File::open("Cargo.toml").unwrap();
    let mut reader = BufReader::new(f);
    let mut line = String::new();

    loop {
        let len = reader.read_line(&mut line).unwrap();
        if len == 0 {
            break;
        }

        println!("{line} ({len} bytes long)");

        line.truncate(0);
    }
}
