use mathematics::{
    bounded_sum, first_above, first_inconsistent_record, has_shared_input, sum_with_limit,
};

#[test]
fn guards_agree_with_the_widened_specification() {
    for a in 0..=u8::MAX {
        for b in 0..=u8::MAX {
            let mathematical_sum = u16::from(a) + u16::from(b);
            assert_eq!(bounded_sum(a, b), mathematical_sum.try_into().ok());
            for limit in [0, 1, 127, 254, 255] {
                let expected = if mathematical_sum <= u16::from(limit) {
                    Some(mathematical_sum as u8)
                } else {
                    None
                };
                assert_eq!(sum_with_limit(a, b, limit), expected);
            }
        }
    }
}

#[test]
fn witnesses_include_first_positions_but_exclude_invalid_records() {
    assert_eq!(first_above(&[], 5), None);
    assert_eq!(first_above(&[3, 2, 6, 9], 5), Some(2));
    assert_eq!(first_above(&[6, 6], 5), Some(0));
    assert_eq!(first_inconsistent_record(&[(false, false)]), None);
    assert_eq!(
        first_inconsistent_record(&[(true, true), (false, false), (true, false)]),
        Some(2)
    );
}

#[test]
fn shared_inputs_preserve_quantifier_order_and_empty_domains() {
    assert!(!has_shared_input(&[[true, false], [false, true]]));
    assert!(has_shared_input(&[[false, true], [true, true]]));
    assert!(!has_shared_input::<2>(&[]));
    assert!(has_shared_input(&[[]]));
}
