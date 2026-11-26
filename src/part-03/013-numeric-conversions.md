# 013. Numeric conversions, overflow, and floating point

A field stores an unsigned byte, and a reader receives the
value 260 from a wider count. One conversion can produce 4.
Another reports a range error. Both can be correct operations
when the interface specifies their meaning. A separate sensor
reports 0.1, which a binary floating-point type stores
approximately. This chapter gives you the tools to choose
and test all three behaviors.

**Prerequisites.** [Names, mutation, primitive types, and expressions](012-names-and-expressions.md)
and [counting, probability, and modular arithmetic](../part-02/009-counting-and-probability.md).

After this chapter, you should be able to select explicit
conversion and arithmetic policies, prove their boundary
behavior, and explain how rounding, infinity, and NaN arise.

## Integer ranges and two's complement

An unsigned word of width `w` represents 0 through
`2^w−1`. A signed Rust integer of that width uses
**two's complement**. Treat the top bit as having weight
`−2^(w−1)`, with the other bits retaining their usual
positive weights. If the top bit is clear, the range is
0 through `2^(w−1)−1`. If it is set, the range is
`−2^(w−1)` through −1. This yields an extra negative
value. Its positive magnitude is one more than the largest
positive value in the type.

| Byte bits | Unsigned | Signed |
| --- | ---: | ---: |
| `00000000` | 0 | 0 |
| `01111111` | 127 | 127 |
| `10000000` | 128 | −128 |
| `11111111` | 255 | −1 |

*Figure 1. The interpretation changes when the top bit
has negative weight. The bits themselves can match.*

When widening a signed value with a negative sign, its
higher positions are logically filled with ones to preserve
the signed value. A wider unsigned interpretation of the
same original bits can have a different value. Conversion
rules state what happens to the value, and a storage format
states the byte order as a separate choice.
[Rust Reference on numeric types](https://doc.rust-lang.org/reference/types/numeric.html).

## Choose a conversion contract

Use a lossless widening such as `u16::from(byte)` when
every source value has the same value in the destination
type. Use `u8::try_from(value)` when the incoming range
may exceed the destination and failure should be visible.
It returns a result indicating successful conversion or
an error. The examples use `Ok` for a successful value
and `is_err()` to check failure. The chapter on data
modeling will explain the result type.

For narrowing from an integer to a smaller integer, a
cast written with `as` drops the higher bits. Converting
`260_u16 as u8` yields 4. In the unsigned-to-unsigned
case, this is the remainder modulo the number of values
in the destination. Use that behavior only when the
application requires the low part. A wrapping sequence
number can have that contract. A length used to validate
whether input fits a field usually needs an explicit
range check.
[Rust Reference on numeric casts](https://doc.rust-lang.org/reference/expressions/operator-expr.html#type-cast-expressions).

```rust,edition2021
assert_eq!(u16::from(250_u8), 250);
assert_eq!(u8::try_from(255_u16), Ok(255));
assert!(u8::try_from(260_u16).is_err());
assert_eq!(260_u16 as u8, 4);
assert_eq!((-1_i8) as u8, 255);
```

The last cast preserves the eight-bit pattern according
to the same-width integer cast rule. If the contract is
to reject negative values, casting would instead transform
the negative input into a positive byte. For a partially
trusted numeric field, choose a test that distinguishes
those outcomes. Testing only small positive values would
pass both conversion policies.

## Make overflow behavior explicit

An arithmetic operation overflows when its mathematical
result cannot be represented by the type. For integer
addition, subtraction, and multiplication with overflow
checks enabled, a runtime overflow triggers a panic.
With those checks disabled, these operations wrap.
Cargo's default development and release profiles
differ in that setting. Profiles are configurable, so
an API should encode its chosen policy where correctness
depends on the outcome. Rust also treats certain
conditions separately, including zero divisors and
signed minimum divided by −1.
[Rust overflow rules](https://doc.rust-lang.org/reference/expressions/operator-expr.html#overflow) · [Cargo profiles](https://doc.rust-lang.org/cargo/reference/profiles.html).

| Operation | Result for 250 and 20 as bytes | Suitable contract |
| --- | --- | --- |
| `checked_add` | `None` | An out-of-range input must make the operation fail |
| `wrapping_add` | 14 | Arithmetic follows an explicitly modular counter |
| `saturating_add` | 255 | Values above the limit are represented by the cap |
| `overflowing_add` | `(14, true)` | The low part and the carry are both needed |

```rust,edition2021
let count = 250_u8;
assert_eq!(count.checked_add(20), None);
assert_eq!(count.wrapping_add(20), 14);
assert_eq!(count.saturating_add(20), 255);
assert_eq!(count.overflowing_add(20), (14, true));
```

For an unsigned byte pair, the ordinary sum lies in
0 through 510. When it exceeds 255, subtracting 256
gives the low part and setting a carry distinguishes that
case from an ordinary sum with the same byte. The adder
proof in [Chapter 2](../part-01/002-bits.md) establishes
this correspondence. With signed values, the flag on
`overflowing_add` reports signed range overflow, which
need not equal the outgoing carry of the underlying bits.
For instance, `127_i8 + 1` exceeds the signed range even
though adding those unsigned byte patterns gives 128 with
no carry out.

The order of operations also matters. An average of two
bytes can be at most 255. Intermediate addition in a
byte can still overflow, so widen before adding. For
floor division of the sum by two, the intermediate
fits in a `u16` and the final result is bounded by 255.
The conversion back is then valid for every permitted input.

## Floating point trades precision across magnitudes

Rust's `f32` and `f64` use binary floating-point
formats with fields for sign, exponent, and fraction.
For a normal finite `f64` value, the sign selects
positive or negative, the exponent supplies a power of
two, and the fraction encodes the fractional part of
a significand with an implicit leading 1. A **significand**
is the factor multiplied by the power of two. For these
normal values `f64` has 52 explicitly stored fraction
bits plus the leading bit, for 53 bits of precision.
Subnormal values use a different leading bit and allow
smaller magnitudes near zero.

Since `1/8 = 2^(−3)`, its finite binary
expansion represents it exactly. The reduced fraction
for 0.1 is `1/10`. Its denominator has a factor of
five. Every finite binary fraction can be expressed with
a power of two in its denominator, so no finite binary
fraction equals it. Conversion uses a representable
approximation. Rounding can also occur after an operation
even when both inputs were representable.

As the exponent increases, the spacing between nearby
representable values increases. At `2^53`, the next
larger `f64` value is `2^53 + 2`. The integer
`2^53 + 1` falls between them and converting it rounds
to `2^53` under nearest-even rounding. This is a boundary
where converting a whole-number identifier to a float can
collapse distinct values. Keep identifiers and exact
counts as integers unless approximate arithmetic is part
of the contract.

```rust,edition2021
let exact = 0.125_f64;
assert_eq!(exact * 8.0, 1.0);
let boundary = 1_u64 << 53;
assert_eq!((boundary + 1) as f64, boundary as f64);
assert_eq!((boundary + 2) as f64, boundary as f64 + 2.0);
```

An infinity represents an unbounded result in the
floating-point format. **NaN**, short for “not a
number,” can arise from an operation such as dividing
floating-point zero by zero. A comparison of a NaN
with itself for equality is false. Check `is_nan()`
when that is the property you intend, and use
`is_finite()` to enforce an interface that excludes
both infinities and NaNs. The bit patterns of NaNs
can vary. Sort or serialize a float only after
specifying the treatment of special cases.
[Rust standard library documentation for f64](https://doc.rust-lang.org/std/primitive.f64.html).

```rust,edition2021
let invalid = f64::NAN;
assert!(invalid.is_nan());
assert!(!f64::INFINITY.is_finite());
assert_eq!(invalid.partial_cmp(&0.0), None);
```

A numeric cast from a float to an integer rounds toward
zero. It saturates at the integer bounds for out-of-range
values, and a NaN maps to zero. With a `u8` destination,
`12.9_f64 as u8` yields 12, `300.0_f64 as u8` yields
255, and `f64::NAN as u8` yields zero. If the receiving
contract requires an exact byte-valued measurement,
check finiteness, range, and that the input has no
fractional part before casting. Distinguish false
acceptance from a legitimate zero measurement.
[Rust Reference on type-cast expressions](https://doc.rust-lang.org/reference/expressions/operator-expr.html#type-cast-expressions).

## Practice and assessment

{{#include ../../exercises/C013.md}}

## Summary

Track the range and written width at each arithmetic
operation. Use a conversion that expresses whether data
loss is expected. Decide which overflow and special-value
cases a numeric interface accepts, then test the cases
that could be silently confused with valid results.

[Names, mutation, primitive types, and expressions](012-names-and-expressions.md) · [Solutions](../rust-fundamentals-solutions.md#chapter-013) · [Compound types, Unicode, and strings](014-types-and-text.md)
