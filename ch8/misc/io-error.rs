use std::fs::File;

pub fn main() -> Result<(), std::io::Error> {
    let _file = File::open("nonexist.file")?;

    Ok(())
}
