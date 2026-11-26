# 025. RAII, Drop, and resource invariants

A component grants at most two simultaneous reservations.
When a caller is done, it should return its reservation
even if a normal control path ends early. A guard can
carry the obligation to return it. To design the guard,
specify when it becomes an owner, what its destructor can
restore, and what the program must do if cleanup is skipped.

**Prerequisites.** [Storage and destruction](../part-04/017-storage-and-destruction.md),
[ownership and moves](021-ownership-and-moves.md), and
[variance and aliasing](024-variance-and-aliasing.md).
After this chapter, you should be able to design a
scoped resource guard, prove its accounting invariant,
and separate ordinary cleanup from fallible finalization.

## Put release responsibility in an owner

**Resource acquisition is initialization**, commonly
abbreviated **RAII**, associates acquisition with the
creation of an owning value. Releasing the resource is
then associated with destroying that value. A resource
could be an allocation, a file handle, a lock, or a permit
representing one unit of capacity. The owner can move to
other bindings or return to a caller. Its ownership
obligation moves with it.

A **guard** is a value established after a successful
acquisition. Until it is destroyed or deliberately
released, the resource is reserved. A failed acquisition
must leave no guard claiming success and no unaccounted
change to the pool. A guard can also restrict operations
by keeping a reference to the thing it protects. Finishing
an ordinary local scope causes Rust to drop its owners
under the applicable scope rules. This helps keep release
paths consistent across success and early return.

## Derive a single-threaded reservation guard

The pool below contains a count of free permits.
It grants one permit only when the count is positive.
The guard borrows the pool, so the pool must remain
available while the guard's destructor could need it.
The counter uses `Cell` so separate guards can all
refer to it with shared references. This program models
one thread. The pool's methods do not describe a
concurrent allocator.

```rust,edition2021
{{#include ../../examples/ownership/permits.rs}}

let pool = permits::Pool::new(2);
let first = pool.acquire().unwrap();
{
    let _second = pool.acquire().unwrap();
    assert!(pool.acquire().is_none());
    assert_eq!(pool.available(), 0);
}
assert_eq!(pool.available(), 1);
drop(first);
assert_eq!(pool.available(), 2);
```

The `mod` block groups the implementation. Methods
on `Pool` are the public entry points. Callers
outside the module cannot construct a `Lease` by
writing its private field or arbitrarily edit the
counter. That boundary is necessary: otherwise a
caller could create a second guard without paying for
it, and the release proof would fail.

Let `capacity` be the constructor's input,
`free` the counter, and `outstanding` the number
of successfully acquired permits that have not been
returned. The invariant is
`free + outstanding = capacity`, with both counts
nonnegative. Creation starts with every permit free.
A successful acquisition decrements free and
increments outstanding. A failed one changes
neither. Dropping an acquired guard reverses one
acquisition, and a move of the guard changes no
counts. During a release there was at least one
outstanding permit, so free could be at most
`capacity−1`. The addition in the destructor
therefore fits even at maximal capacity.

| Event | Free | Outstanding |
| --- | ---: | ---: |
| Create capacity two | 2 | 0 |
| Acquire first | 1 | 1 |
| Acquire second | 0 | 2 |
| Try acquiring at zero | 0 | 2 |
| Exit inner block | 1 | 1 |
| Drop first | 2 | 0 |

*Figure 1. An unsuccessful acquisition preserves the
count. Each ordinary release returns exactly one unit.*

The guarded operations use a fixed number of counter
operations and no heap allocation in this example.
Real resource acquisition can fail for other reasons,
and it may require synchronization. If the guard's
constructor does work after acquiring multiple
resources, earlier successful acquisitions need their
own owners so failure can release them.

## Distinguish Drop from observable finalization

`Drop::drop` runs as part of destruction and
returns unit. It cannot return an error to the
original caller. For a buffered writer, an explicit
flush can report an output error. If persistence
is required, the application may also require
another operation with a documented durability
contract. Decide which result must be observed
before reporting success. Cleanup after that
can still release the remaining ownership.

The standalone `std::mem::drop(value)` function
takes its argument by value so that the passed
owner is destroyed before continuing. Its
effect depends on moving the owner, which
the direct method call `value.drop()` does
not do. Rust disallows calling the destructor
method explicitly. This avoids leaving an
initialized value that could later be used or
destroyed again.
[Rust 1.75.0 documentation for Drop](https://doc.rust-lang.org/1.75.0/std/ops/trait.Drop.html).

Cleanup also depends on control flow. Ordinary
scope exit or a normal return triggers applicable
destruction. Abrupt termination can skip it. Rust
also offers the safe operation `std::mem::forget`,
which consumes a value without running its
destructor. Forgetting a lease in the example
leaves a permit outstanding until the pool ends.
It can make further acquisitions fail, and
the model does not create an invalid access as a
result. The accounting invariant includes such an
outstanding permit. A real abstraction's memory
safety cannot rely on the destructor always running.
[Rust 1.75.0 documentation for forget](https://doc.rust-lang.org/1.75.0/std/mem/fn.forget.html).

An operation that returns errors should acquire,
finish, and report them at distinct points. To
verify it, test acquisition failure with no state
change, a successful acquisition and release,
and a failure after acquisition where the guard
must still release the resource. If intentional
leakage is possible, name what remains safely
accessible and what capacity the leak can consume.

## Practice and assessment

{{#include ../../exercises/C025.md}}

## Summary

Acquire and release a resource through an owner.
Keep construction behind a boundary that maintains
the resource invariant. Test ordinary and early
control paths, report fallible finalization explicitly,
and account for cleanup that can be skipped.

[Variance, subtyping, and aliasing](024-variance-and-aliasing.md) · [Solutions](../ownership-solutions.md#chapter-025) · [Structs and products](../part-06/026-structs-and-products.md)
