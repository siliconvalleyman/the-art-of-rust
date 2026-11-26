fn render<T>(value: T) -> String {
    format!("{value:?}")
}

fn main() {
    assert_eq!(render(4), "4");
}
