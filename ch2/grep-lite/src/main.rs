use clap::{Arg, ArgAction, Command};
use regex::Regex;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn process_lines<T: BufRead + Sized>(reader: T, re: Regex) {
    for l in reader.lines() {
        let line = l.unwrap();
        match re.find(&line) {
            Some(_) => println!("{line}"),
            None => (),
        }
    }
}

pub fn main() {
    let args = Command::new("grep-lite")
        .version("0.1")
        .about("searches for patterns")
        .arg(
            Arg::new("pattern")
                .help("The pattern to search for")
                .action(ArgAction::Set)
                .required(true),
        )
        .arg(
            Arg::new("input")
                .help("File to traverse")
                .action(ArgAction::Set)
                .required(false)
                .default_value("-"),
        )
        .get_matches();
    if let Some(pattern) = args.get_one::<String>("pattern") {
        if let Some(input) = args.get_one::<String>("input") {
            let re = Regex::new(pattern).unwrap();
            if input == "-" {
                let stdin = std::io::stdin();
                let reader = stdin.lock();
                process_lines(reader, re);
            } else {
                let file = File::open(input).unwrap();
                let reader = BufReader::new(file);
                process_lines(reader, re);
            }
        }
    }
}
