use data_modeling::weight::{Grams, ParcelWeight, WeightError};

#[test]
fn checked_construction_accepts_both_inclusive_boundaries() {
    assert_eq!(ParcelWeight::new(Grams::new(0)), Err(WeightError::Zero));
    for accepted in [1, ParcelWeight::MAX_GRAMS] {
        assert_eq!(
            ParcelWeight::new(Grams::new(accepted))
                .unwrap()
                .grams()
                .value(),
            accepted
        );
    }
    assert_eq!(
        ParcelWeight::new(Grams::new(ParcelWeight::MAX_GRAMS + 1)),
        Err(WeightError::TooHeavy)
    );
    assert_eq!(
        ParcelWeight::new(Grams::new(u32::MAX)),
        Err(WeightError::TooHeavy)
    );
}

#[test]
fn packaging_distinguishes_policy_limit_from_machine_width() {
    let parcel = ParcelWeight::new(Grams::new(29_900)).unwrap();
    assert_eq!(
        parcel
            .with_packaging(Grams::new(100))
            .unwrap()
            .grams()
            .value(),
        30_000
    );
    assert_eq!(
        parcel.with_packaging(Grams::new(101)),
        Err(WeightError::TooHeavy)
    );
    assert_eq!(
        parcel.with_packaging(Grams::new(u32::MAX)),
        Err(WeightError::TooHeavy)
    );
    assert_eq!(parcel.grams().value(), 29_900);
}
