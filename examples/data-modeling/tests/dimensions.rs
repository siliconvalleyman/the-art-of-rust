use data_modeling::dimensions::Dimensions;

#[test]
fn rotation_preserves_area_and_exchanges_the_dimensions() {
    let mut dimensions = Dimensions::new(30, 40);
    dimensions.rotate();
    assert_eq!(dimensions.area_mm2(), 1200);
    assert_eq!(dimensions.into_pair(), (40, 30));
}

#[test]
fn dimensions_cover_zero_and_maximum_widths() {
    assert_eq!(Dimensions::new(0, u16::MAX).area_mm2(), 0);
    assert_eq!(
        Dimensions::new(u16::MAX, u16::MAX).area_mm2(),
        4_294_836_225
    );
}
