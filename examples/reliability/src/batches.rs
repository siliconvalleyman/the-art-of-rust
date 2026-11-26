use crate::{
    row_errors::{parse_row, RowError},
    stock::{InsufficientStock, Stock},
};

pub const MAX_BYTES: usize = 64;
pub const MAX_LINES: usize = 4;

#[derive(Debug)]
pub enum BatchError {
    TooManyBytes,
    TooManyLines,
    Quantity(RowError),
    Stock {
        line: usize,
        reason: InsufficientStock,
    },
}

pub fn apply_reservations(stock: &mut Stock, text: &str) -> Result<(), BatchError> {
    if text.len() > MAX_BYTES {
        return Err(BatchError::TooManyBytes);
    }

    let mut draft = *stock;
    for (index, field) in text.lines().enumerate() {
        if index == MAX_LINES {
            return Err(BatchError::TooManyLines);
        }
        let line = index + 1;
        let quantity = parse_row(line, field).map_err(BatchError::Quantity)?;
        draft
            .reserve(quantity)
            .map_err(|reason| BatchError::Stock { line, reason })?;
    }

    *stock = draft;
    Ok(())
}
