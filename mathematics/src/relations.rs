//! Operations on finite sets, mappings, and graphs from Chapter 7.

pub fn remaining_jobs(requested: u8, cached: u8) -> u8 {
    requested & !cached
}

pub fn share_input(left: &[u8], right: &[u8]) -> bool {
    left.iter().any(|input| right.contains(input))
}

// ANCHOR: two_steps
/// Check for a walk of exactly two edges in a square adjacency matrix.
/// An out-of-domain vertex yields absence.
pub fn two_steps<const N: usize>(edges: &[[bool; N]; N], from: usize, to: usize) -> Option<bool> {
    let start = edges.get(from)?;
    if to >= N {
        return None;
    }
    Some((0..N).any(|middle| start[middle] && edges[middle][to]))
}
// ANCHOR_END: two_steps

pub fn first_collision(slots: &[u8]) -> Option<(usize, usize)> {
    for (first, &slot) in slots.iter().enumerate() {
        if let Some(offset) = slots[first + 1..].iter().position(|&other| other == slot) {
            return Some((first, first + 1 + offset));
        }
    }
    None
}

pub fn invert_assignment(slots: [usize; 3]) -> Option<[usize; 3]> {
    let mut inverse = [None; 3];
    for (job, slot) in slots.into_iter().enumerate() {
        let entry = inverse.get_mut(slot)?;
        if entry.is_some() {
            return None;
        }
        *entry = Some(job);
    }
    let [first, second, third] = inverse;
    Some([first?, second?, third?])
}
