fn select(left: &str, _right: &str) -> &str {
    left
}

fn main() {
    assert_eq!(select("left", "right"), "left");
}
