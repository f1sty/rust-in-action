#[derive(Debug)]
struct File {
    name: String,
    data: Vec<u8>,
}

impl File  {
    pub fn new(name: &str) -> Self {
        Self { name: String::from(name), data: Vec::new(), }
    }
}

pub fn main() {
    let file = File::new("f3.txt");
    let file_name = &file.name;
    let file_length = file.data.len();

    println!("{file:?}");
    println!("{file_name} is {file_length} bytes long");
}
