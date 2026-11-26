//! Finite models of address translation and interpretation from Chapter 5.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Page {
    pub frame: u8,
    pub writable: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Access {
    Read,
    Write,
}

#[derive(Debug, PartialEq, Eq)]
pub enum TranslationError {
    Unmapped,
    Permission,
}

/// Translate an 8-bit virtual address using 16-byte pages.
/// Each mapped page permits reads and can optionally permit writes.
pub fn translate(
    map: &[Option<Page>; 16],
    virtual_address: u8,
    access: Access,
) -> Result<u16, TranslationError> {
    let page_number = usize::from(virtual_address / 16);
    let offset = u16::from(virtual_address % 16);
    let page = map[page_number].ok_or(TranslationError::Unmapped)?;
    if access == Access::Write && !page.writable {
        return Err(TranslationError::Permission);
    }
    Ok(u16::from(page.frame) * 16 + offset)
}

pub fn even_parity(bits: &[bool]) -> bool {
    let mut even = true;
    for &bit in bits {
        even ^= bit;
    }
    even
}

#[derive(Debug, PartialEq, Eq)]
pub enum NestingError {
    DepthLimit(usize),
    UnmatchedClosing(usize),
    InvalidSymbol(usize),
    Unfinished(usize),
}

/// Recognize bounded nesting. Positional errors carry a byte index.
/// An unfinished input carries its number of unmatched openings.
pub fn bounded_parentheses(input: &[u8], limit: usize) -> Result<(), NestingError> {
    let mut depth = 0;
    for (index, &byte) in input.iter().enumerate() {
        match byte {
            b'(' if depth == limit => return Err(NestingError::DepthLimit(index)),
            b'(' => depth += 1,
            b')' if depth == 0 => return Err(NestingError::UnmatchedClosing(index)),
            b')' => depth -= 1,
            _ => return Err(NestingError::InvalidSymbol(index)),
        }
    }
    if depth == 0 {
        Ok(())
    } else {
        Err(NestingError::Unfinished(depth))
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum InterpretError {
    MissingHalt,
    InvalidOpcode { index: usize, byte: u8 },
}

pub fn interpret_toggle(program: &[u8]) -> Result<u8, InterpretError> {
    let mut value = 0;
    for (index, &byte) in program.iter().enumerate() {
        match byte {
            0 => {}
            1 => value ^= 1,
            2 => return Ok(value),
            _ => return Err(InterpretError::InvalidOpcode { index, byte }),
        }
    }
    Err(InterpretError::MissingHalt)
}
