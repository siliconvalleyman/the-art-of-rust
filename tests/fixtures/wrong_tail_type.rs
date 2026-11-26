fn total() -> u16 {
    let a = 7_u16;
    a + 6;
}

fn main() {
    assert_eq!(total(), 13);
}
