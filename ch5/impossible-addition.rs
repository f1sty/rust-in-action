#[allow(arithmetic_overflow)]

pub fn main() {
    let (a, b) = (200, 200);
    let c: u8 = a + b;

    println!("{a} + {b} = {c}");
}
