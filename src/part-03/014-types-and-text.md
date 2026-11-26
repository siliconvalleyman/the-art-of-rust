# 014. Compound types, Unicode, and strings

A workshop program receives the label `Aé🦀`. A device
can display only the first four bytes of a message. If the
program blindly takes four bytes from the label, it ends
inside the crab's encoding. To make the output valid,
the code needs to know the difference between a byte limit
and a limit on Unicode scalar values. This chapter starts
with what a sequence stores and then makes that boundary
precise.

**Prerequisites.** [Names and types](012-names-and-expressions.md)
and [numeric conversions](013-numeric-conversions.md).
After this chapter, you should be able to model a small
record or fixed-size sequence, explain how bytes encode
Unicode scalar values, and trim valid text to a byte
budget while preserving valid UTF-8.

## Tuples group fields, and arrays repeat one type

A **tuple** groups values in a fixed number of positions.
The positions may have different types. For a number and
a Boolean flag, the tuple type is `(u16, bool)`. The
expression `sample.0` selects the first field and
`sample.1` selects the second. A **pattern** names the
fields you want to use as part of a `let` declaration.

```rust,edition2021
let sample: (u16, bool) = (42, true);
assert_eq!(sample.0, 42);
let (count, ready) = sample;
assert_eq!(count, 42);
assert!(ready);
```

The example groups two observations by position. With
more fields, names can make an interface easier to
maintain, especially when several fields have identical
types. Chapter 26 introduces named records with structs.
Rust can choose the layout of a tuple under its default
representation rules. Treat the order of tuple access as
part of the language, and define a separate byte format
for data sent to another program.

An **array** holds a fixed number of values of the same
type. The type `[u8; 3]` holds exactly three bytes.
The expression `values[0]` reads the first element.
An array stores its elements consecutively, spaced by
the element type's size. Position 2 in this example
is two element sizes past the beginning, so it is
two bytes after the first element.

```rust,edition2021
let values = [10_u8, 20, 30];
assert_eq!(values.len(), 3);
assert_eq!(values[2], 30);
assert_eq!(values.get(3), None);
let pulse = [0_u8; 4];
assert_eq!(pulse, [0, 0, 0, 0]);
```

The expression `[0_u8; 4]` initializes all four
positions to zero. The length is part of the array
type. An **array slice** is a view of consecutive
elements whose length can vary at runtime. The
expression `&values[1..3]` makes a borrowed view of
positions 1 and 2. A **borrowed view** refers to
existing data for a limited period. Chapter 22
explains how Rust checks that period. The range is
half-open: the lower bound is included and the upper
bound excluded, so the number of positions is the
difference of the bounds.

Direct indexing requires an in-range index. An invalid
index can be rejected at compile time when it is
statically obvious, or can trigger a panic at runtime.
When an index comes from outside the program, a
fallible access such as `get` can return absence and
let the caller respond. For a bounded dataset, the
type and index rule let the program identify the right
element without scanning earlier positions.
[Rust Reference on type layout](https://doc.rust-lang.org/1.75.0/reference/type-layout.html).

## Unicode gives text numbers, and UTF-8 stores them

Unicode assigns integers called **code points** to
positions in its codespace. One textual notation is
`U+00E9` for the code point of `é`. A Unicode
**scalar value** is a Unicode code point other than
the surrogate range U+D800 through U+DFFF. Rust's
`char` represents one scalar value. It always has
size four bytes as a standalone value in Rust.

**UTF-8** encodes a scalar value using one to four
bytes. One-byte values use the same encoding as
ASCII. For longer encodings, the leading byte's
high bits indicate the length, and each continuation
byte begins with binary `10`. A UTF-8 validator
also rejects overlong encodings, surrogate code points,
and values beyond U+10FFFF. Those conditions prevent
different invalid byte encodings from being treated
as equivalent valid representations for a scalar.
[RFC 3629, UTF-8](https://www.rfc-editor.org/rfc/rfc3629).

| Scalar | Code point | UTF-8 bytes | Starting byte offset |
| --- | --- | --- | ---: |
| A | U+0041 | `41` | 0 |
| é | U+00E9 | `c3 a9` | 1 |
| 🦀 | U+1F980 | `f0 9f a6 80` | 3 |

*Figure 1. The label uses seven bytes and three scalar
values. Its UTF-8 boundaries are 0, 1, 3, and 7.*

The scalar `é` supplies another distinction. A
display can present `é` as a single visible unit,
either as U+00E9 or as `e` followed by U+0301, a
combining accent. The UTF-8 bytes and scalar counts
differ. A **grapheme cluster** is a grouping intended
to approximate a user-perceived character under
Unicode segmentation rules. Text editors can need
such boundaries for cursor movement. A byte-oriented
protocol should still specify bytes when that is what
its capacity measures.
[Unicode Standard Annex #29, revision 43](https://www.unicode.org/reports/tr29/tr29-43.html).

## Read a string by the right unit

Rust's `str` type represents valid UTF-8 text. A
`&str` is a borrowed view that carries a pointer
and a byte length. A string literal has type
`&'static str`, a view of data that remain available
throughout the program. A `String` owns a growable
UTF-8 buffer. Use one when building text from pieces
or retaining it after the original source becomes
unavailable.
[Rust standard library documentation for str](https://doc.rust-lang.org/1.75.0/std/primitive.str.html) · [String](https://doc.rust-lang.org/1.75.0/std/string/struct.String.html).

```rust,edition2021
let label = "Aé🦀";
assert_eq!(label.len(), 7);
assert_eq!(label.chars().count(), 3);
assert_eq!(label.get(1..3), Some("é"));
assert_eq!(label.get(1..2), None);

let mut greeting = String::from("A");
greeting.push('é');
greeting.push_str("🦀");
assert_eq!(greeting, label);
```

The method `len` returns the stored byte length.
`chars()` walks Unicode scalar values by decoding the
bytes. Counting them requires traversing the text.
The slice request `1..3` starts at the second
scalar and stops at the start of the third. The request
`1..2` ends in a continuation byte without a
valid boundary. Rust's `str` does not support
indexing with a single integer. A caller can choose raw
bytes using `as_bytes`, scalar values with `chars`,
or byte ranges with endpoints checked for UTF-8.

The `String` example copies the initial text into
an owned buffer and appends one scalar value followed
by another string view. The buffer may grow to
accommodate new bytes. Capacity is distinct from the
number of bytes currently used. A reference to a
portion of the contents must remain valid throughout
its use, which motivates the ownership and borrowing
rules covered later.

To interpret arbitrary bytes, validate them with
`std::str::from_utf8`. Invalid UTF-8 is an error.
Using a lossy decoder that inserts replacement
characters is a separate policy. A log viewer might
prefer a visible replacement. A field used as an exact
identifier might need rejection so different invalid
inputs cannot collapse to the same displayed value.

## Derive a byte-limited prefix

The input is valid text and a nonnegative byte limit.
Return its longest valid prefix whose byte length
does not exceed the limit. Start at the smaller of
the input length and the limit. While that index is
inside a scalar value, move down by one byte. The
first boundary reached is the longest permitted one.

```rust,edition2021
fn prefix_bytes(text: &str, limit: usize) -> &str {
    let mut end = limit.min(text.len());
    while !text.is_char_boundary(end) {
        end -= 1;
    }
    &text[..end]
}

assert_eq!(prefix_bytes("Aé🦀", 4), "Aé");
assert_eq!(prefix_bytes("Aé🦀", 7), "Aé🦀");
assert_eq!(prefix_bytes("é", 1), "");
assert_eq!(prefix_bytes("", 99), "");
```

At each iteration, all already skipped positions within
the budget are non-boundaries. This starts true because
we have skipped none. Rejecting a non-boundary and
decrementing preserves it. Zero is always a boundary,
so an index undergoing a decrement must be positive.
The decreasing integer gives termination. At exit the
text and budget bound still hold, and no greater eligible
index can be a boundary. Therefore the returned subslice
is valid and longest.

UTF-8 uses at most four bytes per scalar, so the
starting index is at most three positions past the
previous boundary. The loop requires at most three
decrements and the result borrows the input with no
new string allocation. The algorithm can still split
a grapheme cluster between scalars. A display layout
requirement would need a grapheme-aware algorithm with
its own cost model.

## Practice and assessment

{{#include ../../exercises/C014.md}}

## Summary

A tuple gives heterogeneous fields fixed positions.
An array gives homogeneous elements a fixed count.
Rust strings hold valid UTF-8. Byte lengths, scalar
counts, and grapheme boundaries answer different
questions. Name the unit in a text operation and
validate any proposed byte boundary before slicing.

[Numeric conversions, overflow, and floating point](013-numeric-conversions.md) · [Solutions](../rust-fundamentals-solutions.md#chapter-014) · [Branches, loops, functions, and recursion](015-control-flow.md)
