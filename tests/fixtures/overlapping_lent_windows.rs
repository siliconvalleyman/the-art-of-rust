#[path = "../../examples/abstractions/src/lending.rs"]
mod lending;

use lending::{ByteWindows, Lending};
use std::num::NonZeroUsize;

fn main() {
    let mut bytes = [1, 2, 3];
    let mut windows = ByteWindows::new(&mut bytes, NonZeroUsize::new(2).unwrap());
    let first = windows.next().unwrap();
    let second = windows.next().unwrap();
    first[1] = 7;
    second[0] = 8;
}
