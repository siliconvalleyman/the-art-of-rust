fn main() {
    let mut values = [1_u8, 2];
    let left = &mut values[0];
    let right = &mut values[1];
    *left = 10;
    *right = 20;
}
