use abstractions::dispatch::{pipeline, via_generic, via_object, Adjust, Offset, Scale};

#[test]
fn concrete_and_erased_dispatch_have_the_same_adjustment_contract() {
    let rules: &[&dyn Adjust] = &[&Offset(1), &Scale(3)];
    for value in [0, 17, u32::MAX] {
        assert_eq!(via_generic(&Offset(1), value), via_object(rules[0], value));
        assert_eq!(via_generic(&Scale(3), value), via_object(rules[1], value));
    }
}

#[test]
fn pipeline_keeps_order_and_empty_pipeline_is_identity() {
    assert_eq!(pipeline(&[], 7), 7);
    assert_eq!(pipeline(&[&Offset(1), &Scale(3)], 7), 24);
    assert_eq!(pipeline(&[&Scale(3), &Offset(1)], 7), 22);
}
