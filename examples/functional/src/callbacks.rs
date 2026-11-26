pub fn sample_two<F: Fn(u8) -> u8>(at: F) -> [u8; 2] {
    [at(0), at(1)]
}

pub fn visit_all<F: FnMut(u8)>(input: &[u8], mut visit: F) {
    for &value in input {
        visit(value);
    }
}

pub fn make_once<F: FnOnce() -> String>(make: F) -> String {
    make()
}
