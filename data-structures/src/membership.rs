#[derive(Debug, Default)]
pub struct Bloom64 {
    bits: u64,
}

impl Bloom64 {
    fn mask(fingerprints: [u64; 2]) -> u64 {
        let [first, second] = fingerprints;
        (1_u64 << (first % 64)) | (1_u64 << (second % 64))
    }

    pub fn insert(&mut self, fingerprints: [u64; 2]) {
        self.bits |= Self::mask(fingerprints);
    }

    pub fn might_contain(&self, fingerprints: [u64; 2]) -> bool {
        let mask = Self::mask(fingerprints);
        self.bits & mask == mask
    }
}
