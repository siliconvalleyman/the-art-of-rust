# 024. Variance, subtyping, and aliasing

A program can often shorten the guarantee attached to a
shared reference. The same substitution inside a writable
slot could let a caller store an address that does not meet
the slot's original promise. Variance describes when such
substitutions are available.

**Prerequisites.** [Sets, relations, and functions](../part-02/007-sets-and-graphs.md)
and [lifetime relationships](023-lifetimes.md).
After this chapter, you should be able to explain lifetime
subtyping, derive the difference between read and write
positions, and diagnose an invalid substitution.

## Subtypes and weaker guarantees

A type `S` is a **subtype** of `T` when a value of
type `S` can serve where type `T` is required under the
language's subtype relation. Here we consider Rust's
lifetime subtyping. Similar named fields do not by
themselves make Rust structs subtypes of one another.

If `'long` outlives `'short`, a shared reference
valid for `'long` can serve as a shared reference valid
for `'short`. The longer guarantee already includes the
shorter one. The substitution does not move the data or
change when its owner destroys it.

```rust,edition2021
fn shorter<'a>(text: &'static str, _: &'a ()) -> &'a str {
    text
}

let scope = ();
assert_eq!(shorter("permanent", &scope), "permanent");
```

The literal supports any region the marker can require.
Reversing the relationship would require a potentially
short-lived value to meet a stronger guarantee.

## Determine variance for each parameter

A **type constructor** builds a type from parameters.
For example, a shared reference uses a lifetime and
a referent type. **Variance** describes how a subtype
relation in one parameter affects the constructed type.

| Variance in one parameter | If S is a subtype of T |
| --- | --- |
| Covariant | The constructor applied to S is a subtype of the constructor applied to T |
| Contravariant | The constructor applied to T is a subtype of the constructor applied to S |
| Invariant | This subtype relation alone supplies no substitution between the constructed types |

A shared reference is covariant in its lifetime and
referent type. A mutable reference is covariant in the
lifetime of its own borrow and invariant in its referent
type. In `&'borrow mut &'data str`, the outer lifetime
describes how long the slot is borrowed. The inner
lifetime describes the type of string reference the
slot can store. A short loan can still carry a
strong promise about the stored reference.
[Rust 1.75.0 Reference on subtyping and variance](https://doc.rust-lang.org/1.75.0/reference/subtyping.html).

```text
&'borrow mut &'data str
    |            |
    |            +-- promise made by the reference stored in the slot
    +--------------- duration of the exclusive loan of the slot
```

*Figure 1. The two lifetime positions describe different
permissions. Shortening the loan need not change what a
write to the slot may store.*

A function pointer is contravariant in its input
types and covariant in its output type. A consumer
that handles a larger set of permitted inputs can serve
a caller that supplies a subset of them. In an output
position, returning a value with a stronger guarantee
satisfies the weaker output requirement. The argument is
about substituted interfaces and their permitted uses.

## A writable slot must preserve readers' promises

Suppose a slot has type `&mut &'static str`. The
caller can read a static reference from it after
the exclusive loan ends. If a callee could treat
that slot as `&mut &'short str`, it could write a
local reference into it. Afterwards the caller would
regain its original type, which makes an unsupported
static promise about the contents. The opposite
substitution could promise a stronger result on a
read than the original slot holds. Because the
referent type governs both reads and writes, it is
invariant.

**Compile-fail Rust.** In this example the explicit
static slot forces a guarantee that the local string
cannot meet. Rust 1.75.0 reports E0597.

```rust,compile_fail,edition2021
{{#include ../../tests/fixtures/invariant_slot.rs}}
```

One repair keeps the static requirement and uses a
literal. Another puts the local view in a slot with a
type that only requires validity over its actual use
region. In the example below, there is no explicit
static requirement on the slot.

```rust,edition2021
fn store<'a>(slot: &mut &'a str, value: &'a str) {
    *slot = value;
}

let source = String::from("sample");
let mut view = "initial";
store(&mut view, source.as_str());
assert_eq!(view, "sample");
```

The loan of the slot can end after the call. The
contents remain valid for the assertion because the
source is still available. A signature that supports
the correct use region avoids forcing callers to
duplicate or leak the text.

## Aliases are access paths

Two **aliases** give different paths to the same
region. Shared aliases to ordinary data can provide
concurrent read access. Accesses can conflict when
one path modifies or frees what another path relies
on. Borrowing checks ordinary shared and exclusive
permissions. Invariance additionally prevents changing
the lifetime requirements of writable storage through
a subtype substitution.

A shared view of a `Cell<T>` can write its contents
through a safe method. If the cell were covariant in
`T`, a shared reference to a cell promising
`&'static str` could be used as a shared reference
to a cell accepting a shorter reference. That view
could overwrite the cell, and another could read
its contents as static. The type's invariance
prevents that conversion. Being reached through a
shared reference does not make every operation on
the referent read-only.

These arguments concern selected safe interfaces.
An unsafe abstraction has further obligations for the
validity of pointers, initialization state, and
applicable access rules. The unsafe section revisits
them under the book's historical scope.

## Practice and assessment

{{#include ../../exercises/C024.md}}

## Summary

A stronger lifetime guarantee can satisfy a weaker
one. Variance determines whether the relationship
survives in a constructed type. A writable slot must
protect both what readers may assume and what writers
may install.

[Lifetimes, elision, and lifetime bounds](023-lifetimes.md) · [Solutions](../ownership-solutions.md#chapter-024) · [RAII, Drop, and resource invariants](025-raii-and-drop.md)
