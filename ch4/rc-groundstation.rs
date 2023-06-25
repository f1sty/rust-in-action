use std::rc::Rc;

#[derive(Debug)]
struct GroundStation { }

pub fn main() {
    let base = Rc::new(GroundStation {});

    println!("{:?}", base);
}
