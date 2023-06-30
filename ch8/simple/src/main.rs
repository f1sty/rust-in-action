use reqwest;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let url = "https://docs.rs/reqwest/latest/reqwest/blocking/fn.get.html";
    let content = reqwest::blocking::get(url)?.text()?;

    print!("{content}");

    Ok(())
}
