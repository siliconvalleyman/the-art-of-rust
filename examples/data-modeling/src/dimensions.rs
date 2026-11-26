/// The two independent dimensions of a rectangular label.
#[derive(Debug, PartialEq, Eq)]
pub struct Dimensions {
    width_mm: u16,
    height_mm: u16,
}

impl Dimensions {
    pub fn new(width_mm: u16, height_mm: u16) -> Self {
        Self {
            width_mm,
            height_mm,
        }
    }

    pub fn area_mm2(&self) -> u32 {
        u32::from(self.width_mm) * u32::from(self.height_mm)
    }

    pub fn rotate(&mut self) {
        std::mem::swap(&mut self.width_mm, &mut self.height_mm);
    }

    pub fn into_pair(self) -> (u16, u16) {
        (self.width_mm, self.height_mm)
    }
}
