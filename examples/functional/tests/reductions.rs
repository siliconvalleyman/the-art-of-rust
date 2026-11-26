use functional::reductions::{greatest, positive_count, sum_fields, SumError};

#[test]
fn reductions_distinguish_absence_from_an_identity_value() {
    assert_eq!(positive_count(&[-4, 0, 2, 3]), 2);
    assert_eq!(greatest(&[]), None);
    assert_eq!(greatest(&[0]), Some(0));
    assert_eq!(greatest(&[2, 0, 9, 9]), Some(9));
    assert_eq!(sum_fields(&[]), Ok(0));
}

#[test]
fn the_first_failing_field_determines_the_error() {
    assert_eq!(sum_fields(&["2", "3"]), Ok(5));
    assert_eq!(sum_fields(&["4294967295", "0"]), Ok(u32::MAX));
    assert_eq!(
        sum_fields(&["4294967295", "1", "bad"]),
        Err(SumError::Overflow(1))
    );
    assert_eq!(
        sum_fields(&["2", "bad", "4294967295"]),
        Err(SumError::InvalidField(1))
    );
}
