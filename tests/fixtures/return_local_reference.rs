fn make_label() -> &'static str {
    let local = String::from("local");
    &local
}

fn main() {
    assert_eq!(make_label(), "local");
}
