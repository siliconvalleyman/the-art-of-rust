use std::{error::Error, fmt, num::ParseIntError};

#[derive(Debug)]
pub struct RowError {
    line: usize,
    source: ParseIntError,
}

impl RowError {
    pub fn line(&self) -> usize {
        self.line
    }
}

impl fmt::Display for RowError {
    fn fmt(&self, out: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(out, "invalid quantity on line {}", self.line)
    }
}

impl Error for RowError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.source)
    }
}

pub fn parse_row(line: usize, field: &str) -> Result<u8, RowError> {
    field.parse().map_err(|source| RowError { line, source })
}
