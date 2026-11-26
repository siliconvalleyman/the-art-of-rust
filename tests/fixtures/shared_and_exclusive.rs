fn main() {
    let mut value = 4;
    let view = &value;
    let target = &mut value;
    *target = 5;
    assert_eq!(*view, 4);
}
