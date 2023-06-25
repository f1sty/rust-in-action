#[derive(Debug)]
enum Cereal {
    Barley,
    Millet,
    Rice,
    Rye,
    Spelt,
    Wheat,
}

fn main() {
    let grains: Vec<Cereal> = vec![];
    drop(grains);

    println!("{grains:?}");
}
