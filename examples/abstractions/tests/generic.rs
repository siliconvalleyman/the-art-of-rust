use abstractions::generic::{array_count, ends, Pair};

#[test]
fn endpoints_preserve_position_for_an_unconstrained_type() {
    struct Reading(u16);

    assert!(ends::<Reading>(&[]).is_none());
    let one = [Reading(2)];
    let (start, end) = ends(&one).unwrap();
    assert!(std::ptr::eq(start, end));
    let values = [Reading(3), Reading(8), Reading(5)];
    let (start, end) = ends(&values).unwrap();
    assert_eq!((start.0, end.0), (3, 5));
}

#[test]
fn reversing_transfers_heterogeneous_fields_and_accepts_zero_length_arrays() {
    let pair = Pair::new(String::from("reading"), 17_u8);
    assert_eq!(
        pair.reversed().reversed().into_parts(),
        (String::from("reading"), 17)
    );
    assert_eq!(array_count(&[false; 0]), 0);
    assert_eq!(array_count(&[4_u32; 3]), 3);
}
