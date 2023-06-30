use std::error::Error;
use std::fs::File;
use std::net::Ipv6Addr;

pub fn main() -> Result<(), Box<dyn Error>> {
    let _file = File::open("nonexist.file")?;
    let _addr = "::1".parse::<Ipv6Addr>()?;

    Ok(())
}
