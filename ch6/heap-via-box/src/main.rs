use std::mem::drop;

fn main() {
    let a = Box::new(1);
    let b = Box::new(1);
    let c = Box::new(1);

    let result1 = *a + *b + *c;

    drop(a);

    let d = Box::new(1);
    let result2 = *d + *b + *c;

    println!("{result1} {result2}");
}
