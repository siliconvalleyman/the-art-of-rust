use data_modeling::scan::{parse_count, ScanError};

#[test]
fn absence_empty_zero_and_malformed_input_are_distinct() {
    assert_eq!(parse_count(None), Ok(None));
    assert_eq!(parse_count(Some("")), Err(ScanError::NotDecimal));
    assert_eq!(parse_count(Some("0")), Ok(Some(0)));
    assert_eq!(parse_count(Some("000")), Ok(Some(0)));
    assert_eq!(parse_count(Some("12x")), Err(ScanError::NotDecimal));
    assert_eq!(parse_count(Some("１２")), Err(ScanError::TooLong));
    assert_eq!(parse_count(Some("１２x")), Err(ScanError::TooLong));
    assert_eq!(parse_count(Some("é")), Err(ScanError::NotDecimal));
}

#[test]
fn every_three_digit_decimal_integer_has_the_specified_range_outcome() {
    for value in 0..=999_u16 {
        let text = format!("{value:03}");
        let expected = match u8::try_from(value) {
            Ok(value) => Ok(Some(value)),
            Err(_) => Err(ScanError::OutOfRange),
        };
        assert_eq!(parse_count(Some(&text)), expected, "{text}");
    }
}
