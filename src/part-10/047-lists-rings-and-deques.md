# 047. Linked lists, doubly linked lists, rings, and deques

A scheduler inserts urgent items at one end and normal items at the
other. A separate program moves a whole group of records from one
sequence to another. Both need an order, but fast operations at the
ends and fast transfer of an existing chain favor different structures.

**Prerequisites.** [Aliasing](../part-05/024-variance-and-aliasing.md)
and [arrays, stacks, and queues](046-arrays-stacks-and-queues.md).
After this chapter, you should be able to derive link invariants,
explain the two spans in a circular layout, and choose a structure
based on the operations and memory access pattern.

## Build a singly linked front list

Each node owns a value and may own one successor. A `Box` gives the
recursive type a finite-sized link and keeps the next node alive.
The whole list owns the first node or records `None`. No public
operation accepts a raw node or a user-provided link. Starting from
that empty base, the chain is finite and acyclic.

Insertion takes the previous head and places it after a new node.
Removal takes the head, installs its successor, and returns the old
value. The iterator follows shared references; it can read values
only while the chain is borrowed. It has no permission to unlink
the node it currently visits.

```rust,edition2021
{{#include ../../data-structures/src/linked.rs}}

let mut list = FrontList::new();
list.push_front(String::from("base"));
list.push_front(String::from("new"));
assert!(list.iter().map(String::as_str).eq(["new", "base"]));
assert_eq!(list.pop_front().as_deref(), Some("new"));
assert_eq!(list.pop_front().as_deref(), Some("base"));
assert!(list.pop_front().is_none());
```

The logical sequence is the values encountered from head to the
first absent link. Insertion attaches the old chain to a fresh node
with no incoming references from that chain, so order changes only
by one new first element and no cycle can appear. Removal detaches
only the first node and exposes its original suffix. The resulting
Box owns no successor when it is destroyed. These two rules give an
induction over any sequence of front operations.

Each front operation visits a constant number of nodes. Insertion
allocates one Box for this node type, and removal deallocates one.
A traversal or a position lookup from the head visits up to `n`
nodes. Dropping a long chain by ordinary recursive field destruction
can use stack proportional to its length. `Drop` instead repeatedly
detaches one node, so normal destruction uses constant additional
call depth for the list itself. A panic inside an element destructor
can interrupt that loop. The example does not promise constant stack
on that failure path or constrain what an element's own destructor
does.

## Add backward links for operations at both ends

A doubly linked sequence has a previous and next link per node and
retains both endpoint handles when nonempty. An endpoint lacks an
outside neighbor. At interior positions, if `a.next` reaches `b`,
then `b.prev` reaches `a`. A removal can connect the two neighbors
in constant link work when the node is already known. Searching
from an endpoint to find the node can still take linear time.
Owning Boxes in both directions would not express a single ownership
chain. This example uses the standard library for a doubly linked
container and reasons about its external interface.

`LinkedList::append` moves a second list onto the end in constant
time and leaves that list empty. This is a library guarantee. It
fits a workload that repeatedly receives two separately allocated
chains and joins them. Verify both the order and the source's final
state.

```rust,edition2021
use std::collections::LinkedList;

let mut ready = LinkedList::from(["A", "B"]);
let mut extra = LinkedList::from(["C", "D"]);
ready.append(&mut extra);
assert!(extra.is_empty());
assert_eq!(ready.pop_back(), Some("D"));
assert!(ready.into_iter().eq(["A", "B", "C"]));
```

## Use a ring for a double-ended queue

A deque can insert and remove at either end. Consider a ring with
capacity `c`, `len` live entries, and front index `first`. If it is
nonempty, the number of logical items before physical wraparound is
`min(len, c - first)`. Any remainder occupies a second segment from
physical index zero. For example, with `c = 5`, `first = 3` and
`len = 4`, iteration visits indices `3, 4` followed by `0, 1`.
The same rule covers a one-segment layout when the second length is
zero. The empty sequence has no live segments.

This representation updates a front index and a length to advance
the window. In a growable ring, a push can also require more storage.
The standard `VecDeque` documents how its two slices are ordered.
Their actual split depends on the current layout, so use both
slices in order instead of assuming a push forces a particular split.

```rust,edition2021
use std::collections::VecDeque;

let mut events = VecDeque::from([10, 20, 30]);
assert_eq!(events.pop_front(), Some(10));
events.push_back(40);
events.push_front(15);
let (front, back) = events.as_slices();
assert!(front.iter().chain(back).copied().eq([15, 20, 30, 40]));
```

A ring has contiguous spans and constant-time indexing within
them. A traversal of separately allocated nodes depends on
following each next link before obtaining the address of its
successor. A compact layout can improve spatial locality for scans.
Actual latency depends on allocation, element size, and the target.
Use a representative workload before making a timing claim.

## Practice and assessment

{{#include ../../exercises/C047.md}}

## Summary and further reading

Name the operations that dominate the workload, then derive the
required links or indices. Check their behavior across empty,
single-element, and wrapped states. See the Rust 1.75.0 documentation
for [LinkedList::append](https://doc.rust-lang.org/1.75.0/std/collections/struct.LinkedList.html#method.append)
and [VecDeque::as_slices](https://doc.rust-lang.org/1.75.0/std/collections/struct.VecDeque.html#method.as_slices).

[Arrays, stacks, and queues](046-arrays-stacks-and-queues.md) · [Hash tables and caches](048-hash-tables-and-caches.md) · [Solutions](../data-structures-solutions.md#chapter-047)
