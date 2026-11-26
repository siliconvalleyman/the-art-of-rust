use std::fmt::{self, Display, Formatter};

impl Display for Vec<u8> {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.len())
    }
}

fn main() {}
