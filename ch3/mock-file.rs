#[derive(Debug)]
struct File {
    name: String,
    data: Vec<u8>,
}

pub fn main() {
    let file = File {
        name: String::from("file.txt"),
        data: Vec::new(),
    };
    let file_name = &file.name;
    let file_lenght = &file.data.len();

    println!("{file:?}");
    println!("{file_name} is {file_lenght} bytes long");
}
