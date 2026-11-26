use mathematics::cost::{copies_with_doubling, counted_first_match, search_expectation};

#[test]
fn comparison_count_uses_the_first_matching_index() {
    assert_eq!(counted_first_match(&[5, 0, 5], 5), (Some(0), 1));
    assert_eq!(counted_first_match(&[5, 0, 5], 0), (Some(1), 2));
    assert_eq!(counted_first_match(&[5, 0, 5], 4), (None, 3));
    assert_eq!(counted_first_match(&[], 5), (None, 0));
}

#[test]
fn expectation_covers_boundary_workloads_and_wide_arithmetic() {
    assert_eq!(search_expectation(7, 1, 4), Some((38, 8)));
    assert_eq!(search_expectation(7, 0, 1), Some((8, 2)));
    assert_eq!(search_expectation(7, 1, 1), Some((14, 2)));
    assert_eq!(search_expectation(0, 0, 1), None);
    assert_eq!(search_expectation(7, 0, 0), None);
    assert_eq!(search_expectation(7, 2, 1), None);

    let max = u128::from(u32::MAX);
    assert_eq!(
        search_expectation(u32::MAX, u32::MAX, u32::MAX),
        Some((2 * max * max, 2 * max))
    );
}

#[test]
fn copy_count_matches_single_insertion_simulation() {
    let mut capacity = 0;
    let mut copies = 0;
    for length in 0..=512 {
        assert_eq!(copies_with_doubling(length), copies);
        if u64::from(length) == capacity {
            copies += capacity;
            capacity = (capacity * 2).max(1);
        }
    }
    assert_eq!(copies_with_doubling(9), 15);
    assert_eq!(copies_with_doubling(u32::MAX), u64::from(u32::MAX));
}
