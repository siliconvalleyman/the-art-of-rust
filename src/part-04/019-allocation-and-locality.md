# 019. Hierarchy, allocation, fragmentation, and locality

A test process has enough free memory in total for a
six-slot buffer, but its allocator fails to find any free
run of six adjacent slots. A second program reads all the
same data in a different order and reuses fewer cached
lines. Both effects depend on placement and history.
This chapter derives them in models with explicit limits.

**Prerequisites.** [Operating systems and virtual memory](../part-01/005-operating-systems.md),
[asymptotic cost models](../part-02/010-asymptotic-work.md),
[storage and destruction](017-storage-and-destruction.md),
and [alignment and layout](018-alignment-and-layout.md).
After this chapter, you should be able to distinguish
internal from external fragmentation, trace a first-fit
allocator, and derive cache traffic under a specified
access pattern.

## Fit is a local property of free space

An **allocation request** specifies an amount of storage and
can also specify alignment. A **free region** is a contiguous
range available for use under a particular allocator's rules.
Free space can be split among several regions.

```text
offset  0       4       8      12      16      20
        [ free  )[  A    )[ free  )[  B    )[ free  )
```

*Figure 1. In this illustrative twenty-slot arena, three
four-slot free runs give twelve free slots in total. None
can hold a contiguous six-slot allocation.*

**External fragmentation** describes available capacity
that cannot be combined to satisfy a request because free
regions are separated by allocations. Freeing B in the
example joins three adjacent pieces into the region
`[8, 20)`. The process is called **coalescing**. It
requires tracking which regions are occupied and which
adjacent boundaries actually meet. Two disjoint free
ranges cannot be merged without also moving whatever
occupies the space between them.

**Internal fragmentation** refers to reservation inside
an allocated region beyond the storage needed by the
request. An allocator that uses a fixed eight-slot size
class for a five-slot request reserves three extra slots
in that block. Choosing larger classes can simplify
lookup and reuse, at the cost of larger internal
overhead for some request sizes. Metadata can also
consume storage, and should be counted separately when
comparing implementations.

To see why no one number describes the state, compare
one twelve-slot free region with three separate four-slot
regions. Both states report twelve free slots. The
largest contiguous free region differs, and alignment
can further restrict which starting positions work.
The allocator needs a description that supports the
questions the application asks.

## Derive a first-fit search

A fixed arena contains a finite sequence of slots.
For each slot, the input says whether it is free.
Find the smallest start index of `wanted` consecutive
free slots. Use alignment one slot and reject zero
length. An allocation routine may later mark the
selected interval occupied. This function only finds it.

Maintain the length of the current consecutive free
suffix of the processed prefix. Reading an occupied
slot sets the suffix length to zero. Reading a free
slot extends it by one. When the length first equals
`wanted`, the end index is the earliest end of
a sufficient run. Since all qualifying windows have
the same length, earliest end also gives earliest
start. Return the position **after** that end minus
the run length.

```rust,edition2021
fn first_fit(free: &[bool], wanted: usize) -> Option<usize> {
    if wanted == 0 || wanted > free.len() {
        return None;
    }
    let mut run = 0;
    for (index, &slot_is_free) in free.iter().enumerate() {
        if slot_is_free {
            run += 1;
            if run == wanted {
                return Some(index + 1 - run);
            }
        } else {
            run = 0;
        }
    }
    None
}

let free = [true, true, false, true, true, true, false];
assert_eq!(first_fit(&free, 3), Some(3));
assert_eq!(first_fit(&free, 4), None);
assert_eq!(first_fit(&free, 0), None);
assert_eq!(first_fit(&[], 1), None);
```

At the start, the processed prefix is empty and the
free suffix length is zero. Updating the suffix
preserves its definition. Along a continuing path,
no earlier window fit. On success the current free
suffix proves that the returned index is valid and
no earlier start works. At exhaustion, the invariant
rules out every end index. One iteration advances
past one input slot. A positive `run` cannot exceed
`index + 1`, so the subtraction is safe. A slice
index is always less than the length, which also
makes `index + 1` representable.

For `n` slots, this search needs at most `n`
inspections and constant extra storage. If the arena
already starts with a valid run, the search needs
`wanted` inspections. A representation of free
ranges can skip contiguous occupied or free groups.
A production allocator also needs allocation ownership,
alignment, deallocation, and synchronization contracts.
Choosing a run and marking it used must occur with
coordination when several callers can allocate at once.

## Calculate locality from a stated hierarchy

Processors often transfer data in cache lines. In a
model with four values per line, a contiguous scan over
sixteen elements visits four distinct lines. With an
initially empty cache that retains all four lines, it
misses four times. Any different order over the same
elements still misses four times under that capacity
assumption. To evaluate a benefit from changing access
order, choose a model in which reuse before eviction
actually differs.

Suppose the sixteen values form a four by four
array in row-major order. Each entire row is one
aligned line. The cache is modeled as fully associative
with two slots, meaning either slot can hold any line.
Assume on a miss it evicts the line whose last access
occurred furthest in the past when no empty slot remains.
This is the **least recently used** policy.

| Traversal | Requested row lines | Misses |
| --- | --- | ---: |
| Row major | `0,0,0,0,1,1,1,1,2,2,2,2,3,3,3,3` | 4 |
| Column major | `0,1,2,3,0,1,2,3,0,1,2,3,0,1,2,3` | 16 |

*Figure 2. The order matters when the working data
exceeds cache capacity. These counts describe the stated
two-line model and have no timing units.*

For row major, the first access to each line misses,
and the next three hit before any other line can replace
it. With column major, any two consecutive accesses use
distinct lines. After the first two accesses, the cache
keeps the last two requested lines. This relation is
preserved because the next request in the four-line cycle
is outside those two, and evicting the older one leaves
the most recent pair. Therefore every such request misses.

Real caches may have a different capacity, mapping rule,
replacement policy, or prefetch behavior. Record the
actual machine and workload before making a timing claim.
When the operation makes the same number of arithmetic
steps, fewer line acquisitions can still reduce memory
traffic. Whether it improves elapsed time also depends on
bandwidth, latency, and overlap with independent work.

## Evaluate a storage decision

For hot operations that read every field of each item,
placing fields together can help consume data already
fetched. For an operation that reads only one field over
many items, a separate contiguous array for that field
can reduce the number of unwanted bytes carried along.
The first representation gives an array of structs.
The second gives a struct of arrays. Changing between
them can affect ownership, iteration, and updates since
one logical item's fields may reside in multiple arrays.
Derive the access pattern and its invariants before
comparing costs.

For allocators, make both capacity and fragmentation part
of resource reasoning. Reusing a pool of equal-size
slots can eliminate external fragmentation for requests
of one slot. It can also reserve unused space for smaller
requests or fail for a larger request. Packing variable-size
objects can save storage in one workload and leave difficult
gaps in another. Workloads with mixed lifetimes need a
representation chosen for the observed request sizes and
lifetime distribution.

## Practice and assessment

{{#include ../../exercises/C019.md}}

## Summary

Contiguous fit depends on free regions and alignment as
well as total capacity. First fit has a simple suffix
invariant. Cache behavior depends on line layout,
capacity, replacement, and order. State those assumptions
to turn a locality claim into a checkable argument.

[Alignment, padding, endianness, and layout](018-alignment-and-layout.md) · [Solutions](../memory-solutions.md#chapter-019) · [Address spaces, pages, and resource limits](020-address-spaces-and-pages.md)
