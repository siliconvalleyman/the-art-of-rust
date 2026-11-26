use std::panic::catch_unwind;

fn main() {
    let mut count = 0;
    let _ = catch_unwind(|| count += 1);
}
