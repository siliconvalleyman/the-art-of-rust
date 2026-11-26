use std::mem::{align_of, size_of};
use std::num::NonZeroUsize;

use abstractions::lending::{inspect_two, ByteWindows, Lending, Quantity};

#[test]
fn marker_has_no_payload_and_short_borrows_can_be_observed_independently() {
    struct Grams;
    assert_eq!(size_of::<Quantity<Grams>>(), size_of::<u32>());
    assert_eq!(align_of::<Quantity<Grams>>(), align_of::<u32>());
    assert_eq!(Quantity::<Grams>::new(21).amount(), 21);
    assert_eq!(inspect_two(str::len), [4, 5]);
}

#[test]
fn each_window_progresses_once_and_prior_mutation_remains_visible() {
    let mut data = [1, 2, 3];
    let mut windows = ByteWindows::new(&mut data, NonZeroUsize::new(2).unwrap());
    windows.next().unwrap()[1] = 7;
    assert_eq!(windows.next().unwrap(), [7, 3]);
    assert!(windows.next().is_none());
    assert!(windows.next().is_none());
    assert_eq!(data, [1, 7, 3]);
}

#[test]
fn short_inputs_and_extreme_widths_do_not_panic() {
    assert!(ByteWindows::new(&mut [], NonZeroUsize::new(1).unwrap())
        .next()
        .is_none());
    let mut one = [4];
    let mut exact = ByteWindows::new(&mut one, NonZeroUsize::new(1).unwrap());
    assert_eq!(exact.next().unwrap(), [4]);
    assert!(exact.next().is_none());
    let mut beyond = ByteWindows::new(&mut one, NonZeroUsize::new(usize::MAX).unwrap());
    assert!(beyond.next().is_none());
}
