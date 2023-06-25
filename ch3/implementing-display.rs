#![allow(unused_variables)]

use std::fmt;
use std::fmt::Display;

#[derive(Debug,PartialEq)]
enum FileState {
    Open,
    Closed,
}

#[derive(Debug)]
struct File {
    name: String,
    data: Vec<u8>,
    state: FileState,
}

impl Display for FileState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            FileState::Open => write!(f, "OPEN"),
            FileState::Closed => write!(f, "CLOSED"),
        }
    }
}

impl Display for File {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<{} ({})>", self.name, self.state)
    }
}

impl File {
    pub fn new(name: &str) -> Self {
        Self { name: String::from(name), data: Vec::new(), state: FileState::Closed }
    }
}

pub fn main() {
    let file = File::new("6.txt");

    println!("{:?}", file);
    println!("{}", file);
}
