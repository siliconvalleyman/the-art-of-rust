use std::num::NonZeroU64;

#[derive(Debug, PartialEq, Eq)]
pub struct Dispatch<State> {
    id: u32,
    state: State,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Draft;

#[derive(Debug, PartialEq, Eq)]
pub struct Ready {
    parcels: u16,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Shipped {
    parcels: u16,
    tracking: NonZeroU64,
}

#[derive(Debug, PartialEq, Eq)]
pub enum PrepareError {
    Empty,
    TooMany,
}

impl<State> Dispatch<State> {
    pub fn id(&self) -> u32 {
        self.id
    }
}

impl Dispatch<Draft> {
    pub fn new(id: u32) -> Self {
        Self { id, state: Draft }
    }

    pub fn prepare(self, parcels: u16) -> Result<Dispatch<Ready>, (Self, PrepareError)> {
        match parcels {
            0 => Err((self, PrepareError::Empty)),
            1..=500 => Ok(Dispatch {
                id: self.id,
                state: Ready { parcels },
            }),
            _ => Err((self, PrepareError::TooMany)),
        }
    }
}

impl Dispatch<Ready> {
    pub fn record_shipment(self, tracking: NonZeroU64) -> Dispatch<Shipped> {
        Dispatch {
            id: self.id,
            state: Shipped {
                parcels: self.state.parcels,
                tracking,
            },
        }
    }
}

impl Dispatch<Shipped> {
    pub fn receipt(&self) -> (u32, u16, NonZeroU64) {
        (self.id, self.state.parcels, self.state.tracking)
    }
}
