# 002. Bits, binary, hexadecimal, and bounded arithmetic

The monitor in [Chapter 1](001-machine.md) wraps from 255 back to
0. The number 256 is easy to write on paper. Why is it different
inside the monitor? To answer, count the different states of the
storage, choose a meaning for each one, and follow an addition
past the last available position.

**Prerequisite.** [Chapter 1](001-machine.md) and division of whole
numbers with a quotient and remainder.

After this chapter you should be able to derive a number from its bits,
convert between binary and hexadecimal, separate an integer from
its byte representation, and explain what a carry means at a
fixed-width boundary.

## Symbols and positions

A **bit** is a position with two possible values, conventionally
written 0 and 1. A string of eight positions is one byte in this
book. Each added position doubles the number of possible strings
because either choice for the new position can accompany every
previous string. Eight positions therefore admit 2 multiplied
by itself eight times, or 256, distinct strings.

A **representation** assigns meanings to those strings. The same
byte could represent an unsigned count, part of a text encoding,
or eight independent switch settings. Its physical storage gives
us the choices. The format gives them a meaning. In the unsigned
binary format, a position numbered `i` from the right has weight
`2^i`. Here `^` in mathematical prose denotes exponentiation,
not the Rust bitwise operator. Position zero has weight 1 because
`2^0 = 1`.

| Position from the right | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Weight | 128 | 64 | 32 | 16 | 8 | 4 | 2 | 1 |
| Bits in `10100110` | 1 | 0 | 1 | 0 | 0 | 1 | 1 | 0 |

Add just the weights for positions that contain 1. This gives
128 + 32 + 4 + 2 = 166. The least significant bit, at position
zero, tells whether the unsigned value is odd, since all other
weights are even. If you replace the leading 1 with a zero,
subtract 128. If you drop a leading zero, the value is unchanged,
but the written width changes. Width matters in a stored field
or a text format that requires a specific number of digits.

The greatest unsigned byte is `11111111`. Its weights sum to
255. To see that without adding eight numbers, start with the
binary numeral for one followed by eight zeros. It has value 256.
One less replaces the leading 1 with the eight lower positions
set to 1, just as decimal 1000 − 1 yields 999. In general,
`w` bits give unsigned values 0 through `2^w − 1`.

**Compile-fail Rust preview.** After the first Rust programming
chapter, try assigning the literal 256 to a `u8`. The type
specifies an unsigned eight-bit value. The literal is outside
that range, so Rust 1.75.0 rejects this example with its
default lint settings. This check concerns a literal in
source code. The later chapter on numeric conversions will
explain how to handle an out-of-range value obtained at runtime.

```rust,compile_fail,edition2021
{{#include ../../tests/fixtures/out_of_range_byte.rs}}
```

## Derive the inverse by division

To encode 38, divide it by 2. The remainder tells whether 38 is
odd, so it gives the least significant bit. The quotient contains
the remaining positions, shifted down one place. Repeat while
there are positions to fill.

| Value before division | Quotient | Remainder | Position filled |
| ---: | ---: | ---: | ---: |
| 38 | 19 | 0 | 0 |
| 19 | 9 | 1 | 1 |
| 9 | 4 | 1 | 2 |
| 4 | 2 | 0 | 3 |
| 2 | 1 | 0 | 4 |
| 1 | 0 | 1 | 5 |
| 0 | 0 | 0 | 6 |
| 0 | 0 | 0 | 7 |

Read the remainders from position 7 down to position 0 to obtain
`00100110`, whose weights sum to 32 + 4 + 2. The remainder order
is a consequence of the first step extracting the least significant
position. Reversing the mathematical display order is part of the
conversion.

To justify the procedure, let `n` be the original nonnegative
integer and `q` the value still to encode after filling `k`
positions. At this point, `n` is the value of the emitted low
bits plus `2^k × q`. Before any work the emitted value is 0 and
`q = n`. The next division expresses `q` as twice a new
quotient plus a remainder of 0 or 1. Substituting that equality
preserves the relationship while putting the remainder in position
`k`. After eight divisions the quotient is zero for `n < 256`.
If it were still positive, the required representation would need
more than eight bits. Uniqueness follows because the first bit is
forced by the remainder and the same argument forces each later bit.

A fixed-width encoder makes exactly `w` steps. Under a model
where operations on the working word take constant time, the time
is proportional to `w` and an array of output bits uses `w`
positions. A variable-width encoder can finish when the quotient
first reaches zero, with a special representation for the value 0.
The cost model changes for integers much larger than a machine
word. The loop count still follows the number of emitted positions.

## Hexadecimal groups four bits

**Hexadecimal** uses base 16. The digit symbols `0` through `9`
represent their decimal values, then `A` through `F` represent
10 through 15. Since 16 is `2^4`, all four-bit strings fit the
range of one hex digit. Separate a binary number into groups of
four starting at the right. Pad the leftmost group with zeros if
needed for display.

```text
bit positions                         7 6 5 4     3 2 1 0
byte representing decimal 166         1 0 1 0  |  0 1 1 0
value of each group                     10            6
hexadecimal digit                        A            6
```

*Figure 1. The high group gets weight 16 and the low group
gets weight 1. The two hex digits have value 10 × 16 + 6.*

A **nibble** is a group of four bits. Above, the high nibble
contains positions 7 through 4. The low nibble contains 3
through 0. Hex `A6` is a two-character notation for the byte,
often written with a prefix as `0xA6`. The notation can itself
be stored as text. In ASCII or UTF-8, the text `A6` then takes
two bytes, with values `0x41` and `0x36`. A parser needs a
format specification to know which form is expected.

An input boundary should also state which characters can appear.
A protocol field specifying two ASCII hex digits can permit
upper and lower case and reject a leading sign, a prefix, or
extra characters. Accepting malformed content and silently
choosing a substitute value can hide a data error. The exercise
format distinguishes malformed input from the valid byte zero.

## Byte order answers a different question

Suppose a message carries the 16-bit number `0x1234`. At the
bit level, the high group of eight positions has value `0x12`
and the low group has value `0x34`. In a **big-endian** layout,
the high byte is stored at the smaller address or appears first
in the field. In a **little-endian** layout, the low byte appears
first. The stored sequence `12 34` has different meanings under
the two interpretations.

| Interpretation of bytes `12 34` | Calculation | Value |
| --- | --- | --- |
| Big-endian | `0x12 × 256 + 0x34` | `0x1234` (4660) |
| Little-endian | `0x34 × 256 + 0x12` | `0x3412` (13330) |

Choosing byte order does not reverse the digits within each byte.
Choose an order explicitly at a file or network boundary. An
implementation that accidentally relies on its host layout can
read its own output successfully while disagreeing with another
program. Use a specimen with distinct bytes when checking the
format. The sequence `12 12` would hide an order reversal.
For an example of a documented multibyte field order, see
[RFC 791, Appendix B](https://www.rfc-editor.org/rfc/rfc791).

## Carry and overflow

To add 1 to `00001111`, begin at the right. The first column
adds 1 and 1, gives a low bit of 0, and passes a carry of 1 to
the next column. The next three columns each receive that carry
and already contain a 1. They again emit 0 and pass a carry.
At position 4 the carry meets two zeros, so position 4 emits 1
and stops the chain. The result is `00010000`, or 16.

At each column, the small total is the sum of the two input bits
and the carry coming in. The total ranges from 0 through 3.
The low bit is its remainder on division by 2, and the carry out
is the quotient. That rule also explains the last column: a
carry from position 7 has weight 256. If the result is stored
in one byte, that carry must be discarded or reported somewhere
else. **Overflow** here means that the unsigned mathematical sum
exceeds the largest representable unsigned value.

| Operands | Ordinary sum | Stored byte | Carry out |
| --- | ---: | ---: | ---: |
| 15 and 1 | 16 | 16 | 0 |
| 255 and 0 | 255 | 255 | 0 |
| 255 and 1 | 256 | 0 | 1 |
| 250 and 20 | 270 | 14 | 1 |
| 255 and 255 | 510 | 254 | 1 |

A **wrapping** interface returns the stored low part and discards
the carry. A **checked** interface represents failure when the
ordinary result does not fit. A **saturating** interface caps the
result at the maximum. A two-part interface can return both the
low byte and the carry, which preserves enough information to
reconstruct any sum of two bytes. A timer with a specified
wraparound may use the first rule. A bounded count that must be
accurate can use a checked sum and a larger representation or
report an error. Capping is useful only when the specification
assigns meaning to the cap. A capped value can no longer
represent how far the addition went over the limit.

### A proof of column addition

{{#include ../../proofs/C002-ripple-carry.md}}

## Practice and assessment

{{#include ../../exercises/C002.md}}

## Summary and further reading

Positional notation gives each bit a weight. Division recovers
the weights from low to high, and hexadecimal displays the same
bits in groups of four. A multibyte field also needs a specified
byte order. A byte addition can lose a high carry, and the
interface must specify what happens in that case.

The next chapter derives Boolean rules for each addition column.
Rust's numeric chapter later connects these contracts to concrete
operations and compiler settings.

[Previous: How computers execute instructions](001-machine.md) · [Solutions](../foundations-solutions.md#chapter-002) · [Next: Boolean algebra and digital logic](003-logic.md)
