/// An unvalidated whole-gram quantity.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Grams(u32);

impl Grams {
    pub fn new(value: u32) -> Self {
        Self(value)
    }

    pub fn value(self) -> u32 {
        self.0
    }
}

/// A whole-gram weight accepted by the example parcel service.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ParcelWeight(Grams);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WeightError {
    Zero,
    TooHeavy,
}

impl ParcelWeight {
    pub const MAX_GRAMS: u32 = 30_000;

    pub fn new(weight: Grams) -> Result<Self, WeightError> {
        match weight.value() {
            0 => Err(WeightError::Zero),
            value if value > Self::MAX_GRAMS => Err(WeightError::TooHeavy),
            _ => Ok(Self(weight)),
        }
    }

    pub fn grams(self) -> Grams {
        self.0
    }

    pub fn with_packaging(self, packaging: Grams) -> Result<Self, WeightError> {
        let total = self
            .grams()
            .value()
            .checked_add(packaging.value())
            .ok_or(WeightError::TooHeavy)?;
        Self::new(Grams::new(total))
    }
}
