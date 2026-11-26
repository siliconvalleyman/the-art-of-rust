/// Validation starts with the byte limit, then checks syntax and range.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScanError {
    TooLong,
    NotDecimal,
    OutOfRange,
}

/// Parse a supplied scanner field containing one to three ASCII decimal digits.
pub fn parse_count(field: Option<&str>) -> Result<Option<u8>, ScanError> {
    let Some(field) = field else {
        return Ok(None);
    };
    if field.len() > 3 {
        return Err(ScanError::TooLong);
    }
    if field.is_empty() || !field.bytes().all(|byte| byte.is_ascii_digit()) {
        return Err(ScanError::NotDecimal);
    }

    let value = field
        .bytes()
        .fold(0_u16, |value, digit| value * 10 + u16::from(digit - b'0'));
    u8::try_from(value)
        .map(Some)
        .map_err(|_| ScanError::OutOfRange)
}
