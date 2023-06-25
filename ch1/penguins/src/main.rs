fn main() {
    let penguins = "\
        common name,length (cm)
        Little penguin,33
        Yellow-eyed penguin,65
        Fiordland penguin,60
        Invalid,data
        ";

    let records = penguins.lines();

    for (i, record) in records.enumerate() {
        if i == 0 || record.trim().len() == 0 {
            continue;
        }

        let fields: Vec<_> = record.split(",").map(|field| field.trim()).collect();

        if cfg!(debug_assertions) {
            dbg!(&record, &fields);
        }

        let name = fields[0];

        if let Ok(length) = fields[1].parse::<u32>() {
            println!("{name}, {length}cm");
        }
    }
}
