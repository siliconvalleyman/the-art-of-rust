# 022. Shared and mutable borrowing

A function needs to measure the length of an existing
string. Another needs to append one character. Both can
work without taking ownership of the string. To give
the functions the access each needs, Rust lets a caller
lend a view temporarily while keeping responsibility
for the value. The borrow checker verifies that those
temporary permissions are compatible.

**Prerequisites.** [Ownership and moves](021-ownership-and-moves.md)
and [pointer values](../part-04/016-addresses-and-stack-frames.md).
After this chapter, you should be able to distinguish
shared from exclusive access, identify the last use of
a borrow, reborrow a mutable reference, and derive a
safe split of a mutable slice.

## Lend access without transferring the owner

A **borrow** grants access through a reference while
another place retains ownership. The expression
`&text` can create a shared reference, and
`&mut text` can create a mutable reference. In this
example, the parameter `&str` gives a read-oriented
view of text. Passing an appropriate borrow of a
`String` can supply that view.

```rust,edition2021
fn byte_length(text: &str) -> usize {
    text.len()
}

let text = String::from("café");
assert_eq!(byte_length(&text), 5);
assert_eq!(text, "café");
```

The caller keeps the string and its storage. The
function receives a view that supplies the data location
and length, and computes the length through the view.
The argument's reference is usable for the duration of
this call. No clone of the text is required.

With ordinary data, several shared references can
read the same location while those accesses remain
compatible. Through a mutable reference, the caller lends
exclusive access to the borrowed region, permitting
reads and updates through that reference. Rust must
reject conflicting access through other paths while
the exclusive borrow is active. An abstraction based
on interior mutability can provide mutation through a
shared reference under its own rules. Chapter 58
examines that case. For the rules here, use an ordinary
integer or String whose contents are accessed through
these references.
[Rust 1.75.0 Reference on pointer types](https://doc.rust-lang.org/1.75.0/reference/types/pointer.html).

```rust,edition2021
fn increment(value: &mut u16) {
    *value += 1;
}

let mut count = 10;
increment(&mut count);
assert_eq!(count, 11);
```

The caller's binding is mutable, so Rust can
lend writable access. Inside the function, the `*`
selects the referred-to value. Updating it leaves
ownership with the caller. The caller can read the
result when the call and this borrow are finished.
For the demonstrated input the arithmetic is within
range. A reusable increment API for arbitrary inputs
would also choose an overflow policy.

## Track when a borrow is used

Consider a shared reference to an integer with value
4. If the program prints that reference and then
creates an exclusive borrow that stores 5, the two
required regions can be disjoint in the execution.
Rust 2021 can determine this from the uses of each
reference, even if the two local names appear in the
same surrounding block. This behavior is called
**non-lexical lifetimes**, where the region for a
borrow can be smaller than the enclosing lexical scope.

| Program point | Access path being used | Value |
| --- | --- | ---: |
| Create shared view | `snapshot = &value` | 4 |
| Last use of shared view | `assert_eq!(*snapshot, 4)` | 4 |
| Create exclusive view | `target = &mut value` | 4 |
| Update through exclusive view | `*target = 5` | 5 |
| Read owner after last exclusive use | `assert_eq!(value, 5)` | 5 |

*Figure 1. The values occupy the same owner throughout.
The reference uses end before the next incompatible
access starts.*

```rust,edition2021
let mut value = 4;
let snapshot = &value;
assert_eq!(*snapshot, 4);
let target = &mut value;
*target = 5;
assert_eq!(value, 5);
```

Move the assertion using `snapshot` below the
exclusive update and the compiler must account for
the shared reference still being used. A later use can
extend the borrow back across an earlier attempted
operation. Diagnose that conflict by finding both the
new borrow and the later use that keeps the earlier
one live.

**Compile-fail Rust.** Rust 1.75.0 produces E0502 for
the attempted mutable borrow while a shared borrow is
still used.

```rust,compile_fail,edition2021
{{#include ../../tests/fixtures/shared_and_exclusive.rs}}
```

If the code needs only the old number, make a copy
of the integer rather than retain a reference. If
it needs the same object's latest value, use the
exclusive reference to write and then read that
object through a compatible path. Which repair is
correct depends on whether the program is preserving
an observation or retaining a live view.

## Reborrow an existing mutable reference

A **reborrow** creates a new borrow through an
existing reference. Given a mutable reference
`original`, the expression `&mut *original`
temporarily lends exclusive access again. The shorter
borrow uses the same object. When it is no longer
required, the original reference can resume using
its permission. While the shorter borrow is active,
conflicting use of `original` would violate exclusivity.

```rust,edition2021
fn increment(value: &mut u16) {
    *value += 1;
}

let mut total = 10;
let original = &mut total;
let temporary = &mut *original;
increment(temporary);
increment(original);
assert_eq!(total, 12);
```

The first call uses the shortened access, and the
second uses the original after that use ends. When
an argument position expects `&mut T), Rust can
reborrow a compatible mutable reference for that call.
A simple binding such as `let next = original;`
with its type inferred can instead move the mutable
reference. Mutable references are not Copy. That
distinction explains why repeatedly calling a function
with a mutable reference can work while using the
original after an ordinary by-value move can fail.

## Prove separation when accesses are disjoint

A program can safely write to two different positions of an
array. But two indexed borrows can both be seen as
borrows of the same slice, and a general indexing
expression may use indices that coincide. Rust does not
accept two active mutable borrows in this example.

**Compile-fail Rust.** The simultaneous mutable borrows
of the array lead to E0499 on the reference compiler.

```rust,compile_fail,edition2021
{{#include ../../tests/fixtures/two_indexed_borrows.rs}}
```

For a validated split index `mid`, a slice of length
`n` can be divided into two ranges, `[0, mid)` and
`[mid, n)`. No index belongs to both. If an index
belongs to the first range it is strictly less than
`mid`, while every index in the second is at least
`mid`. Their union covers the original slice because
each valid index satisfies exactly one of those two
conditions. A split at zero or the original length
simply leaves one range empty.

The standard library's `split_at_mut` provides those
disjoint regions through a safe method. It requires a
split position at most the slice's length. When the
split comes from an external request, check that before
calling it and return a recoverable outcome on failure.
[Rust standard library documentation for split_at_mut](https://doc.rust-lang.org/1.75.0/std/primitive.slice.html#method.split_at_mut).

```rust,edition2021
fn halves(values: &mut [u8], mid: usize) -> Option<(&mut [u8], &mut [u8])> {
    if mid > values.len() {
        return None;
    }
    Some(values.split_at_mut(mid))
}

let mut values = [1, 2, 3, 4];
let (left, right) = halves(&mut values, 2).unwrap();
left[0] = 10;
right[0] = 30;
assert_eq!(values, [10, 2, 30, 4]);
assert!(halves(&mut values, 5).is_none());
```

The returned slice references carry views of the same
underlying allocation. The split needs neither a new
buffer nor a traversal of elements. The method's contract
guarantees the views are disjoint. The caller's original
access remains constrained until those views are no longer
used. This example demonstrates that borrow rules track
which parts are accessed, as well as whether mutation is
permitted.

## Practice and assessment

{{#include ../../exercises/C022.md}}

## Summary

Borrowing lends permission while ownership stays with
the caller. Several shared views can coexist under
compatible operations. Exclusive access rules prohibit
conflicting active paths. Reborrows shorten a loan,
and a library split can expose proven disjointness as
two safe mutable views.

[Ownership, moves, Copy, and Clone](021-ownership-and-moves.md) · [Solutions](../ownership-solutions.md#chapter-022) · [Lifetimes, elision, and lifetime bounds](023-lifetimes.md)
