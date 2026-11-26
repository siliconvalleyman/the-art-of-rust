#[derive(Debug, PartialEq, Eq)]
pub enum SumError {
    InvalidField(usize),
    Overflow(usize),
}

pub fn positive_count(values: &[i16]) -> usize {
    values.iter().copied().filter(|&value| value > 0).count()
}

pub fn greatest(values: &[u8]) -> Option<u8> {
    values.iter().copied().reduce(u8::max)
}

pub fn sum_fields(fields: &[&str]) -> Result<u32, SumError> {
    fields
        .iter()
        .enumerate()
        .map(|(index, field)| {
            field
                .parse::<u32>()
                .map(|value| (index, value))
                .map_err(|_| SumError::InvalidField(index))
        })
        .try_fold(0_u32, |total, field| {
            let (index, value) = field?;
            total.checked_add(value).ok_or(SumError::Overflow(index))
        })
}
