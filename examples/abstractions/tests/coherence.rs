use abstractions::coherence::{DispatchId, HasLabel};

#[test]
fn extension_and_wrapper_keep_their_contracts_separate() {
    assert!("fragile".has_label());
    assert!(!" \t".has_label());
    assert_eq!(DispatchId(7).to_string(), "dispatch-7");
    assert_eq!(7_u32.to_string(), "7");
}
