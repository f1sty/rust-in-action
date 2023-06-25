fn main() {
    let things = vec!["⏱", "📔", "🚘"];
    let buffer_overflow = things[4];

    assert_eq!(buffer_overflow, "🍋");
}
