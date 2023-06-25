fn add_with_lifitimes<'a, 'b>(i: &'a i32, j: &'b i32) -> i32 {
    *i + *j
}

pub fn main() {
    let a = 10;
    let b = 20;
    let res = add_with_lifitimes(&a, &b);

    println!("{res}");
}
