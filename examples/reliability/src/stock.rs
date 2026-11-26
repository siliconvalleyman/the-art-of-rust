#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InsufficientStock {
    pub requested: u8,
    pub available: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Stock {
    total: u8,
    available: u8,
}

impl Stock {
    pub fn new(total: u8) -> Self {
        Self {
            total,
            available: total,
        }
    }

    pub fn available(&self) -> u8 {
        self.available
    }

    pub fn held(&self) -> u8 {
        debug_assert!(self.available <= self.total);
        self.total - self.available
    }

    pub fn reserve(&mut self, requested: u8) -> Result<(), InsufficientStock> {
        let available = self
            .available
            .checked_sub(requested)
            .ok_or(InsufficientStock {
                requested,
                available: self.available,
            })?;
        self.available = available;
        Ok(())
    }
}
