fn greet_world() {
    println!("Hello, world!");
    let germany = "Grüß Gott!";
    let ukraine = "Привіт, світе!";
    let regions = [germany, ukraine];

    for region in regions.iter() {
        println!("{}", &region);
    }
}

fn main() {
    greet_world();
}
