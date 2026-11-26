//! Bounded integer examples from Chapter 8.

pub fn sum_first(n: u32) -> u64 {
    let n = u64::from(n);
    n * (n + 1) / 2
}

/// Construct counts for denominations 3 and 5.
pub fn postage(amount: u32) -> Option<(u32, u32)> {
    match amount {
        0 => Some((0, 0)),
        3 => Some((1, 0)),
        5 => Some((0, 1)),
        6 => Some((2, 0)),
        1 | 2 | 4 | 7 => None,
        _ => {
            let extra = (amount - 8) / 3;
            match (amount - 8) % 3 {
                0 => Some((1 + extra, 1)),
                1 => Some((3 + extra, 0)),
                _ => Some((extra, 2)),
            }
        }
    }
}

// ANCHOR: balanced_visits
/// Count combine visits for the balanced splitting recurrence.
pub fn balanced_visits(n: u32) -> u64 {
    if n == 0 {
        return 0;
    }
    let levels = u32::BITS - 1 - n.leading_zeros();
    let n = u64::from(n);
    let widest_full_level = 1_u64 << levels;
    n * u64::from(levels) + 2 * (n - widest_full_level)
}
// ANCHOR_END: balanced_visits

pub fn decimal_digits(mut n: u32) -> u32 {
    if n == 0 {
        return 1;
    }
    let mut count = 0;
    while n != 0 {
        n /= 10;
        count += 1;
    }
    count
}
