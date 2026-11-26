#![forbid(unsafe_code)]

//! Executable models for the mathematical foundations chapters.

pub mod cost;
pub mod counting;
pub mod induction;
pub mod relations;

// ANCHOR: bounded_sum
/// Add two bytes when their ordinary sum fits.
pub fn bounded_sum(a: u8, b: u8) -> Option<u8> {
    if a <= u8::MAX - b {
        Some(a + b)
    } else {
        None
    }
}
// ANCHOR_END: bounded_sum

/// Apply an application-level maximum to a byte sum.
pub fn sum_with_limit(a: u8, b: u8, limit: u8) -> Option<u8> {
    if b <= limit && a <= limit - b {
        Some(a + b)
    } else {
        None
    }
}

pub fn first_above(totals: &[u8], limit: u8) -> Option<usize> {
    totals.iter().position(|&total| total > limit)
}

/// A fixed-width row type ensures a shared set of monitor columns.
pub fn has_shared_input<const MONITORS: usize>(rows: &[[bool; MONITORS]]) -> bool {
    rows.iter().any(|row| row.iter().all(|&passed| passed))
}

pub fn first_inconsistent_record(records: &[(bool, bool)]) -> Option<usize> {
    records
        .iter()
        .position(|&(accepted, matches)| accepted && !matches)
}
