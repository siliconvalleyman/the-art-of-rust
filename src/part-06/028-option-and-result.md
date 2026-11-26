# 028. Option, Result, and domain states

A scanner reports a count for an incoming parcel. Sometimes
the scanner has not supplied a field. Sometimes it reports
zero. Sometimes it supplies invalid text. A caller may wait
for a missing field, accept zero as a measurement, and flag
an invalid entry for correction. A useful interface makes
all three outcomes representable.

**Prerequisite.** [Enums, sums, destructuring, and exhaustive matches](027-enums-and-matching.md).
[Numeric conversions](../part-03/013-numeric-conversions.md) and
[UTF-8](../part-03/014-types-and-text.md) explain range and byte
checks. After this chapter, you should be able to model absence
and failure, justify validation order, and preserve the original
conditions through a conversion.

## Read the possibilities from the type

`Option<T>` has the two variants `None` and `Some(T)`.
Read `T` as the payload type, which an interface chooses.
For an optional byte, the 256 numeric values appear inside
`Some` and `None` adds one distinct alternative. Using
zero as the empty marker would discard one legitimate
payload. A number inside the option can still be zero.

`Result<T, E>` has the two variants `Ok(T)` and
`Err(E)`. The latter carries a reason chosen by the
interface. Returning `Result<Option<u8>, ScanError>`
says that on success a field may be absent, and
when validation fails the reason is available. The
outer match can answer whether validation succeeded.
The inner match can then answer whether the scanner
reported a count.

| Situation | Output |
| --- | --- |
| No supplied field | `Ok(None)` |
| Supplied field with a valid value of zero | `Ok(Some(0))` |
| Supplied text fails the just-defined input contract | `Err(reason)` |

An interface can substitute a domain-specific enum when
names describe the outcomes better. The nested standard
types are useful when callers already compose fallible
operations. Either choice should preserve distinctions the
caller needs. A blanket fallback to `None` on every
failure would hide whether a supplied reading was corrupt.

## Specify validation before implementing it

The parser accepts an optional UTF-8 string view.
A supplied field must contain one, two, or three ASCII
digits. Leading zeros are permitted. A field longer
than three bytes fails the length check before syntax
is inspected. A field within that limit fails syntax
if it is empty or contains a nondigit. Nonempty digit
fields continue to numeric conversion. Values greater
than 255 fail the range check.

Length is counted in bytes because the accepted grammar
uses only ASCII characters. A lookalike Unicode digit
does not pass the ASCII predicate. For example, a
single two-byte letter reaches syntax checking and
fails it. Longer non-ASCII text can fail the byte
limit earlier. Validation order is observable once
different paths report different errors.

```rust,edition2021
{{#include ../../examples/data-modeling/src/scan.rs}}

assert_eq!(parse_count(None), Ok(None));
assert_eq!(parse_count(Some("000")), Ok(Some(0)));
assert_eq!(parse_count(Some("255")), Ok(Some(255)));
assert_eq!(parse_count(Some("256")), Err(ScanError::OutOfRange));
assert_eq!(parse_count(Some("bad!")), Err(ScanError::TooLong));
```

The `let Some(field) = field else` statement unwraps
the supplied field. The `else` path returns immediately
when it is absent. The digit predicate after the byte
limit ensures every input byte lies between `b'0'`
and `b'9'`. This justifies subtracting the base value
of the zero digit. With the resulting digits, a left
fold builds the number. A **fold** updates an accumulator
once per input in sequence.

## Prove the representation and error boundaries

Before processing any digits, the accumulator is zero,
the value of the empty prefix. If the accumulator is
the decimal value of a prefix, appending digit `d`
shifts that prefix one decimal place and adds `d`.
Updating with `10 × value + d` therefore preserves the
invariant. The loop model processes at most three
digits. Its largest prefix is 999, so each operation
fits in `u16`.

The final `u8::try_from` succeeds exactly when that
accumulated value fits in a byte. `map(Some)` wraps a
successful byte while leaving an error to be handled.
`map_err` changes the conversion failure into the
scanner's range error. The length and syntax checks
already ensure that the attempted conversion represents
the complete input.

The implementation does at most three syntax checks and
three digit updates. It creates no heap allocation and does
not copy the input string. An external reader still needs
its own transport limit if it has to collect the whole
field before making this call. This bounded parser already
has a borrow of the field and can reject long views
without walking their bytes.

## Handle outcomes where a policy belongs

```rust,edition2021
{{#include ../../examples/data-modeling/src/scan.rs}}

fn status(field: Option<&str>) -> &'static str {
    match parse_count(field) {
        Ok(None) => "waiting",
        Ok(Some(0)) => "empty parcel",
        Ok(Some(_)) => "count recorded",
        Err(_) => "rescan",
    }
}

assert_eq!(status(None), "waiting");
assert_eq!(status(Some("0")), "empty parcel");
assert_eq!(status(Some("x")), "rescan");
```

The application can merge different error causes into one
operator message while the parser retains their distinctions
for callers that need them. Test the policy's behavior
using at least one value from each domain outcome. If a
caller stores a missing count as zero, downstream code
can no longer recover the distinction. Keep the state
until the design explicitly chooses how to act on it.

An optional value's representation is not a byte format.
The standard library documents size and alignment guarantees
for certain payload types. For example, it includes
`Option<NonZeroU16>` in the representations documented for
nonzero integers. The unused zero case can represent
absence for this type. `Option<u8>` has 257 possible
logical states, so a single eight-bit storage byte cannot
represent them all. Consult the documented guarantee for
the specific payload rather than assuming one layout for
every Option.

## Practice and assessment

{{#include ../../exercises/C028.md}}

## Summary and further reading

Option represents presence or absence. Result preserves
success or failure and can carry either an optional value
or an error. The interface determines which combinations
are meaningful and which validation order callers observe.
[Rust 1.75.0 Option representation](https://doc.rust-lang.org/1.75.0/std/option/index.html#representation) · [Result](https://doc.rust-lang.org/1.75.0/std/result/enum.Result.html).

[Enums and matching](027-enums-and-matching.md) · [Solutions](../data-modeling-solutions.md#chapter-028) · [Newtypes and invariants](029-newtypes-and-invariants.md)
