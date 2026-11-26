# 031. Generic functions, data, and parametric reasoning

You need the first and last elements of a list. The same
operation makes sense for lists of readings, instructions,
or strings. The element type determines what those items
mean. The operation only needs their positions. A generic
signature can express that separation directly.

**Prerequisites.** [Structs and products](../part-06/026-structs-and-products.md)
and [Option and Result](../part-06/028-option-and-result.md).
After this chapter, you should be able to read a generic
signature, infer concrete type arguments, and derive which
operations its body may use.

## Separate structure from element behavior

A **generic parameter** stands for a type, lifetime,
or constant supplied by an instantiation. A generic
function can define one operation on many choices of type.
Making the element type a parameter instead of copying a
function for every concrete type is called **parametric
polymorphism**.

For a slice of arbitrary `T`, Rust knows the length and
can access its contained values under the borrow rules.
Without further constraints, the body cannot assume
`T` has a useful order, allows implicit copies, or can be
formatted for debugging. It can still move an owned `T`,
borrow one, or pass it to operations whose signatures
accept that unconstrained type.

```rust,edition2021
{{#include ../../examples/abstractions/src/generic.rs}}

assert_eq!(ends(&[3, 8, 5]), Some((&3, &5)));
assert_eq!(ends::<u8>(&[]), None);
let words = [String::from("start"), String::from("end")];
let (first, last) = ends(&words).unwrap();
assert_eq!((first.as_str(), last.as_str()), ("start", "end"));
```

`ends` borrows elements from its input. Its single
input lifetime supports both returned references.
`first()?` stops the operation when the slice is empty.
Otherwise the slice has a last element as well.
For a singleton it is valid for both shared references
to refer to the same element. There is no requirement
to duplicate or compare any value of type `T`.

The input slice often tells the compiler what `T`
must be. The string array constrains the element type
to String. An empty array literal supplies no element
from which to infer the type, so the example uses
`::<u8>` to state it. Expected result types and other
uses can also constrain a type argument.

## Reason universally about the function body

The signature promises the same endpoint rule for
every type that can instantiate the function. For
empty input, the first probe yields absence, as
required. For nonempty input, the first and last
probes identify indices 0 and `len − 1`.
Each is valid by `len > 0`. The probes make a fixed
number of slice operations and allocate no storage.
The behavior depends on the arrangement of values,
not a particular element comparison.

This analysis also identifies the operations the
signature leaves unavailable. Attempting to print the
value using debug formatting imposes a new requirement.
Calling the function with an integer does not provide
that requirement inside the generic definition.

**Compile-fail Rust.** The debug formatter requires
the Debug capability. Rust 1.75.0 reports E0277 because
the unconstrained signature promises to accept every `T`.

```rust,compile_fail,edition2021
{{#include ../../tests/fixtures/generic_missing_debug.rs}}
```

You can either add a bound to require debug formatting
or change the function so it never formats the element.
The choice belongs to the contract. Adding a requirement
just to inspect intermediate values can exclude legitimate
callers from an operation that otherwise has no need for
that capability. Chapter 32 explains how traits express
such bounds.

## Relate multiple parameters

A generic data type can use multiple distinct type
parameters. `Pair<A, B>` stores an A and a B.
The compiler can check that operations on the left and
right preserve those identities even when each choice
differs. An impl declares parameters before using them
in its implementing type.

```rust,edition2021
{{#include ../../examples/abstractions/src/generic.rs}}

let original = Pair::new(String::from("label"), 9_u8);
let reversed: Pair<u8, String> = original.reversed();
assert_eq!(reversed.into_parts(), (9, String::from("label")));
```

Reversal consumes the original pair. Its second field
becomes the first, and the result type changes to
`Pair<B, A>`. Neither field needs Copy or Clone.
A function that passed both values into a
`Pair<T, T>` would instead ask them to agree on
one type. Reusing the same parameter is a constraint,
and distinct parameters permit independent choices.

Parametric reasoning can predict identities above the
representation. Reversal twice changes `Pair<A, B>`
to `Pair<B, A>` and back, preserving each component.
An implementation that duplicated one field in place
of the other would fail to fulfill the generic type for
independent `A` and `B`. The functional claim also
depends on reading the implementation: types alone do
not guarantee arbitrary functions terminate or perform
no side effects.

## Put a constant in a type when it matters

An array type includes its length. The
`array_count<T, const N: usize>` signature uses a
**const generic** parameter for that length. A borrowed
`[u8; 3]` fixes `T = u8` and `N = 3`.
A borrowed `[u8; 0]` also has an accepted length.
The function can return `N` without walking elements.

When an algorithm must accept views whose sizes are
known only at runtime, it can take a slice instead
and use its length. Choosing a fixed length in a
signature can express a required layout or permit
compile-time checking of callers. It can also create
more concrete instantiations. Code generation and the
cost of specialization are discussed in Chapter 34.

## Practice and assessment

{{#include ../../exercises/C031.md}}

## Summary and further reading

Use a generic parameter for the choices a caller
supplies. Justify a body's operations from the
signature and its bounds. Distinguish independent
parameters from repeated ones, and use a runtime
collection length where a fixed type-level length is
not part of the contract.
[Rust 1.75.0 Reference on generic parameters](https://doc.rust-lang.org/1.75.0/reference/items/generics.html).

[Typestate and transitions](../part-06/030-typestate-and-transitions.md) · [Solutions](../abstraction-solutions.md#chapter-031) · [Traits and associated items](032-traits-and-associated-items.md)
