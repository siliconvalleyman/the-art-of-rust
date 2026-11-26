//! Instruction encodings and the two-slot cache from Chapter 4.

/// Encode an RV32I ADD with register numbers in 0 through 31.
pub fn encode_add(dest: u8, src1: u8, src2: u8) -> Option<u32> {
    if dest > 31 || src1 > 31 || src2 > 31 {
        return None;
    }
    Some((u32::from(src2) << 20) | (u32::from(src1) << 15) | (u32::from(dest) << 7) | 0x33)
}

/// Emit one hit decision per address, starting from a cold cache.
pub fn hits(addresses: impl IntoIterator<Item = u64>) -> impl Iterator<Item = bool> {
    let mut slots = [None; 2];
    addresses.into_iter().map(move |address| {
        let line = address / 4;
        let slot = (line % 2) as usize;
        let hit = slots[slot] == Some(line);
        slots[slot] = Some(line);
        hit
    })
}

pub fn sequential_misses(length: u64) -> u64 {
    length / 4 + u64::from(length % 4 != 0)
}
