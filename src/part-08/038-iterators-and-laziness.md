# 038. Iterator contracts and laziness

An input source can yield values one at a time. A
caller may consume all of them or stop after finding
the first useful value. A Rust iterator lets the source
keep the state required to produce its next item. The
caller decides how much of the sequence is demanded.

**Prerequisites.** [Traits and associated items](../part-07/032-traits-and-associated-items.md)
and [closures and capture](037-closures-and-capture.md).
After this chapter, you should be able to implement an
iterator, prove its stopping rule, and distinguish a
lazy adapter from a consumer.

## Describe the state and the next operation

The `Iterator` trait has an associated `Item` type
and a required `next(&mut self) -> Option<Self::Item>`
method. The mutable receiver lets the iterator update
its own traversal state. A successful call yields an
item. A call returning None supplies no item. The
general trait allows a later call to resume after None.
An iterator with the stronger `FusedIterator` marker
promises that once it returns None, it continues to
return None.

```rust,edition2021
{{#include ../../examples/functional/src/countdown.rs}}

let mut countdown = Countdown::new(3);
assert_eq!(countdown.size_hint(), (3, Some(3)));
assert_eq!(countdown.next(), Some(3));
assert_eq!(countdown.size_hint(), (2, Some(2)));
assert_eq!(countdown.next(), Some(2));
assert_eq!(countdown.next(), Some(1));
assert_eq!(countdown.next(), None);
assert_eq!(countdown.next(), None);
```

The state field contains the next value or zero if
there is nothing left to yield. Zero is internal
state, and the public interface signals absence
with None. For a positive start `s`, after `k`
successful calls the remaining field is `s − k`.
Initially `k = 0`. Each positive call returns its
old field value and subtracts one, so it preserves
that relation and advances toward zero. At zero
there is no next item and the state does not change.
The subtraction is therefore never performed at
zero, and the successful sequence is precisely
`s, s−1, …, 1`.

`size_hint` returns a lower bound and an optional
upper bound on the number of remaining items.
Here the count is exactly the field value, so the
two bounds agree. In general a size hint is useful
for planning allocation, and callers must still
obtain items through the iterator. For example,
an iterator that filters an unsorted sequence
cannot know how many items will pass from its
original length alone.

Each call uses a fixed number of operations and
no heap storage. Consuming all items calls the
transition once per item and checks exhaustion.
An upstream source with I/O or more work could
have a different per-item cost under the same trait.

## Adapters defer work until a consumer requests it

`map` creates an **adapter** that stores an upstream
iterator and a callable. When asked for a value, it
requests an upstream item and transforms it.
`take` restricts how many items may be requested
from its source. `collect` is a **consumer** that
requests values to build an output collection.

```rust,edition2021
{{#include ../../examples/functional/src/countdown.rs}}
use std::cell::Cell;

let calls = Cell::new(0);
let mapped = Countdown::new(4).map(|value| {
    calls.set(calls.get() + 1);
    value + 1
});
assert_eq!(calls.get(), 0);
let prefix: Vec<_> = mapped.take(2).collect();
assert_eq!(prefix, [5, 4]);
assert_eq!(calls.get(), 2);
```

The mapping closure is created immediately and can
capture values during that construction. Its body
does not run just because the adapter is constructed.
With this chain the consumer stops after two values,
and the mapping body has run only twice. The output
Vec is a materialized collection and needs storage.
The adapter itself stores traversal state and the
closure.

A loop can express the same demand explicitly.
An iterator makes composition easier when a caller
needs to supply adapters or change the stopping
condition. Short-circuiting can also change which
effects occur, so use a loop or explicit consumer
when each item must be processed.

## Practice and assessment

{{#include ../../exercises/C038.md}}

## Summary and further reading

An iterator represents a sequence through state and
its next operation. State whether exhaustion is permanent.
Use the invariant to prove progress and coverage. Lazy
adapters perform work as their items are requested.
[Rust 1.75.0 Iterator](https://doc.rust-lang.org/1.75.0/std/iter/trait.Iterator.html) · [FusedIterator](https://doc.rust-lang.org/1.75.0/std/iter/trait.FusedIterator.html).

[Closures and capture](037-closures-and-capture.md) · [Solutions](../functional-solutions.md#chapter-038) · [Adapters and reductions](039-adapters-and-reductions.md)
