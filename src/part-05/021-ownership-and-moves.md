# 021. Ownership, moves, Copy, and Clone

A function builds a message with heap-owned text and
hands it to a queue. A second line then tries to use the
original message again. To decide whether that use is
legal, Rust needs to know what the handoff meant. Did
the function receive the only owner of the message, a
separate copy, or temporary access to the original?
Those choices determine who can use the value and who
destructs it.

**Prerequisites.** [Functions and calls](../part-03/015-control-flow.md)
and [storage and destruction](../part-04/017-storage-and-destruction.md).
After this chapter, you should be able to trace a move
through variables and calls, predict which uses Rust
rejects, compare Copy and Clone, and account for
individual fields after a partial move.

## Start with one resource and a cleanup obligation

When a value owns a resource, its destruction can clean up
that resource. A `String` controls the storage for its
UTF-8 buffer. When its current owner is destroyed, Rust can
release that storage. This chapter follows normally exiting
scope and transfer paths with one `String`. Shared owners,
intentional leaks, and abnormal termination require their
own reasoning, developed later.

Suppose a language let an assignment produce a second
independent owner of the same buffer without coordination.
Either name might release the buffer first. The other
could access released storage or try to release it again.
One way to keep the resource contract is to transfer the
ownership obligation with the value. After transfer, the
source is unavailable as a value and the destination has
the obligation.

Rust calls this transfer a **move**. For a type such
as `String`, using a local variable by value in an
assignment moves its value. Later using the source requires
reinitialization, or a different design that keeps it.
[Rust Reference on moved and copied values](https://doc.rust-lang.org/1.75.0/reference/expressions.html#moved-and-copied-types).

```rust,edition2021
let staged = String::from("ready");
let queued = staged;
assert_eq!(queued, "ready");
```

| Program point | Source `staged` | Destination `queued` | Owner of buffer |
| --- | --- | --- | --- |
| After the first statement | initialized | outside scope | staged |
| After the second statement | unavailable until reinitialized | initialized | queued |
| After ordinary scope exit | ended | destroyed | allocation released |

*Figure 1. Moving the string changes which binding may
use it and carries the cleanup obligation with the value.*

A useful invariant for this example is that each live
owned buffer has exactly one initialized owner. Creating
the string establishes it. The move removes the source's
ownership and installs the destination's ownership as one
language transition. Normal destruction of that owner
releases the resource. If a later access still used the
source as an initialized value, it would violate the
invariant that licenses this reasoning.

The move describes language-level behavior. Rust does
not promise that the processor copies any particular
number of bytes or chooses new addresses for bindings.
Semantically moving a `String` transfers the handle and
ownership of its buffer without cloning the text. The
compiler can eliminate physical operations in a build
while preserving what the program observes.

## Read the diagnostic as a state transition

**Compile-fail Rust.** The reference compiler rejects
the use of `staged` after the move with E0382.

```rust,compile_fail,edition2021
{{#include ../../tests/fixtures/use_after_move.rs}}
```

Find the diagnostic's source declaration, its move site,
and the later use. Draw a line through the old binding
at the move site. Afterward its type still appears in the
source, but it no longer contains an initialized value.
Use the new binding when there should be one owner.
Choose a clone when two independent values are required.
Use a borrow when the call needs only limited access
and the original owner should remain available. Chapter 22
introduces that third choice.
[Rust error code E0382](https://doc.rust-lang.org/1.75.0/error_codes/E0382.html).

Assigning a new value to the moved-out binding can make it
usable again. A binding that was initialized before the
move still needs `mut` to accept a second assignment.
Removing `mut` below causes E0384 on Rust 1.75.0.
Giving the source a new value starts a separate lifetime
for that value and leaves the earlier resource with
its new owner.

```rust,edition2021
let mut message = String::from("first");
let sent = message;
message = String::from("second");
assert_eq!(sent, "first");
assert_eq!(message, "second");
```

## Move through a function boundary

A function parameter that takes a `String` takes its
value and receives ownership. The return expression can
transfer that ownership to another caller-owned name.
Observe where the value exists at each point.

```rust,edition2021
fn add_exclamation(mut text: String) -> String {
    text.push('!');
    text
}

let draft = String::from("ready");
let finished = add_exclamation(draft);
assert_eq!(finished, "ready!");
```

At entry, the parameter owns the string. It is mutable
inside the function so the append can modify the value.
The append may need additional capacity for the new bytes.
Returning the parameter transfers the resulting buffer
handle. The original `draft` remains moved-out. If the
function instead returned unit and kept no other owner,
its local string would normally be destroyed before
the call completed.

This is useful in API design when the function consumes a
work item, replaces its contents, or owns how it will be
stored. A function that merely counts the bytes could
take a text view instead, avoiding a transfer and showing
that the caller retains ownership.

## Compare implicit Copy with explicit Clone

The `Copy` trait marks types for which using a value
by value from an initialized place leaves the original
available. A **trait** is a named language interface
or capability associated with a type. Rust implements
`Copy` for ordinary integers and several small value
types. The copied value can be used independently for
an integer. Some kinds of pointers also implement
`Copy`, and copying those pointer values does not
copy the underlying object.

```rust,edition2021
let first = 42_u16;
let second = first;
assert_eq!((first, second), (42, 42));
```

Size alone does not determine Copy. A type with a
destructor defined through `Drop` cannot be Copy,
and a composite type's fields must all satisfy Copy
to define that behavior. A large fixed array of Copy
elements can itself be Copy. A physical copy of it can
be more expensive than transferring a small handle,
depending on its length and optimization.
[Rust standard library documentation for Copy](https://doc.rust-lang.org/1.75.0/std/marker/trait.Copy.html).

`Clone` is an explicit request written as `.clone()`.
The actual behavior depends on the type. Cloning a
`String` copies its contents into an independently
owned string. Cloning some shared handles can preserve
the same underlying resource by extending shared
ownership. Read the API's cost and aliasing contract for
the particular type.

```rust,edition2021
let mut editable = String::from("draft");
let archived = editable.clone();
editable.push('!');
assert_eq!(editable, "draft!");
assert_eq!(archived, "draft");
```

The two strings allow independent edits. The test compares
the text after modifying only the original. A clone of a
nonempty string copies data proportional to its byte
length and can allocate. A move does not request this
copy. An unnecessary clone may become costly when data
is large. Use a clone when the caller's contract actually
requires independent owned text.
[Rust standard library documentation for Clone](https://doc.rust-lang.org/1.75.0/std/clone/trait.Clone.html).

## Account for transfers field by field

A `(String, String)` tuple has two owning fields.
Moving one makes the whole tuple unavailable as a complete
value. The other field can still be used individually.
For a type whose destructor requires the whole value
through its own `Drop` implementation, Rust rejects
moving a non-Copy field directly out of that value.
A complete owner can offer an explicit consuming
operation with a valid cleanup contract.

```rust,edition2021
let entry = (String::from("sensor"), String::from("active"));
let label = entry.0;
assert_eq!(label, "sensor");
assert_eq!(entry.1, "active");
```

Ownership now has two separate locations: `label`
owns the first buffer and the surviving field owns the
second one. Ordinary scope exit destroys each remaining
initialized owner. A function expecting the complete
tuple cannot use `entry` after the partial move.
Replacing the moved field, when permitted, can restore
the whole value. Keeping an ownership trace per field
helps diagnose why a use of one field is legal and a
whole-object use is rejected.
[Rust error code E0509](https://doc.rust-lang.org/1.75.0/error_codes/E0509.html).

## Practice and assessment

{{#include ../../exercises/C021.md}}

## Summary

Moving a value transfers responsibility and leaves its
source unavailable. For Copy types, a by-value use
can leave the original usable. Cloning explicitly invokes
the type's duplication behavior. When reasoning about
an aggregate, check which owned fields remain initialized
along the selected path.

[Address spaces, pages, and resource limits](../part-04/020-address-spaces-and-pages.md) · [Solutions](../ownership-solutions.md#chapter-021) · [Shared and mutable borrowing](022-borrowing.md)
