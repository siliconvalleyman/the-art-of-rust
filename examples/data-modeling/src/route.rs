/// A parcel has exactly one route at a time.
#[derive(Debug, PartialEq, Eq)]
pub enum Route {
    Collect,
    Locker(u8),
    Deliver { zone: u8, fragile: bool },
}

#[derive(Debug, PartialEq, Eq)]
pub enum Desk {
    Counter,
    Locker(u8),
    Standard(u8),
    Padded(u8),
}

impl Route {
    pub fn desk(&self) -> Desk {
        match self {
            Self::Collect => Desk::Counter,
            Self::Locker(number) => Desk::Locker(*number),
            Self::Deliver {
                zone,
                fragile: true,
            } => Desk::Padded(*zone),
            Self::Deliver {
                zone,
                fragile: false,
            } => Desk::Standard(*zone),
        }
    }
}
