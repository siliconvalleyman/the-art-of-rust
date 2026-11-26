//! Cost models from Chapter 10.

// ANCHOR: counted_first_match
/// Return the first matching position and the count of comparisons.
pub fn counted_first_match(values: &[u8], target: u8) -> (Option<usize>, usize) {
    for (index, &value) in values.iter().enumerate() {
        if value == target {
            return (Some(index), index + 1);
        }
    }
    (None, values.len())
}
// ANCHOR_END: counted_first_match

/// Expectation under a uniform unique hit and miss probability miss/total.
/// Returns an exact numerator and denominator without requiring reduction.
pub fn search_expectation(n: u32, miss: u32, total: u32) -> Option<(u128, u128)> {
    if n == 0 || total == 0 || miss > total {
        return None;
    }
    let n = u128::from(n);
    let miss = u128::from(miss);
    let total = u128::from(total);
    Some(((total - miss) * (n + 1) + 2 * miss * n, 2 * total))
}

pub fn copies_with_doubling(n: u32) -> u64 {
    let mut capacity = 1;
    let mut copies = 0;
    while capacity < u64::from(n) {
        copies += capacity;
        capacity *= 2;
    }
    copies
}
