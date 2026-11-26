# 036. Pure functions and composition

A command prints a price quote. To check that quote, you
want to test the arithmetic without redirecting standard
output or waiting for another component. The key design
separates the transformation from the effect that publishes
it. The calculation can then be composed, checked, and
reused on its own.

**Prerequisites.** [Functions and control flow](../part-03/015-control-flow.md)
and [generic functions and parametric reasoning](../part-07/031-generics-and-parametricity.md).
After this chapter, you should be able to state an effect
boundary, justify composing pure transformations, and test
the result without depending on input/output.

## State which observations matter

A **pure function** computes a result from its inputs and
performs no externally observable effects in the model
being considered. A result obtained from a clock, a mutable
global counter, or an external input stream depends on more
than a function's written arguments. Sending data to an
external output also changes observations beyond the returned
value.

If a function is pure and returns a value for the given
arguments, replacing a call by that value preserves the
observable result in the same model. This property is
called **referential transparency**. For example, replacing
`base_price(3)` by 375 does not change the quote. Replacing
a printing call by its unit return value would skip the
output. Evaluation cost and allocation failure need separate
consideration if they are among the observable properties
being studied.

Rust's ordinary function syntax does not declare purity.
A function can perform output or panic even when its
signature takes and returns simple values. Conversely, a
function can use a local mutable accumulator without
exposing mutations outside the calculation. Review effects
from the implementation and its called operations.

## Compose two transformations

Suppose each item costs 125 cents. The service subtracts
one tenth of the subtotal, rounded down to whole cents.
The input is an unsigned 16-bit item count. The largest
pre-discount result is `65,535 × 125 = 8,191,875`,
so widening before multiplying is sufficient. The
discount is no larger than the subtotal, ensuring its
subtraction is safe.

```rust,edition2021
{{#include ../../examples/functional/src/quotes.rs}}

assert_eq!(base_price(3), 375);
assert_eq!(apply_discount(375), 338);
assert_eq!(quote(3), 338);
assert_eq!(receipt(3), "items=3, cents=338");
```

For input 3, the integer discount is 37 cents, so the
result is 338 cents. The composition is `discount ∘
base_price` in the notation of
[Chapter 7](../part-02/007-sets-and-graphs.md).
The output type of the first function matches the input
type of the second. Their policy order matters: taking a
discount from the item count first would apply rounding
to different quantities and can change the result.

The numerical functions use a fixed number of operations
on fixed-width integers. Receipt formatting produces
a String and can allocate for its text. The example
treats the returned bytes as the formatting result,
without using the allocation behavior as part of the
quote's arithmetic contract.

## Keep input/output at a chosen boundary

A complete tool can parse arguments, pass a validated
count to `quote`, build a receipt, and write the
receipt to a caller's destination. Tests of the arithmetic
do not need that destination. Tests of the output
boundary can instead verify the byte sequence and the
handling of write errors. The decomposition keeps the
meaning of input, computation, and publication explicit.

| Step | Input | Produced result |
| --- | --- | --- |
| Unit price | Validated count | Subtotal in cents |
| Discount | Subtotal | Quote in cents |
| Render | Count and quote | Receipt bytes |
| Write | Receipt bytes and output | An external write outcome |

In a larger system, pass relevant policy values rather
than reading a mutable global from the calculation.
If rates depend on a time of day, obtain that time at
a boundary and supply the chosen policy or time as
an argument. The resulting transformation can still be
tested independently. Testing deterministic operations
does not establish that the boundary supplied the right
policy. That needs its own evidence.

## Practice and assessment

{{#include ../../exercises/C036.md}}

## Summary

Identify which steps produce values and which affect
the outside world. Compose transformations in their
documented order and use tests that distinguish rounding
policies. Keep the effect boundary observable independently
of the pure calculation.

[PhantomData and lending](../part-07/035-markers-and-lending.md) · [Solutions](../functional-solutions.md#chapter-036) · [Closures and capture](037-closures-and-capture.md)
