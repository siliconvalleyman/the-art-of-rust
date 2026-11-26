use std::{ffi::OsStr, fmt};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Unit {
    Millimeter,
    Centimeter,
    Meter,
    Kilometer,
}

impl Unit {
    pub fn parse(input: &OsStr) -> Result<Self, ConversionError> {
        match input.to_str() {
            Some("mm") => Ok(Self::Millimeter),
            Some("cm") => Ok(Self::Centimeter),
            Some("m") => Ok(Self::Meter),
            Some("km") => Ok(Self::Kilometer),
            _ => Err(ConversionError::Unit),
        }
    }

    fn millimeters(self) -> u128 {
        match self {
            Self::Millimeter => 1,
            Self::Centimeter => 10,
            Self::Meter => 1_000,
            Self::Kilometer => 1_000_000,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConversionError {
    Quantity,
    Unit,
    Precision,
}

impl fmt::Display for ConversionError {
    fn fmt(&self, out: &mut fmt::Formatter<'_>) -> fmt::Result {
        out.write_str(match self {
            Self::Quantity => "expected a nonnegative decimal with at most three fractional digits",
            Self::Unit => "expected one of mm, cm, m, km",
            Self::Precision => "expected decimal places from 0 to 9",
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Length(u128);

impl Length {
    pub fn parse(input: &OsStr, unit: Unit) -> Result<Self, ConversionError> {
        let text = input.to_str().ok_or(ConversionError::Quantity)?;
        let (whole, fraction) = match text.split_once('.') {
            Some((whole, fraction)) if (1..=3).contains(&fraction.len()) => (whole, fraction),
            Some(_) => return Err(ConversionError::Quantity),
            None => (text, ""),
        };
        if whole.is_empty()
            || !whole
                .bytes()
                .chain(fraction.bytes())
                .all(|byte| byte.is_ascii_digit())
        {
            return Err(ConversionError::Quantity);
        }
        let whole = whole
            .parse::<u64>()
            .map_err(|_| ConversionError::Quantity)?;
        let fraction = fraction
            .bytes()
            .zip([100, 10, 1])
            .map(|(digit, place)| u128::from(digit - b'0') * place)
            .sum::<u128>();
        Ok(Self(
            (u128::from(whole) * 1_000 + fraction) * unit.millimeters(),
        ))
    }

    pub fn render(self, unit: Unit, places: Option<u8>) -> Result<String, ConversionError> {
        let digits = places.unwrap_or(9);
        if digits > 9 {
            return Err(ConversionError::Precision);
        }
        let scale = 10_u128.pow(u32::from(digits));
        let divisor = unit.millimeters() * 1_000;
        let numerator = self.0 * scale;
        let quotient = numerator / divisor;
        let residue = numerator % divisor;
        let rounded = quotient + u128::from(2 * residue >= divisor);
        let whole = rounded / scale;
        if digits == 0 {
            return Ok(whole.to_string());
        }
        let fraction = rounded % scale;
        let mut text = format!("{whole}.{fraction:0width$}", width = usize::from(digits));
        if places.is_none() {
            let end = text.trim_end_matches('0').trim_end_matches('.').len();
            text.truncate(end);
        }
        Ok(text)
    }
}
