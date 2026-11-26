# 023. Lifetimes, elision, and lifetime bounds

A search function can return a view of a matching
name without allocating a new string. The compiler
needs to know which input that result can refer to
and how long an invocation can let the caller use it.
Lifetime annotations express those relationships in
types. They add no storage to the references.

**Prerequisite.** [Shared and mutable borrowing](022-borrowing.md).
After this chapter, you should be able to read a
lifetime parameter, derive a function's elided signature,
and distinguish a static reference from a type that
satisfies a static bound.

## A lifetime annotation relates places where a borrow is valid

A **lifetime** is a region over which a reference must
remain usable. The notation `&'a str` describes a
shared string reference valid for a lifetime named
`'a`. A function declares a lifetime parameter with
syntax such as `fn name<'a>(...)`. Its body must
honor the relationship the signature states for every
valid choice of that parameter. The caller can then
use the result within the region that its inputs
support.

```rust,edition2021
fn choose<'a>(left: &'a str, right: &'a str, use_left: bool) -> &'a str {
    if use_left { left } else { right }
}

let first = String::from("red");
let second = String::from("blue");
assert_eq!(choose(&first, &second, false), "blue");
```

Either branch can supply the result. Both inputs must
be valid for the caller's selected `'a`, so each
branch meets the output promise. The references do
not have to be originally valid for the same length of
time. A shorter common region can satisfy the
relationship when the owners' scopes differ.

```text
outer source      [ --------------------------------------- )
inner source                 [ ----------------- )
shared result region           [ ---------- )
```

*Figure 1. A returned view that may come from either
source needs a region supported by both. The diagram
illustrates nested source scopes and a possible use
region within their overlap.*

If a function always returns its first input, its
signature can express a more precise contract. The
other input needs to be available while it is used,
without imposing a lifetime relationship on the
returned view.

```rust,edition2021
fn select_first<'a>(first: &'a str, other: &str) -> &'a str {
    assert!(!other.is_empty());
    first
}

let keep = String::from("red");
let result;
{
    let local = String::from("check");
    result = select_first(&keep, &local);
}
assert_eq!(result, "red");
```

The returned reference is supported by `keep`. The
shorter-lived `local` is used during the call but
does not support the return value. Trying to return
the second input under this signature would require
an additional relationship.

## Apply the elision rules

**Lifetime elision** is a set of rules for omitted
annotated lifetimes in signatures. It makes common
relationships concise while keeping the same type
constraints. For function signatures here, use these
rules in order.

1. Each elided lifetime in the parameters creates
   a distinct lifetime parameter.
2. If there is exactly one lifetime used in the
   parameters, assign that lifetime to each elided
   output lifetime.
3. For an applicable method with receiver `&Self`
   or `&mut Self`, assign the receiver lifetime
   to the elided output lifetimes.

`fn first_byte(text: &str) -> Option<&u8>` has one
input lifetime. Its output reference, when present,
uses the same one. An explicit spelling is
`fn first_byte<'a>(text: &'a str) -> Option<&'a u8>`.
The method rule supports views into an object when
its receiver supplies the lifetime. If multiple
nonreceiver input lifetimes are present and no rule
chooses an output lifetime, the compiler needs a
written annotation.
[Rust 1.75.0 Reference on lifetime elision](https://doc.rust-lang.org/1.75.0/reference/lifetime-elision.html).

**Compile-fail Rust.** The unannotated two-input return
gives error E0106 even though its body always returns
the first argument. The body does not supply missing
public signature information.

```rust,compile_fail,edition2021
{{#include ../../tests/fixtures/ambiguous_output_lifetime.rs}}
```

Giving every parameter the same annotation can be
needlessly restrictive. First identify which sources
each possible returned reference can use. Then choose
the weakest relationship that guarantees every output
path. A too-restrictive signature can reject a caller
whose desired use actually has a valid supporting source.

## Return an owner when no input can support a view

A local `String` created during a call generally
ceases to exist when the call completes unless ownership
is transferred. A reference into that local storage cannot
be used after the owner is destroyed. An annotation
describes which regions the reference already supports.
It cannot make the allocation or local object survive.

**Compile-fail Rust.** Rust 1.75.0 reports E0515
when the function tries to return a reference into
its local string.

```rust,compile_fail,edition2021
{{#include ../../tests/fixtures/return_local_reference.rs}}
```

```rust,edition2021
fn make_label() -> String {
    String::from("local")
}

let label = make_label();
assert_eq!(label, "local");
```

Returning the owned string transfers cleanup to the
caller and allows the text to remain available. Another
valid design can return a view into a supplied input
when that input contains the needed data. Decide
whether the result derives from memory the caller
owns or whether it needs new owned storage.

## Read outlives relationships

The bound `'long: 'short` means the first lifetime
**outlives** the second. A shared reference valid for
`'long` can then support an output required for
`'short`. The relationship is inclusive: they can
be the same. Bounds can also apply to types.
`T: 'a` requires the type `T` to be valid for
`'a`, accounting for the data it borrows. It places
a constraint on what the type can contain. It does
not require any particular value of that type to
stay alive through the entire region.
[Rust 1.75.0 Reference on lifetime bounds](https://doc.rust-lang.org/1.75.0/reference/trait-bounds.html).

```rust,edition2021
fn through<'short, 'long>(text: &'long str, _: &'short ()) -> &'short str
where
    'long: 'short,
{
    text
}

let text = String::from("available");
{
    let scope = ();
    assert_eq!(through(&text, &scope), "available");
}
```

The unit reference acts as a second lifetime input.
The bound permits returning a shorter view of the
text. No data move or lifetime extension occurs.
The important proof question is whether the
output's region is contained in the input's region.

## Distinguish two meanings of static

A reference of type `&'static str` is valid for
the static lifetime. A string literal provides an
example. The reference variable holding that view
can go out of scope sooner than the data it names.

The bound `T: 'static` has a different shape. It
says that the type can be used without relying on
short-lived borrowed data. An owned `String` can
satisfy this bound even if an individual string is
destroyed immediately afterward. By contrast, a
reference into a local string ordinarily cannot
be supplied to an interface that demands a static
reference.

```rust,edition2021
fn consume_static_value<T: 'static>(value: T) {
    drop(value);
}

let literal: &'static str = "fixed";
assert_eq!(literal, "fixed");
consume_static_value(String::from("short-lived owned value"));
```

The caller gains no automatic guarantee about
resource cleanup, ownership transfer, or whether
the function stores the value. Those are separate
parts of the interface. In this example the body
explicitly drops the value during the call.

## Practice and assessment

{{#include ../../exercises/C023.md}}

## Summary

Write a reference's relationship to the data that
can support it. Elision applies fixed signature
rules. An annotation cannot extend a local
allocation's availability. An outlives bound permits
shorter use where longer validity is established.
Read `&'static T` and `T: 'static` according
to their different contracts.

[Shared and mutable borrowing](022-borrowing.md) · [Solutions](../ownership-solutions.md#chapter-023) · [Variance, subtyping, and aliasing](024-variance-and-aliasing.md)
