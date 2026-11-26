//! Counting and residue arithmetic from Chapter 9.

pub fn unordered_pairs(n: u32) -> u64 {
    let n = u64::from(n);
    n * n.saturating_sub(1) / 2
}

pub fn successful_sample_pairs(good: u16, faulty: u16) -> Option<(u64, u64)> {
    let total = u32::from(good) + u32::from(faulty);
    if total < 2 {
        return None;
    }
    Some((unordered_pairs(u32::from(good)), unordered_pairs(total)))
}

pub fn advance_index(index: u64, advance: u64, capacity: u64) -> Option<u64> {
    if index >= capacity {
        return None;
    }
    let step = advance % capacity;
    let remaining = capacity - index;
    if step < remaining {
        Some(index + step)
    } else {
        Some(step - remaining)
    }
}

// ANCHOR: gcd
/// Return the greatest common divisor, using zero for (0, 0).
pub fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let remainder = a % b;
        a = b;
        b = remainder;
    }
    a
}
// ANCHOR_END: gcd
