use functional::quotes::{apply_discount, base_price, quote, receipt};

#[test]
fn quote_matches_the_composition_with_the_declared_rounding() {
    assert_eq!(quote(0), 0);
    assert_eq!(quote(3), 338);
    assert_eq!(quote(u16::MAX), 7_372_688);
    assert_eq!(quote(5), apply_discount(base_price(5)));
    assert_eq!(receipt(3), "items=3, cents=338");
}
