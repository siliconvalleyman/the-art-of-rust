use std::iter::FusedIterator;

/// Yield a starting value through one in descending order.
pub struct Countdown {
    remaining: u8,
}

impl Countdown {
    pub fn new(start: u8) -> Self {
        Self { remaining: start }
    }
}

impl Iterator for Countdown {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining == 0 {
            return None;
        }
        let value = self.remaining;
        self.remaining -= 1;
        Some(value)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let count = usize::from(self.remaining);
        (count, Some(count))
    }
}

impl FusedIterator for Countdown {}
