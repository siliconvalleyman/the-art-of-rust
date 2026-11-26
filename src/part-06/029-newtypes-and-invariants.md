# 029. Newtypes and type-driven API design

One parcel scale reports milligrams. A dispatch rule takes
whole grams and accepts at most 30,000 of them. If both
quantities have type `u32`, the compiler can accept the
wrong unit. A separate risk remains even after assigning
the right unit: some quantities exceed the dispatch rule.
The two mistakes need different checks.

**Prerequisites.** [Structs and products](026-structs-and-products.md),
[Option and Result](028-option-and-result.md), and
[lifetime relationships](../part-05/023-lifetimes.md).
After this chapter, you should be able to distinguish a
unit type from a validated domain type, identify all
construction paths, and prove an operation preserves its
type's invariant.

## Express units with different type identities

A **newtype** is a distinct type that wraps one other
value. Rust's tuple struct is a common way to implement
one. A type alias written `type Grams = u32` gives a
second name to the same type. A tuple struct instead
creates its own identity. Rust checks that identity at
calls and assignments.

**Compile-fail Rust.** The scale's milligrams cannot be
passed as grams merely because both wrappers contain a
`u32`. Rust 1.75.0 reports E0308 at the call.

```rust,compile_fail,edition2021
{{#include ../../tests/fixtures/mixed_weight_units.rs}}
```

An explicit conversion must state its rounding policy.
The value 4,000 mg is exactly 4 g. For 4,500 mg, the
interface could round down, round to the nearest whole
gram, or reject a fractional number of grams. Those
choices are observable around limits. Use the one the
domain requires. Extracting the integer and repackaging
it without a conversion merely moves the error into a
different line of source.

## Establish a predicate once at construction

`Grams` answers the unit question and allows all
`u32` values. `ParcelWeight` represents a second
contract. Define the admissible set
`P = { g ∈ [0, u32::MAX] | 1 ≤ g ≤ 30,000 }`.
A constructor should return a value of `ParcelWeight`
exactly for inputs in `P`. The inner field is private,
so callers in other modules must use an exposed
construction path.

```rust,edition2021
{{#include ../../examples/data-modeling/src/weight.rs}}

let parcel = ParcelWeight::new(Grams::new(29_900)).unwrap();
assert_eq!(parcel.with_packaging(Grams::new(100)).unwrap().grams(), Grams::new(30_000));
assert_eq!(parcel.with_packaging(Grams::new(101)), Err(WeightError::TooHeavy));
```

The match rejects zero and then any value above the upper
bound. The successful branch therefore establishes both
inequalities. Conversely, every member of `P` reaches
that branch. The type implements Copy, so reading the
underlying quantity or trying a packaged value does
not invalidate the prior value. A copy of a valid value
remains valid. The implementation exposes no method
that lends a mutable reference to the wrapped integer.

The constructor can protect the contract only if every
other path respects it. For example, adding a public field
or a method returning `&mut Grams` would let the caller
write zero into a previously valid instance. A decoding
path must call the same validation logic or establish the
same property itself, including when decoding older data.

## Preserve the invariant in another operation

Packaging is an unvalidated nonnegative number of grams.
Let `g` be the original weight, which already satisfies
`1 ≤ g ≤ 30,000`, and let `p` be the packaging mass.
If their mathematical sum overflows `u32`, it necessarily
exceeds 30,000. The checked addition can therefore report
the same domain error as an in-range sum that exceeds
the service limit. If the sum fits, the constructor
tests the business constraint. The original value remains
usable if creating a new weight fails.

The check uses a fixed number of integer operations and
allocates no memory. A useful boundary test uses 29,900
as the original weight and adds 100 or 101. The first
fits exactly and the second violates the business limit
without exceeding `u32`. Adding `u32::MAX` exercises
the different arithmetic failure path.

There are other valid models. An API receiving an
external untyped value can validate it at every call.
A private validated type lets later operations rely on
a documented checked boundary. A newtype has no need
to allocate merely to store a small scalar; its default
representation still should not be treated as an ABI
promise. The language has a separate `repr(transparent)`
rule for cases that require the wrapper to have a
field's layout and ABI under its stated conditions.

## Practice and assessment

{{#include ../../exercises/C029.md}}

## Summary and further reading

Use a distinct wrapper when two equally represented values
have different meanings. State admissible values precisely,
validate all construction routes, and keep update operations
inside the invariant. Treat unit conversion and domain
validation as separate decisions.
[Rust 1.75.0 Reference on type aliases](https://doc.rust-lang.org/1.75.0/reference/items/type-aliases.html) · [Type layout](https://doc.rust-lang.org/1.75.0/reference/type-layout.html).

[Option, Result, and domain states](028-option-and-result.md) · [Solutions](../data-modeling-solutions.md#chapter-029) · [Typestate and state transitions](030-typestate-and-transitions.md)
