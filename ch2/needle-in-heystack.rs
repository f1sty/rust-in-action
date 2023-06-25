pub fn main() {
    let needle = 0o204;
    let heystack = [1, 1, 2, 5, 15, 52, 203, 877, 4140, 21147];

    for item in &heystack {
        if *item == needle {
            println!("{item}");
        }
    }
}
