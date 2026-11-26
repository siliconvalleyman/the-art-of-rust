use functional::prefixes::qualified_prefix;

#[test]
fn borrowed_prefix_agrees_with_an_eager_reference_over_small_domains() {
    for encoded in 0..256_u16 {
        let values = [
            encoded & 3,
            (encoded >> 2) & 3,
            (encoded >> 4) & 3,
            (encoded >> 6) & 3,
        ];
        for minimum in 0..=4 {
            for limit in 0..=5 {
                let mut expected = Vec::new();
                for value in values {
                    if value >= minimum {
                        expected.push(value);
                    }
                }
                expected.truncate(limit);
                assert!(qualified_prefix(&values, minimum, limit).eq(expected));
            }
        }
    }
}

#[test]
fn empty_input_and_large_budgets_have_defined_results() {
    assert!(qualified_prefix(&[], 0, usize::MAX).eq([]));
    assert!(qualified_prefix(&[0, 7, u16::MAX], 7, 0).eq([]));
    assert!(qualified_prefix(&[0, 7, u16::MAX], 7, usize::MAX).eq([7, u16::MAX]));
}
