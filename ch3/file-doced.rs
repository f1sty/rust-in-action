//! Simulating files one step at a time.

/// Represemnts a "file",
/// which probably lives on a file system.
#[derive(Debug)]
pub struct File {
    name: String,
    data: Vec<u8>,
}

impl File {
    pub fn new(name: &str) -> Self {
        Self { name: String::from(name), data: Vec::new() }
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }
}

pub fn main() {
    let file = File::new("file.txt");
    let file_name = file.name();
    let file_length = file.len();

    println!("{file:?}");
    println!("{file_name} is {file_length} bytes long");
}
