# P03. Unit conversion utility

Build a command that expresses a length in a different metric unit.
Measurements such as `0.001 mm` must survive an exact conversion to
kilometers. A report may instead require two decimal places and a
well-defined tie rule. This project uses
[numeric conversions](../part-03/013-numeric-conversions.md),
[validated units](../part-06/029-newtypes-and-invariants.md), and
[bounded input contracts](../part-09/045-bounded-recovery.md).

## Pick the input and output contracts

Use `art-convert <quantity> <from> <to> [places]`. Units are the
case-sensitive ASCII strings `mm`, `cm`, `m`, and `km`. A quantity
has at least one ASCII decimal digit in its whole part, whose value
fits `u64`. A decimal point, if present, must be followed by one to
three ASCII digits. Zero and leading zeros are allowed. A sign,
whitespace, a decimal exponent, and a second decimal point are errors.
No fields repeat the supplied data in a diagnostic.

Output is one decimal number and a newline. When `places` is absent,
the format is exact and discards trailing fractional zeros. For an
explicit decimal `places` from 0 through 9, print that many fractional
digits, rounding a tie upward. Test both sides of ties and rounding
that carries into the whole part. The runtime checks arity, the two
units, the quantity, then the precision. It exits with `2` for
invalid input, `1` after an output failure, and `0` after successfully
flushing the result. A failed write can leave partial output.

## Choose an exact storage unit and prove its capacity

| Input unit | Millimeters in one unit | Micrometers in one thousandth |
| --- | ---: | ---: |
| `mm` | 1 | 1 |
| `cm` | 10 | 10 |
| `m` | 1,000 | 1,000 |
| `km` | 1,000,000 | 1,000,000 |

The conversion factors follow the [SI prefix definitions](https://www.bipm.org/en/-/resolution-cgpm-11-12).

An integer count of micrometers represents all supported input
lengths. Parse the whole part and the fractional contribution to
thousandths separately. The value `12.04` has `12 * 1000 + 40`
thousandths. Multiply this integer by the source unit's millimeter
factor to get micrometers. Store the result in a `Length` newtype
with a private field. `Unit` is an enum whose factors are total over
the supported choices.

The count of thousandths is less than `2^64 * 1000`, so the stored
value is less than `2^64 * 10^9`. Rendering multiplies it by at most
`10^9` before dividing. The numerator is therefore less than
`2^64 * 10^18`, which is below `2^124` because `10^18 < 2^60`.
The largest divisor is `10^9`. Doubling a remainder and adding an
increment of one to the quotient both fit `u128` under these bounds.
The code validates its limits before relying on them.

## Derive the formatting and rounding steps

If `D` is the number of micrometers in one output unit and `p` is
the required number of places, scale the measurement by `10^p`.
Divide the scaled value by `D`, obtaining quotient `q` and residue
`r` with `0 <= r < D`. Rounding upward exactly when `2r >= D`
chooses the nearest integer and resolves a tie upward. Split the
result into whole and fractional parts by dividing by the scale.
Padding the fractional part preserves the requested width.

For exact display, use `p = 9`. Every supported `D` divides `10^9`,
so `r` is zero. Removing trailing fractional zeros preserves the
number. A boundary such as `9.995 m` to two decimal places rounds
from `999.5` hundredths to `1000` hundredths, displayed as `10.00`.

```rust,edition2021
{{#include ../../projects/p03-unit-conversion/src/lib.rs}}

let small = Length::parse(OsStr::new("0.001"), Unit::Millimeter).unwrap();
assert_eq!(small.render(Unit::Kilometer, None).unwrap(), "0.000000001");
let tie = Length::parse(OsStr::new("9.995"), Unit::Meter).unwrap();
assert_eq!(tie.render(Unit::Meter, Some(2)).unwrap(), "10.00");
```

Implement and test the parser before the formatter. Test unit
conversion with an exact one-way value and its inverse scale, then
check maximum input and the smallest supported conversion. Finally
wire the CLI to the library and verify status and streams at the
process boundary. The library only deals in quantities and units.

## Run and verify

From the repository root with the pinned Rust toolchain, use the
following commands.

```sh
cargo run --locked -p art-convert -- 150 cm m 0
cargo run --locked -p art-convert -- 0.001 mm km
cargo test --locked -p art-convert
cargo build --locked --release -p art-convert
```

The first command prints `2`. The second prints `0.000000001`.
The release program is in `target/release/`, with an `.exe` suffix
on Windows. Its tests cover parsing, rounding, formatting, and
successful and rejected invocations.

## Review costs and extensions

The parser performs linear work in the amount of argument text
examined, with constant additional state. The transform uses a bounded
number of fixed-width operations. The output buffer is bounded by the
largest permitted conversion and nine fractional digits. The command
uses operating system argument limits and treats unexpected argument
text as an ordinary error. It accepts no files or external data.

This representation is for nonnegative lengths in units with
power-of-ten factors. Support for signed operands needs an explicit
tie rule on negatives. Non-decimal factors can require a rational
representation to delay rounding. Units with an offset, such as
temperatures on different scales, need an affine transform and a
different dimension type.
