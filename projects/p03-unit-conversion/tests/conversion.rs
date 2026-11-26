use std::{ffi::OsStr, process::Command};

use art_convert::{ConversionError, Length, Unit};

fn convert(text: &str, from: Unit, to: Unit, places: Option<u8>) -> String {
    Length::parse(OsStr::new(text), from)
        .unwrap()
        .render(to, places)
        .unwrap()
}

#[test]
fn exact_values_cover_both_conversion_directions_and_extremes() {
    use Unit::{Kilometer, Meter, Millimeter};
    assert_eq!(convert("0.001", Millimeter, Kilometer, None), "0.000000001");
    assert_eq!(convert("4.825", Kilometer, Meter, None), "4825");
    assert_eq!(convert("10.000", Meter, Meter, None), "10");
    assert_eq!(convert("0", Meter, Meter, None), "0");
    assert_eq!(
        convert("18446744073709551615.999", Kilometer, Millimeter, None),
        "18446744073709551615999000"
    );
}

#[test]
fn rounding_handles_ties_carries_and_padded_places() {
    use Unit::{Centimeter, Meter};
    assert_eq!(convert("149.999", Centimeter, Meter, Some(0)), "1");
    assert_eq!(convert("150", Centimeter, Meter, Some(0)), "2");
    assert_eq!(convert("9.995", Meter, Meter, Some(2)), "10.00");
    assert_eq!(convert("1.2", Meter, Meter, Some(4)), "1.2000");
}

#[test]
fn invalid_values_have_typed_errors() {
    for input in [
        "",
        "-0",
        "1e2",
        ".3",
        "3.",
        "1.2345",
        "1.2.3",
        " 1",
        "18446744073709551616",
    ] {
        assert_eq!(
            Length::parse(OsStr::new(input), Unit::Meter),
            Err(ConversionError::Quantity)
        );
    }
    assert_eq!(Unit::parse(OsStr::new("mile")), Err(ConversionError::Unit));
    let length = Length::parse(OsStr::new("0"), Unit::Meter).unwrap();
    assert_eq!(
        length.render(Unit::Meter, Some(10)),
        Err(ConversionError::Precision)
    );
}

#[test]
fn binary_output_and_status_are_scriptable() {
    let binary = env!("CARGO_BIN_EXE_art-convert");
    let success = Command::new(binary)
        .args(["150", "cm", "m", "0"])
        .output()
        .unwrap();
    assert_eq!(success.status.code(), Some(0));
    assert_eq!(success.stdout, b"2\n");
    assert!(success.stderr.is_empty());

    let invalid = Command::new(binary)
        .args(["private field", "m", "cm"])
        .output()
        .unwrap();
    assert_eq!(invalid.status.code(), Some(2));
    assert!(invalid.stdout.is_empty());
    assert_eq!(
        invalid.stderr,
        b"expected a nonnegative decimal with at most three fractional digits\n"
    );
}
