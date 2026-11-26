# 032. Traits, associated items, and supertraits

A dispatcher totals weight readings. Some readings come
from a scale and others are computed from a stored record.
The dispatcher needs a way to obtain the amount without
depending on the representation of each source. It also
must reject volume readings at the call boundary. A trait
can describe the common operation and preserve the unit.

**Prerequisite.** [Generic functions, data, and parametric reasoning](031-generics-and-parametricity.md).
After this chapter, you should be able to read a trait
bound, constrain an associated type, and justify a default
method and a supertrait relationship.

## State the capability that a function relies on

A **trait** declares a set of associated items, such as
methods, types, and constants. A trait implementation
supplies those items for a concrete implementing type or
for types described by a generic rule. A **trait bound**
requires that an argument have an implementation.

An amount method alone would not carry information about
units. An associated type can state which unit belongs
to an implementation. A trait can also expose an
associated constant such as a textual unit label.
The constant belongs to its implementing type.

```rust,edition2021
{{#include ../../examples/abstractions/src/reading.rs}}

assert_eq!(total_grams(&[Weight(4), Weight(6)]), Some(10));
assert_eq!(Weight::LABEL, "g");
assert_eq!(Volume::LABEL, "mL");
assert!(Volume(0).is_zero());
```

Each `Reading` implementation supplies a `Unit`
type, a `LABEL` constant, and a `value` method.
The default body of `is_zero` calls `value` and compares
its result. An implementation may override a default
body. The example's semantic contract for `is_zero`
is agreement with `value() == 0`. Rust checks which
items are provided and types how they are called.
It does not prove an implementation's numerical
or semantic claims.

`R: Reading<Unit = Grams>` requires two things. R
implements Reading, and that implementation's associated
unit equals Grams. The compiler uses the associated
type equality without needing a runtime unit label.
The keyword `type` inside the trait declaration names
an associated choice. It differs from a separately
chosen function parameter. An implementation for an
ordinary nongeneric trait and concrete type chooses the
associated type for that implementation.

## Add stronger capabilities with a supertrait

`BoundedReading: Reading` means a type implementing
BoundedReading must also implement Reading. Reading
is its **supertrait**. A default method in
BoundedReading can therefore call the Reading method
`value`. It combines that method with the new
associated constant `MAX`.

```rust,edition2021
{{#include ../../examples/abstractions/src/reading.rs}}

fn admissible<R: BoundedReading>(reading: &R) -> bool {
    reading.is_within_limit()
}

assert!(admissible(&Weight(30_000)));
assert!(!admissible(&Weight(30_001)));
```

Holding a `BoundedReading` bound is enough to call
the Reading behavior. The reverse is unavailable:
Volume demonstrates a Reading whose implementation
does not declare a bound through this second trait.
A generic function that only measures the readings
should ask for Reading and let more types satisfy its
contract. A function processing a policy limit has a
reason to require the stronger capability.

The unit marker and limit describe different properties.
All Weight values implement BoundedReading, even if
a particular observed value is oversized.
`is_within_limit` returns a Boolean after checking the
value. A constructor-validated newtype would instead
prevent representing a value outside the accepted range
through its intended public API.

## Prove an aggregation against the interface

The aggregation starts with zero, the sum of the
empty prefix. After processing any accepted prefix,
`total` is its arithmetic sum in grams. Each associated
unit is Grams. Checked addition either extends the
sum exactly or returns absence on overflow. Since
all amounts are nonnegative, once the mathematical
prefix exceeds `u32::MAX`, the full sum cannot later
fall back into range. Failure thus agrees with the
contract for the whole slice.

The loop calls `value` at most once per element
and uses constant additional accumulator storage.
Actual complexity also depends on the implementation
of `value`. The two examples return a field, but
a trait method may perform other work unless its
contract constrains it. The bound allows calls to
the method. It does not set the method's cost or
force a specific machine-level dispatch strategy
in every use.

## Practice and assessment

{{#include ../../exercises/C032.md}}

## Summary and further reading

A trait describes common operations and associated
choices. Bounds admit only implementations with the
required capabilities, and an associated type equality
can preserve units. A supertrait adds a prerequisite
capability. Test semantic laws that Rust's type checker
does not establish.
[Rust 1.75.0 Reference on traits](https://doc.rust-lang.org/1.75.0/reference/items/traits.html) · [Trait bounds](https://doc.rust-lang.org/1.75.0/reference/trait-bounds.html).

[Generics and parametricity](031-generics-and-parametricity.md) · [Solutions](../abstraction-solutions.md#chapter-032) · [Coherence and orphan rules](033-coherence-and-orphan-rules.md)
