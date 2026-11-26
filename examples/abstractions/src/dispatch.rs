pub trait Adjust {
    fn apply(&self, value: u32) -> u32;
}

pub struct Offset(pub u32);
pub struct Scale(pub u32);

impl Adjust for Offset {
    fn apply(&self, value: u32) -> u32 {
        value.wrapping_add(self.0)
    }
}

impl Adjust for Scale {
    fn apply(&self, value: u32) -> u32 {
        value.wrapping_mul(self.0)
    }
}

pub fn via_generic<A: Adjust>(adjustment: &A, value: u32) -> u32 {
    adjustment.apply(value)
}

pub fn via_object(adjustment: &dyn Adjust, value: u32) -> u32 {
    adjustment.apply(value)
}

pub fn pipeline(adjustments: &[&dyn Adjust], input: u32) -> u32 {
    let mut value = input;
    for adjustment in adjustments {
        value = via_object(*adjustment, value);
    }
    value
}
