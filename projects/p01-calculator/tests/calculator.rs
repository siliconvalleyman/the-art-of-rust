use std::{
    ffi::OsStr,
    process::{Command, Output},
};

use art_calc::{calculate, CalcError, Operator};

#[test]
fn signed_operations_have_defined_boundaries() {
    use CalcError::{DivisionByZero, Overflow};
    use Operator::{Add, Divide, Multiply, Remainder, Subtract};

    assert_eq!(Add.evaluate(i64::MAX, 0), Ok(i64::MAX));
    assert_eq!(Add.evaluate(i64::MAX, 1), Err(Overflow));
    assert_eq!(Subtract.evaluate(i64::MIN, 1), Err(Overflow));
    assert_eq!(Multiply.evaluate(i64::MAX, 2), Err(Overflow));
    assert_eq!(Divide.evaluate(i64::MIN, -1), Err(Overflow));
    assert_eq!(Remainder.evaluate(i64::MIN, -1), Err(Overflow));
    assert_eq!(Divide.evaluate(2, 0), Err(DivisionByZero));
    assert_eq!(Remainder.evaluate(2, 0), Err(DivisionByZero));
    assert_eq!(Divide.evaluate(-7, 3), Ok(-2));
    assert_eq!(Remainder.evaluate(-7, 3), Ok(-1));
}

#[test]
fn parsing_follows_the_documented_input_order() {
    let [bad, operator, valid] = ["bad data", "?", "7"].map(OsStr::new);
    assert_eq!(calculate(bad, operator, bad), Err(CalcError::Operand));
    assert_eq!(calculate(valid, operator, bad), Err(CalcError::Operator));
    assert_eq!(
        calculate(valid, OsStr::new("+"), bad),
        Err(CalcError::Operand)
    );
}

fn run(args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_art-calc"))
        .args(args)
        .output()
        .expect("run the calculator executable")
}

#[test]
fn the_cli_writes_only_the_answer_on_success() {
    let output = run(&["-7", "*", "9"]);
    assert_eq!(output.status.code(), Some(0));
    assert_eq!(output.stdout, b"-63\n");
    assert!(output.stderr.is_empty());
}

#[test]
fn the_cli_distinguishes_usage_input_and_arithmetic_errors() {
    for (args, code, message) in [
        (
            &["1", "+"][..],
            2,
            "usage: art-calc <left> <operator> <right>\n",
        ),
        (
            &["1", "+", "2", "3"][..],
            2,
            "usage: art-calc <left> <operator> <right>\n",
        ),
        (
            &["1", "+", "untouched field"][..],
            2,
            "expected a signed 64-bit decimal operand\n",
        ),
        (&["1", "/", "0"][..], 1, "divisor must be nonzero\n"),
        (
            &["9223372036854775807", "+", "1"][..],
            1,
            "arithmetic overflow\n",
        ),
    ] {
        let output = run(args);
        assert_eq!(output.status.code(), Some(code));
        assert!(output.stdout.is_empty());
        assert_eq!(output.stderr, message.as_bytes());
    }
}
