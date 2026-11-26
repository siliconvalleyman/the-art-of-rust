pub struct Grams;
pub struct Milliliters;

pub trait Reading {
    type Unit;
    const LABEL: &'static str;

    fn value(&self) -> u32;

    fn is_zero(&self) -> bool {
        self.value() == 0
    }
}

pub trait BoundedReading: Reading {
    const MAX: u32;

    fn is_within_limit(&self) -> bool {
        self.value() <= Self::MAX
    }
}

pub struct Weight(pub u32);
pub struct Volume(pub u32);

impl Reading for Weight {
    type Unit = Grams;
    const LABEL: &'static str = "g";

    fn value(&self) -> u32 {
        self.0
    }
}

impl BoundedReading for Weight {
    const MAX: u32 = 30_000;
}

impl Reading for Volume {
    type Unit = Milliliters;
    const LABEL: &'static str = "mL";

    fn value(&self) -> u32 {
        self.0
    }
}

pub fn total_grams<R: Reading<Unit = Grams>>(readings: &[R]) -> Option<u32> {
    let mut total = 0_u32;
    for reading in readings {
        total = total.checked_add(reading.value())?;
    }
    Some(total)
}
