use std::{ffi::OsStr, fmt};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CalcError {
    Operand,
    Operator,
    DivisionByZero,
    Overflow,
}

impl fmt::Display for CalcError {
    fn fmt(&self, out: &mut fmt::Formatter<'_>) -> fmt::Result {
        out.write_str(match self {
            Self::Operand => "expected a signed 64-bit decimal operand",
            Self::Operator => "expected one of + - '*' / %",
            Self::DivisionByZero => "divisor must be nonzero",
            Self::Overflow => "arithmetic overflow",
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Remainder,
}

impl Operator {
    pub fn parse(text: &OsStr) -> Result<Self, CalcError> {
        match text.to_str() {
            Some("+") => Ok(Self::Add),
            Some("-") => Ok(Self::Subtract),
            Some("*") => Ok(Self::Multiply),
            Some("/") => Ok(Self::Divide),
            Some("%") => Ok(Self::Remainder),
            _ => Err(CalcError::Operator),
        }
    }

    pub fn evaluate(self, left: i64, right: i64) -> Result<i64, CalcError> {
        if matches!(self, Self::Divide | Self::Remainder) && right == 0 {
            return Err(CalcError::DivisionByZero);
        }
        let result = match self {
            Self::Add => left.checked_add(right),
            Self::Subtract => left.checked_sub(right),
            Self::Multiply => left.checked_mul(right),
            Self::Divide => left.checked_div(right),
            Self::Remainder => left.checked_rem(right),
        };
        result.ok_or(CalcError::Overflow)
    }
}

fn parse_operand(text: &OsStr) -> Result<i64, CalcError> {
    text.to_str()
        .ok_or(CalcError::Operand)?
        .parse()
        .map_err(|_| CalcError::Operand)
}

pub fn calculate(left: &OsStr, operator: &OsStr, right: &OsStr) -> Result<i64, CalcError> {
    let left = parse_operand(left)?;
    let operator = Operator::parse(operator)?;
    let right = parse_operand(right)?;
    operator.evaluate(left, right)
}
