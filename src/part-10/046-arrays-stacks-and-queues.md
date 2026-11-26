# 046. Arrays, dynamic arrays, stacks, and queues

A device keeps three incoming readings while its consumer catches up.
A separate editor keeps undo records in the order that edits arrive,
then removes the latest record first. Both store sequences, but they
make different promises about capacity and which value leaves next.

**Prerequisites.** [Cost models](../part-02/010-asymptotic-work.md),
[alignment and layout](../part-04/018-alignment-and-layout.md),
[drop behavior](../part-05/025-raii-and-drop.md), and
[bounded updates](../part-09/045-bounded-recovery.md).
After this chapter, you should be able to choose between fixed and
dynamic capacity, derive a growth bound, and prove that a circular
index preserves queue order.

## Separate representation from removal order

Rust's `[T; N]` has a fixed length `N` in its type. Its elements occupy
consecutive positions inside the array value. A `Vec<T>` has a runtime
length and capacity. For an element type that occupies storage, an
allocated vector's initialized elements occupy the first `len`
positions of its contiguous allocation. Spare positions are capacity
for future elements. `Vec` does not allocate element storage for a
zero-sized `T`. An array's storage location depends on where its owning
value lives, including inside another allocation.

| Operation pattern | Suitable representation | Cost to inspect a known valid index |
| --- | --- | --- |
| A fixed number of homogeneous samples | `[T; N]` | Constant time |
| Grow a sequence at the tail | `Vec<T>` | Constant time |
| Remove the newest outstanding edit first | A stack using the end of a `Vec<T>` | Stack exposes the top |
| Remove the oldest pending request first | A ring buffer with a front cursor | Queue exposes the front through removal |

A stack follows last-in, first-out order. A queue follows first-in,
first-out order. A queue implemented by repeatedly removing index
zero from a vector moves the remaining entries to fill the gap.
Instead, keep an index for the logical front and reuse vacant slots
after the tail wraps. The standard growable representation for such
use is `VecDeque`. Here we implement a fixed-capacity variant so the
full-state policy and exact work are visible.

## Derive a growth bound from a stated policy

Consider a dynamic array that starts with zero capacity, allocates
one slot on the first push, and doubles its capacity whenever full.
Count one write for each inserted element and one relocation per
existing element when allocating additional space. Assume a sequence
of pushes only and set allocator internals aside. When pushing five
items, the writes include five new elements and relocations of
`1 + 2 + 4 = 7` existing elements.

For `m >= 1` pushes let `C` be the final capacity. It is a power of
two with `m <= C < 2m`. The relocations sum to
`1 + 2 + ... + C/2 = C - 1`, with an empty sum for `C = 1`. That
sum follows by adding the previous capacity and subtracting from
twice the sum, or by induction on doublings. The total counted work
is `m + C - 1 < 3m`. Thus each push has constant amortized work in
this model, even though a growth operation can move linearly many
items. The aggregate bound needs no probability distribution. Rust's
`Vec` documents amortized constant-time push and leaves the exact
growth factor unspecified. Measure allocation counts and time if a
specific allocator or workload matters.

## Implement the order and handle full storage

The stack delegates to `Vec` at its tail. `peek` shares the newest
item, and `pop` transfers its ownership. The bounded queue stores
`Option<T>` in each slot. A vacant slot uses `None`. `push` returns
`Err(item)` when full, giving ownership back to the caller. A
successful `pop` uses `take` to clear the front slot and return its
value. No heap allocation is performed by the queue itself. `T` may
of course own allocations of its own.

```rust,edition2021
{{#include ../../data-structures/src/linear.rs}}

let mut stack = Stack::new();
stack.push("old edit");
stack.push("new edit");
assert_eq!(stack.pop(), Some("new edit"));
assert_eq!(stack.peek(), Some(&"old edit"));

let mut queue = Queue::<_, 2>::new();
assert_eq!(queue.push("A"), Ok(()));
assert_eq!(queue.push("B"), Ok(()));
assert_eq!(queue.push("C"), Err("C"));
assert_eq!(queue.pop(), Some("A"));
assert_eq!(queue.push("C"), Ok(()));
assert_eq!(queue.pop(), Some("B"));
assert_eq!(queue.pop(), Some("C"));
assert_eq!(queue.pop(), None);
```

The two-slot queue wraps after dequeueing A. The next insertion
reuses physical slot zero, but B remains the logical front.

| Step | Physical slot 0 | Physical slot 1 | Front index | Logical sequence |
| --- | --- | --- | ---: | --- |
| Enqueue A and B | A | B | 0 | A, B |
| Dequeue A | vacant | B | 1 | B |
| Enqueue C | C | B | 1 | B, C |
| Dequeue B | C | vacant | 0 | C |

## Prove the ring invariant and finite-width indices

For `N > 0`, `first < N` and `len <= N`. In order, each offset
`j` from zero through `len - 1` occupies `(first + j) mod N` in
mathematical integers. All other slots are vacant. Initially `len`
is zero and every slot is vacant. A full push returns before any
write. Otherwise offset `len` designates the next vacant slot.
Distinct offsets smaller than `N` have distinct residues modulo
`N`, so storing there adds precisely one item to the logical tail.

The code avoids forming a potentially overflowing `first + len`.
If `len < N - first`, their sum is below `N`. Otherwise the wrapped
position is `len - (N - first)`, which is nonnegative and below
`N`. A nonempty pop removes the head, advances `first` by one with
wraparound, and decreases `len`. That moves every surviving logical
offset down by one while leaving its physical position unchanged.
The zero-capacity case returns before either operation can index
the array. These arguments establish first-in, first-out behavior
for any sequence of public operations.

Each queue operation touches a constant number of indices and
slots. Initialization writes `N` vacancies and storage is linear in
`N`. The queue's destructor drops the remaining `Some` values.
Independent tests compare all seven-operation sequences over two
push values and one pop against a `VecDeque` reference, for capacities
zero through three. Another test counts drops for removed, retained,
and rejected items separately.

## Practice and assessment

{{#include ../../exercises/C046.md}}

## Summary and further reading

Choose removal order and full-state policy before choosing storage.
Amortized cost bounds a sequence. A fixed-capacity ring gives a
per-operation bound by spending storage up front. Consult the
[Rust 1.75.0 Vec push contract](https://doc.rust-lang.org/1.75.0/std/vec/struct.Vec.html#method.push)
and [VecDeque](https://doc.rust-lang.org/1.75.0/std/collections/struct.VecDeque.html).

[Robust APIs](../part-09/045-bounded-recovery.md) · [Linked lists, rings, and deques](047-lists-rings-and-deques.md) · [Solutions](../data-structures-solutions.md#chapter-046)
