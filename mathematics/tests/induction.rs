use mathematics::induction::{balanced_visits, decimal_digits, postage, sum_first};

#[test]
fn widened_sum_covers_the_entire_input_range() {
    assert_eq!(sum_first(0), 0);
    assert_eq!(sum_first(1), 1);
    assert_eq!(sum_first(100), 5050);
    assert_eq!(sum_first(u32::MAX), 9_223_372_034_707_292_160);
}

#[test]
fn postage_handles_base_cases_and_reconstructs_large_amounts() {
    for amount in (0..=100).chain([u32::MAX]) {
        match postage(amount) {
            Some((threes, fives)) => assert_eq!(3 * threes + 5 * fives, amount),
            None => assert!([1, 2, 4, 7].contains(&amount)),
        }
    }
    assert_eq!(postage(0), Some((0, 0)));
    assert_eq!(postage(4), None);
    assert_eq!(postage(7), None);
    assert_eq!(postage(9), Some((3, 0)));
    assert_eq!(postage(20), Some((5, 1)));
}

#[test]
fn recurrence_prediction_agrees_with_recursive_model() {
    fn oracle(n: u32) -> u64 {
        if n < 2 {
            return 0;
        }
        let left = n / 2;
        u64::from(n) + oracle(left) + oracle(n - left)
    }

    for n in 0..=128 {
        assert_eq!(balanced_visits(n), oracle(n));
    }
    assert_eq!(balanced_visits(7), 20);
    assert_eq!(balanced_visits(8), 24);
    assert_eq!(balanced_visits(u32::MAX), 32 * u64::from(u32::MAX) - 1);
}

#[test]
fn digit_count_changes_at_decimal_boundaries() {
    for (n, expected) in [(0, 1), (9, 1), (10, 2), (99, 2), (100, 3), (u32::MAX, 10)] {
        assert_eq!(decimal_digits(n), expected);
    }
}
