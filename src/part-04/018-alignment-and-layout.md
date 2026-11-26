# 018. Alignment, padding, endianness, and layout

An in-memory record contains a tag, a count, and flags.
Their types add up to seven bytes of values. On the target
measured in this chapter, the record occupies twelve bytes.
A file format for the same data can use exactly seven.
The difference comes from two contracts: how typed values
are placed in memory, and how a sequence of bytes is
exchanged with another system.

**Prerequisites.** [Numeric conversions](../part-03/013-numeric-conversions.md),
[array and slice representation](../part-03/014-types-and-text.md),
and [addresses and frames](016-addresses-and-stack-frames.md).
After this chapter, you should be able to compute offsets
under a stated layout rule, verify a target measurement,
and encode or decode a portable byte format.

## Alignment constrains an address

The **alignment** of a type is a positive power of two.
A value with alignment `a` needs an address divisible
by `a` for an ordinary typed access. For a value whose
alignment is four, address `0x1004` meets the rule and
`0x1002` does not. The **size** is the separation in
bytes between consecutive elements of an array of that
type. A small field can have a lower alignment than the
whole record to which it belongs.

The default Rust representation makes certain placement
guarantees, but it does not promise source field order
for an ordinary struct. A **struct** groups values into
named fields. Here we use `#[repr(C)]` to choose the
documented C representation for the struct. With fields
in declaration order, the algorithm rounds each field's
current offset up to its alignment, then adds its size.
The struct's alignment is the greatest field alignment.
Finally the total size is rounded up to that alignment.
The rules use field sizes and alignments determined on
the target.
[Rust Reference on type layout](https://doc.rust-lang.org/1.75.0/reference/type-layout.html).

To round an offset `x` up to a multiple of `a`, find
its remainder `r` on division by `a`. If `r = 0`,
the offset is already allowed. Otherwise add `a−r`.
That is the least positive increment reaching a multiple
because every smaller increment keeps the remainder
strictly between zero and `a`. In code, validate the
alignment and check the addition. A formula that first
computes `x+a−1` can overflow even when the original
offset was already aligned.

## Derive one record's offsets

Use the target assumptions that `u8`, `u32`, and
`u16` have sizes and alignments (1, 1), (4, 4), and
(2, 2). The tag starts at offset 0 and ends before
offset 1. The count needs a multiple of four, so it
starts at offset 4, leaving three bytes of **padding**.
After four bytes of count, flags begin at offset 8 and
occupy two bytes. The end offset is 10. Rounding it
up to a multiple of four adds two bytes of tail padding.

| Region | Starting offset | Bytes |
| --- | ---: | ---: |
| tag | 0 | 1 |
| padding | 1 | 3 |
| count | 4 | 4 |
| flags | 8 | 2 |
| tail padding | 10 | 2 |

*Figure 1. The measured layout stores seven field bytes
in a twelve-byte stride. Tail padding lets an array's
next record begin at the required alignment.*

For an array whose first record starts at a multiple
of four, adding twelve to get the second start preserves
that alignment. If the stride were 10, an aligned
first record would give an unaligned next record. Tail
padding is therefore part of the stride and contributes
to the storage used by an array. Under these same
assumptions, ordering the fields as count, flags, tag
gives offsets 0, 4, and 6, an ending offset of 7, and
size 8 after tail padding. That can matter when creating
many records. Changing an existing external layout
must be coordinated with all consumers of that layout.

## Measure, and record what was measured

**Executable Rust.** Rust supplies `size_of` and
`align_of` for measurements on the compiled target.
The program also obtains raw field addresses without
reading through raw pointers. Subtracting the address
of the containing value gives each field's numeric
offset within this object.

```rust,edition2021
{{#include ../../examples/memory/record_layout.rs}}
```

The command below was run from the repository root
using Rust 1.75.0, commit `82e1608df`, with target
`x86_64-pc-windows-gnu` and optimization level zero.

```powershell
rustc +1.75.0 --edition=2021 --target x86_64-pc-windows-gnu -C opt-level=0 examples/memory/record_layout.rs -o record_layout.exe
.\record_layout.exe
```

**Captured standard output.**

```text
size=12, alignment=4, tag=0, count=4, flags=8
```

The result agrees with the offset derivation for this
target. The code predicts relative positions and
compares neither full addresses across runs nor the
contents of padding bytes. Padding can be left
uninitialized. Reading or transmitting an entire raw
structure as if it were a sequence of initialized
fields can therefore be incorrect and can expose
unintended storage contents. Encode the intended fields
explicitly when defining an exchange format.

## Specify byte order at an exchange boundary

A portable record can be exactly seven bytes: one tag,
four count bytes in big-endian order, and two flag
bytes in big-endian order. In this format, count
`0x01020304` is written as `01 02 03 04`, regardless
of the sender's native order. Rust's `to_be_bytes`
and `from_be_bytes` express the corresponding library
operations. “Big-endian” refers to the placement of
the most significant byte.

```rust,edition2021
fn encode(tag: u8, count: u32, flags: u16) -> [u8; 7] {
    let [a, b, c, d] = count.to_be_bytes();
    let [e, f] = flags.to_be_bytes();
    [tag, a, b, c, d, e, f]
}

fn decode(bytes: &[u8]) -> Option<(u8, u32, u16)> {
    let [tag, a, b, c, d, e, f] = bytes else {
        return None;
    };
    Some((
        *tag,
        u32::from_be_bytes([*a, *b, *c, *d]),
        u16::from_be_bytes([*e, *f]),
    ))
}

let expected = [0x7f, 1, 2, 3, 4, 0xa1, 0xd2];
assert_eq!(encode(0x7f, 0x01020304, 0xa1d2), expected);
assert_eq!(decode(&expected), Some((0x7f, 0x01020304, 0xa1d2)));
assert_eq!(decode(&expected[..6]), None);
```

The pattern in `decode` checks length before reading
any field. It binds a shared reference for each byte,
and the dereferences copy the byte values into fixed
arrays. Interpreting the count reconstructs the sum
of the bytes with successive weights `256^3`,
`256^2`, `256`, and 1. The flags use weights
256 and 1. Each field encoder emits exactly those
positional digits, so decoding its output recovers
the input for all values in the field domains.

This example does bounded work and needs no heap
allocation. It validates length and representation.
If the application permits only specific flag bits,
check those as an additional contract before using
the record. Swapping encode and decode order together
could pass a round trip on the same machine. The
explicit expected specimen tests the external byte
order independently.

## Practice and assessment

{{#include ../../exercises/C018.md}}

## Summary

Derive layout from size, alignment, and a stated
representation rule. Compare offsets on a documented
target. At a boundary, choose field sizes and byte
order explicitly and serialize only defined values.

[Stack, heap, static storage, and destruction](017-storage-and-destruction.md) · [Solutions](../memory-solutions.md#chapter-018) · [Hierarchy, allocation, fragmentation, and locality](019-allocation-and-locality.md)
