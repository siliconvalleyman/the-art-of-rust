# 048. Hash tables, sets, Bloom filters, and LRU caches

A package index needs the record for a package name. A prefilter
can sometimes rule out a lookup before reading the index. A cache
keeps a limited number of recent records. These are three different
contracts. One returns the exact stored value, one can return a
false positive, and one deliberately forgets older entries.

**Prerequisites.** [Counting and probability](../part-02/009-counting-and-probability.md),
[coherence](../part-07/033-coherence-and-orphan-rules.md), and
[linked lists and deques](047-lists-rings-and-deques.md).
After this chapter, you should be able to separate key equality from
bucket placement, derive a probability under explicit assumptions,
and verify what a cache evicts.

## Use equality to confirm the candidate

A hash function maps key data to a fixed-size result. A hash table
uses that result to guide storage and search. Distinct keys can have
the same hash, a **collision**. The table still compares keys for
equality. Rust requires equal keys to hash equally. Unequal keys may
hash equally. Derive `Eq` and `Hash` together when the derived
fieldwise relation matches the domain. If identity ignores a field,
make both implementations respect that rule.

`HashMap<K, V>` stores values by key. `HashSet<K>` stores membership
by key and exposes set operations. Reinserting an equivalent key
updates a map's associated value or reports an existing set member.
Iteration over these collections is unordered by contract. Use an
ordered data structure or sort explicit output when order matters.

```rust,edition2021
use std::collections::{HashMap, HashSet};

let mut versions = HashMap::from([("api", 1), ("ui", 2)]);
assert_eq!(versions.insert("api", 3), Some(1));
assert_eq!(versions.get("api"), Some(&3));

let mut pending = HashSet::from(["api", "ui"]);
assert!(!pending.insert("api"));
assert!(pending.remove("api"));
assert!(!pending.contains("api"));
```

For a simple separate-chaining model with `n` entries and `m > 0`
buckets, the load is `n/m`. Suppose each stored key is assigned to a
uniform bucket and an unsuccessful query inspects its entire bucket.
Condition on the query bucket when stored assignments are independent
of it. Every stored key has probability `1/m` of landing there.
Summing indicators gives expected inspected entries `n/m`. Include
the work of hashing the query and checking candidate equality for
a total cost. If all entries land in the queried bucket, the chain
has `n` entries. This is a model of load and collisions. Rust's
`HashMap` documents open addressing using quadratic probing and
SIMD lookup, so its probe sequence differs from this example's chain.
The integration fixture gives unequal keys the same hash and checks
that both remain addressable.

## Derive what a Bloom filter can answer

Start with `m` zero bits. An insertion computes `k` bit indices and
sets them all. A lookup checks whether all its bits are set. If one
bit is clear, the item was never inserted under the same mapping.
If all are set, the item may have been inserted. Different items can
have collectively set its bits. Clearing a bit on removal could
also clear evidence for another item, so this insertion-only model
has no delete operation.

```rust,edition2021
{{#include ../../data-structures/src/membership.rs}}

let mut filter = Bloom64::default();
filter.insert([1, 2]);
filter.insert([3, 4]);
assert!(filter.might_contain([1, 4]));
assert!(!filter.might_contain([1, 5]));
```

The small implementation has 64 bits and takes two fingerprints
from a caller. Reducing them modulo 64 gives valid shift counts.
The test compares all 4,096 possible independent ordered pairs of
positions after these two insertions. A lookup is positive for
exactly the 16 pairs drawn from the four occupied positions. For
an application, verify a positive result against the exact store.

In the idealized model, assume the `k` indices of an absent query
are independent uniform draws with replacement, independent of the
filter state. Given `X` occupied bits, the conditional false-positive
probability is `(X/m)^k`. After `n` insertions at most `min(m, kn)`
bits can be set, so substituting it for `X` gives an upper bound.
Under the additional assumption that insertion positions are also
independent uniform draws, each bit remains clear with probability
`(1 - 1/m)^(kn)`. The expected occupancy is
`m * (1 - (1 - 1/m)^(kn))`. To get the exact unconditional
false-positive probability, average `(X/m)^k` over the occupancy
distribution. Raising the mean occupancy fraction to the kth power
generally differs from that average. The assumptions concern the
query and hash distribution and are separate from the monotone-bit
proof of no false negatives for inserted items.

## Specify recency and evict only the oldest entry

The least recently used cache in this chapter counts successful
lookups and replacements as uses. The front of the deque holds
the oldest entry and the back holds the newest. A miss leaves the
order alone. A new item at capacity removes the oldest pair. A
zero-capacity cache returns the input without storing it.

```rust,edition2021
{{#include ../../data-structures/src/lru.rs}}

let mut cache = SmallLru::new(2);
assert_eq!(cache.insert("A", 1), Insertion::Stored);
assert_eq!(cache.insert("B", 2), Insertion::Stored);
assert_eq!(cache.get(&"A"), Some(&1));
assert_eq!(cache.insert("C", 3), Insertion::Evicted("B", 2));
```

The invariant is at most `capacity` distinct keys ordered from
oldest to newest. The cache starts empty. A hit moves one pair
to the back, preserving the relative order of survivors. A
replacement discards the previous pair and adds its replacement
at the back, preserving uniqueness and size. A new key appends
directly if there is room. Otherwise removing the front first
makes room for exactly one append. Each case preserves the invariant.

This cache searches a deque, spending at most `O(capacity)` entry
comparisons and moves per operation, with `O(capacity)` storage.
For a larger cache, a hash index mapping keys to stable node handles
and a doubly linked recency structure can reduce expected lookup and
relinking work. Both structures must always agree on which nodes are
live. Measure hit rate, contention, value size, and lookup cost before
choosing a representation for an actual service.

## Practice and assessment

{{#include ../../exercises/C048.md}}

## Summary and further reading

Keep exact membership, approximate membership, and eviction policies
distinct in the API. State hash workload assumptions and test both
collisions and replacement order. See [Rust 1.75.0 Hash and Eq](https://doc.rust-lang.org/1.75.0/std/hash/trait.Hash.html#hash-and-eq),
[HashMap](https://doc.rust-lang.org/1.75.0/std/collections/struct.HashMap.html),
and [Bloom's original paper](https://crystal.uta.edu/~mcguigan/cse6350/papers/Bloom.pdf).

[Linked lists and deques](047-lists-rings-and-deques.md) · [Solutions](../data-structures-solutions.md#chapter-048)
