# 033. Coherence, orphan rules, and blanket implementations

Two libraries both want to add a display format to a type
they did not create. If Rust let both independently define
the same trait for that type, a caller could encounter two
incompatible implementation choices. Rust's coherence rules
prevent invalid and overlapping implementations. The rules
also guide where to put a customization.

**Prerequisites.** [Newtypes and invariant boundaries](../part-06/029-newtypes-and-invariants.md)
and [traits and associated items](032-traits-and-associated-items.md).
After this chapter, you should be able to identify an orphan
implementation, show overlap with a concrete witness, and
choose a local extension trait or wrapper.

## Ask who defines the participating items

A crate is the compilation unit that defines items. In
these examples, a type or trait is **local** when it is
defined in the crate doing the implementing. An
**orphan rule** determines whether that crate is allowed
to provide the implementation in the first place.

A local trait can be implemented for a foreign type,
subject to the other coherence rules. A local type
can provide a foreign trait implementation in a
straightforward case. When neither the trait, the
implementing type, nor any trait input is local, the
usual attempt is rejected. Generic and fundamental
type cases use the more precise rules in the
reference. In particular, the reference also constrains
where uncovered type parameters can occur before a
local input. A type alias does not make a foreign
type local.

**Compile-fail Rust.** Both Display and the outer
Vec type below come from the standard library.
Rust 1.75.0 reports E0117 for this orphan
implementation.

```rust,compile_fail,edition2021
{{#include ../../tests/fixtures/orphan_display.rs}}
```

Public ownership of a value and definition of its
type are separate. Creating a Vec does not give the
crate authority to define Display for that Vec type.

## A local extension and a local wrapper solve different needs

```rust,edition2021
{{#include ../../examples/abstractions/src/coherence.rs}}

assert!("fragile".has_label());
assert!(!"  ".has_label());
assert_eq!(DispatchId(7).to_string(), "dispatch-7");
assert_eq!(7_u32.to_string(), "7");
```

The local `HasLabel` trait adds a method to strings
when the trait is in scope. It does not change Rust's
standard string operations. If another trait defines
an accessible method with the same name, qualified
syntax such as `HasLabel::has_label(text)` can
name the intended operation.

The wrapper gives the program a new local type and
lets that type implement the standard Display trait.
Formatting a plain `u32` retains its usual behavior.
Callers can choose the wrapper at the point where
the distinct formatting contract is required.
Writing the format performs work proportional to
the resulting text under a per-byte output model.
Creating the wrapper itself is independent of the
formatting operation.

## Identify overlap with a witness

A **blanket implementation** covers a family of
types described by parameters and bounds. The rule
`impl<T: Display> Summary for T` covers every
type meeting that Display bound. A second rule
`impl Summary for String` would cover String.
String satisfies Display, so it belongs to both
sets. That concrete witness proves overlap.

**Compile-fail Rust.** The second implementation
conflicts with the blanket implementation and
produces E0119 with Rust 1.75.0.

```rust,compile_fail,edition2021
{{#include ../../tests/fixtures/overlapping_summary.rs}}
```

Adding more methods or a more specific format to
one implementation cannot remove the overlap.
Both still implement the same trait for the same
type. Rust 1.75.0 stable does not allow treating
the concrete rule here as a specialization of
the blanket rule. One repair identifies two
disjoint wrapper types and implements the
behavior for those types. Another uses different
trait methods or traits for the different tasks.

Blanket implementations are useful when the same
behavior follows from a smaller capability. The
standard library's conversion and formatting
interfaces use such rules. Before adding one
to a public interface, consider which types it
covers and which special-purpose implementations
would then overlap. Coherence can reject overlaps
that are possible under the applicable crate and
trait rules, even when no example caller currently
uses a type at the intersection.

## Review the decisions separately

First decide whether the declaration is eligible under
the orphan rule. Then compare it with other trait
implementations. Eligibility alone proves nothing
about whether a second implementation covers the same
type. A local trait allows both declarations in the
overlap specimen to be considered. The witness String
then exposes the conflict.

At a review boundary, ask whether the customization
needs to integrate with an external trait. A wrapper
lets it do that while keeping the previous behavior
available on the wrapped type. If a helper is useful
on multiple existing foreign types, a local extension
trait can express it without changing their identity.

## Practice and assessment

{{#include ../../exercises/C033.md}}

## Summary and further reading

Coherence limits implementations to choices that
can remain unambiguous. The orphan rule governs
eligibility. Overlap concerns the intersection of
applicable implementations. Use the domain's type
boundaries to organize distinct behaviors.
[Rust 1.75.0 Reference on trait implementation coherence](https://doc.rust-lang.org/1.75.0/reference/items/implementations.html#trait-implementation-coherence).

[Traits and associated items](032-traits-and-associated-items.md) · [Solutions](../abstraction-solutions.md#chapter-033) · [Dispatch and trait objects](034-dispatch-and-trait-objects.md)
