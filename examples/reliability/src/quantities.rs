use std::{io, num::ParseIntError};

#[derive(Debug)]
pub enum LoadQuantityError {
    Read(io::Error),
    Parse(ParseIntError),
}

impl From<io::Error> for LoadQuantityError {
    fn from(error: io::Error) -> Self {
        Self::Read(error)
    }
}

impl From<ParseIntError> for LoadQuantityError {
    fn from(error: ParseIntError) -> Self {
        Self::Parse(error)
    }
}

pub fn load_quantity(mut input: impl io::Read) -> Result<u16, LoadQuantityError> {
    let mut text = String::new();
    input.read_to_string(&mut text)?;
    Ok(text.trim().parse()?)
}
