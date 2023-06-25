pub fn main() {
    let needle = 42;
    let heystack = [1, 1, 2, 5, 14, 42, 132, 429, 1430, 4862];

    for item in &heystack {
        let result = match item {
            42 | 132 => "hit!",
            _ => "miss",
        };

        if result == "hit!" {
            println!("{item}: {result}");
        }
    }
}
