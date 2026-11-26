use abstractions::reading::{total_grams, BoundedReading, Reading, Volume, Weight};

#[test]
fn defaults_and_bounds_produce_the_specified_outcomes() {
    assert!(Weight(0).is_zero());
    assert!(!Volume(3).is_zero());
    assert_eq!(Weight::LABEL, "g");
    assert!(Weight(30_000).is_within_limit());
    assert!(!Weight(30_001).is_within_limit());
}

#[test]
fn constrained_sum_rejects_overflow_and_preserves_an_empty_identity() {
    assert_eq!(total_grams::<Weight>(&[]), Some(0));
    assert_eq!(total_grams(&[Weight(9), Weight(11)]), Some(20));
    assert_eq!(total_grams(&[Weight(u32::MAX), Weight(1)]), None);
}
