use data_modeling::dispatch::{Dispatch, PrepareError};
use std::num::NonZeroU64;

#[test]
fn failed_validation_returns_the_original_draft_for_retry() {
    let draft = Dispatch::new(17);
    let (draft, error) = draft.prepare(0).unwrap_err();
    assert_eq!(error, PrepareError::Empty);
    let (draft, error) = draft.prepare(501).unwrap_err();
    assert_eq!(error, PrepareError::TooMany);
    assert_eq!(draft.id(), 17);

    let ready = draft.prepare(500).unwrap();
    let tracking = NonZeroU64::new(99).unwrap();
    assert_eq!(
        ready.record_shipment(tracking).receipt(),
        (17, 500, tracking)
    );
}

#[test]
fn single_parcel_is_a_valid_prepared_dispatch() {
    let ready = Dispatch::new(0).prepare(1).unwrap();
    let tracking = NonZeroU64::new(1).unwrap();
    assert_eq!(ready.record_shipment(tracking).receipt(), (0, 1, tracking));
    assert_eq!(NonZeroU64::new(0), None);
}
