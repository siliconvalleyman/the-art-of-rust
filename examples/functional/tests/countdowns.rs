use functional::countdown::Countdown;

#[test]
fn all_starts_agree_with_a_reverse_range() {
    for start in 0..=u8::MAX {
        let mut countdown = Countdown::new(start);
        assert_eq!(
            countdown.size_hint(),
            (usize::from(start), Some(usize::from(start)))
        );
        assert!(countdown.by_ref().eq((1..=start).rev()));
        assert_eq!(countdown.size_hint(), (0, Some(0)));
        assert_eq!(countdown.next(), None);
    }
}

#[test]
fn mapping_and_prefix_consumption_evaluate_only_requested_items() {
    use std::cell::Cell;

    let calls = Cell::new(0);
    let values = Countdown::new(4).map(|value| {
        calls.set(calls.get() + 1);
        value + 1
    });
    assert_eq!(calls.get(), 0);
    assert_eq!(values.take(2).collect::<Vec<_>>(), [5, 4]);
    assert_eq!(calls.get(), 2);
}
