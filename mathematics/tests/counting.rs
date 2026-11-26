use mathematics::counting::{advance_index, gcd, successful_sample_pairs, unordered_pairs};

#[test]
fn pair_counts_match_small_enumerations_and_widen_before_multiplying() {
    for n in 0..=8 {
        let count: u64 = (0..n).map(|first| u64::from(n - first - 1)).sum();
        assert_eq!(unordered_pairs(n), count);
    }
    assert_eq!(unordered_pairs(u32::MAX), 9_223_372_030_412_324_865);
    assert_eq!(successful_sample_pairs(4, 2), Some((6, 15)));
    assert_eq!(successful_sample_pairs(0, 2), Some((0, 1)));
    assert_eq!(successful_sample_pairs(1, 0), None);
}

#[test]
fn ring_index_matches_widened_modulo_and_handles_bounds() {
    assert_eq!(advance_index(0, 0, 0), None);
    assert_eq!(advance_index(8, 0, 8), None);
    assert_eq!(advance_index(0, u64::MAX, 1), Some(0));
    assert_eq!(advance_index(6, 2, 8), Some(0));
    for capacity in [1, 8, u64::MAX] {
        for index in [0, capacity - 1] {
            for advance in [0, 1, 2, u64::MAX] {
                let widened = (u128::from(index) + u128::from(advance)) % u128::from(capacity);
                assert_eq!(
                    advance_index(index, advance, capacity),
                    Some(widened as u64)
                );
            }
        }
    }
}

#[test]
fn euclid_agrees_with_divisor_search() {
    for a in 0..=30 {
        for b in 0..=30 {
            let oracle = (1..=a.max(b)).rev().find(|d| a % d == 0 && b % d == 0);
            assert_eq!(gcd(a, b), oracle.unwrap_or(0));
        }
    }
    assert_eq!(gcd(84, 30), 6);
    assert_eq!(gcd(30, 84), 6);
    assert_eq!(gcd(u64::MAX, u64::MAX), u64::MAX);
}
