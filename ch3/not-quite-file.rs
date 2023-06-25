#[allow(unused_variables)]

type File = String;

fn open(file: &mut File) -> bool {
    true
}

fn close(file: &mut File) -> bool {
    true
}

#[allow(dead_code)]
fn read(file: &mut File, save_to: &mut Vec<u8>) -> ! {
    unimplemented!()
}

pub fn main() {
    let mut file = File::from("file.txt");
    open(&mut file);
    //read(&mut file);
    close(&mut file);
}
