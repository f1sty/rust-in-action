use std::collections::HashMap;

pub fn main() {
    let mut capitals = HashMap::new();

    capitals.insert("Cook Island", "Avarua");
    capitals.insert("Fiji", "Suve");
    capitals.insert("Kiribati", "South Tarava");
    capitals.insert("Niue", "Alofi");
    capitals.insert("Tonga", "Nuku'alofa");
    capitals.insert("Tuvalu", "Finafuti");

    let tonga_capital = capitals["Tonga"];

    println!("Capital of Tonga is: {tonga_capital}");
}
