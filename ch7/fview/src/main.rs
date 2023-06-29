use std::env;
use std::fs::File;
use std::io::prelude::*;

const BYTES_PER_LINE: usize = 16;

fn main() {
    let filename = env::args().nth(1).expect("usage: fview FILENAME");
    let mut file = File::open(&filename).expect("unable to open file");
    let mut position = 0;
    let mut buffer = [0; BYTES_PER_LINE];

    while let Ok(_) = file.read_exact(&mut buffer) {
        print!("[0x{:08x}] ", position);
        for byte in &buffer {
            match *byte {
                0x00 => print!(".  "),
                0xff => print!("## "),
                _ => print!("{:02x} ", byte),
            }
        }
        println!();
        position += BYTES_PER_LINE;
    }
}
