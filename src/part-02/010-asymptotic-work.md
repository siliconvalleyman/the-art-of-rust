# 010. Asymptotic reasoning and cost models

An implementation reads a list once. Another compares every
pair of items. If both complete quickly for 10 items, what
justifies choosing one when the list may contain 100,000?
First count an operation tied to the problem, then explain
how that count grows. The machine still matters when turning
the result into elapsed time.

**Prerequisites.** [Induction and recurrence relations](008-induction.md)
and [counting, probability, and modular arithmetic](009-counting-and-probability.md).

After this chapter, you should be able to state a cost
model, prove tight or one-sided growth bounds, distinguish
best, expected, worst, and amortized work, and account for
output space separately from auxiliary storage.

## Choose what “cost” measures

A **cost model** identifies the quantities charged to an
execution. For a loop that searches bytes, counting byte
comparisons is a way to isolate the search work. If values
are strings of varying length, comparing two of them can
inspect several characters, so a comparison count alone
may omit an important variable. For a file-processing
algorithm, bytes read and the number of reads may each
matter. Specify what is fixed, what varies, and how work
is counted before stating a bound.

Let `n` be the length of an input list. Compare a scan
with a routine that visits unordered pairs of indices.
The scan makes exactly `n` visits if it reads each item
once. The pair routine makes one visit for each choice
`i < j`. For a fixed `i`, there are `n−i−1`
possible values of `j`. Summing the counts for
`i = 0` through `i = n−1` gives `n(n−1)/2`.
Counting both directions would give twice that number.

| n | Scan visits | Unordered pair visits |
| ---: | ---: | ---: |
| 0 | 0 | 0 |
| 2 | 2 | 1 |
| 10 | 10 | 45 |
| 100 | 100 | 4,950 |
| 100,000 | 100,000 | 4,999,950,000 |

*Figure 1. Counts by input length under two specified operation
rules. The table is an arithmetic comparison of work, with
no machine measurements.*

A unit-cost model for bounded machine words assigns a fixed
cost to operations such as an in-range index or adding
two words. This is useful for expressing how often the
algorithm performs such operations. CPU caches, branch
prediction, allocation, and input/output can change measured
time under a given workload. For an operation on integers
of increasing width, you may need to count word or bit
operations instead of assigning one unit to each arithmetic
symbol.

## Bounds speak about sufficiently large inputs

For nonnegative cost functions `f` and `g`, the notation
`f ∈ O(g)` means positive constants `c` and `n₀`
exist such that `f(n) ≤ c × g(n)` for every
`n ≥ n₀`. It is an upper bound on growth.
Similarly `f ∈ Ω(g)` means some positive
`c` and `n₀` satisfy `f(n) ≥ c × g(n)` from
that point on. The tight bound `f ∈ Θ(g)` follows
when both conditions hold, using possibly different
multipliers on the two sides.

For unordered pairs and `n ≥ 2`, we have
`n/2 ≤ n−1 ≤ n`. Multiplying by the nonnegative
`n/2` gives
`n²/4 ≤ n(n−1)/2 ≤ n²/2`. Those constants prove
the count is `Θ(n²)`. It is also `O(n³)`, since a
constant times `n³` is an upper bound, but that coarser
claim gives less information about growth. When comparing
two algorithms with the same asymptotic class, constants,
input families, memory behavior, and implementation can
still matter.

The size variable matters. If `n` is the integer value
of a positive input, the number of bits needed to represent
it is `floor(log₂ n) + 1`. A loop doing one operation
per integer from 1 through `n` is linear in the value
and can require exponentially many operations in the bit
length. If `n` denotes list length, linear in `n)
describes something different. Label the variable when
justifying a bound.

## Derive a search average under a distribution

Take a list of length `n` containing at most one occurrence
of the target. Visit entries from the start and stop at
the first match or the end. At the start of a comparison,
the already visited prefix contains no occurrence. The
invariant starts with an empty prefix. An unequal entry
extends the prefix while preserving the invariant. An
equal entry returns the first match. The number of
unvisited entries strictly decreases on a failed
comparison. Exhausting the input proves absence.

On a nonempty list, the best case is one comparison when
the first item is the target. The worst is `n` on a
miss or a match at the end. Empty input takes no
comparisons. For an average, suppose that for a fixed
`n ≥ 1` a request misses with probability `q`,
and conditional on success the unique position is
uniform among `0..n` with the end excluded. If the
target appears at index `i`, the cost is `i+1`.
The conditional expectation on a successful request is
`(1 + 2 + … + n)/n = (n+1)/2`. Weight that
by success probability `1−q` and add the misses:
`E = (1−q)(n+1)/2 + qn`.

With `n = 7` and `q = 1/4`, the expected cost
is `3/4 × 4 + 1/4 × 7 = 19/4` comparisons.
For `q = 0` and the stated uniform successful
position, the average grows linearly. If the workload
always hits the first item, the average is one and the
position distribution differs. Saying search alone is
“usually half the list” without a workload assumption
misses the role of both position and absence.

**Executable Rust preview.** The routine returns the
first matching position, if present, and its comparison
count. When duplicates are permitted, it still gives
the first position. The distribution above applies only
to the explicitly stated unique-match workload.

```rust,edition2021
{{#include ../../mathematics/src/cost.rs:counted_first_match}}
assert_eq!(counted_first_match(&[4, 6, 6], 6), (Some(1), 2));
assert_eq!(counted_first_match(&[4, 6, 6], 5), (None, 3));
assert_eq!(counted_first_match(&[], 5), (None, 0));
```

The function uses a traversal position and the result,
with constant space beyond the input. A routine that
collects all matching indices has output storage
proportional to the number of matches if it materializes
them. A caller might instead consume matching indices
as they arrive. Report which storage is counted when
comparing the interfaces.

## Amortize a sequence of insertions

Imagine an expandable array with a stipulated strategy.
It starts empty with zero capacity. The first insertion
allocates capacity 1. Any later insertion when the array
is full doubles its capacity, copies its elements to
the new storage, and writes the new item. Each copy
or write costs one unit. This is a teaching policy.
The actual growth strategy of Rust's `Vec` is not part
of the model.

A single growth insertion into a full array of capacity
`c` costs `c+1`. A non-growing insertion costs
one. After `n > 0` insertions there have been
`n` writes and copies at capacities that are powers of
two smaller than `n`. If `C` is the smallest
power of two at least `n`, the copies add to
`1 + 2 + … + C/2 = C−1` for `C > 1`, and
zero for `C = 1`. The sum follows by doubling the
series and subtracting the original. Since `C < 2n`,
the number of copies is less than `2n`. Counting
writes gives total cost below `3n`. Per insertion
this is constant **amortized** cost, a bound on a
sequence without a probability assumption.

| Insertions completed | Final capacity | Total copies |
| ---: | ---: | ---: |
| 0 | 0 | 0 |
| 1 | 1 | 0 |
| 2 | 2 | 1 |
| 4 | 4 | 3 |
| 5 | 8 | 7 |
| 9 | 16 | 15 |

A second derivation uses a **potential function** to
record accumulated credit. Let `Φ = 2l − c`, where
`l` is the number of stored items and `c` the
capacity. It is zero initially and nonnegative after
every insertion. In a non-growing step, the real cost
is one and potential increases by two. In a growing
step from full capacity `c > 0`, the old potential
is `c`. After growing to capacity `2c` and storing
`c+1` items, the new potential is two. Real cost
`c+1` plus potential change `2−c` gives three.
The first insertion costs one and raises potential to
one, for total adjusted cost two.

Adding the adjusted costs across the sequence leaves
only the final potential minus the initial potential.
Since that difference is nonnegative, the real total
is at most the adjusted total, which is bounded by
three times the number of insertions. This reasoning
does not bound the latency of each individual operation.
A workload that needs each insertion to complete
within a small deadline may need a different allocation
or copying policy. A growth can also temporarily require
both old and new arrays. Under a separate metric,
peak occupied capacity at that step is `3c` slots
before the old storage is released. The relevant cost
depends on whether the requirement limits total
work, one-operation latency, or memory.

## Practice and assessment

{{#include ../../exercises/C010.md}}

## Summary and further reading

Count a stated resource in terms of a declared input
size. Use upper and lower bounds to justify a tight
growth class. An expected cost needs a distribution.
An amortized bound follows from the operation sequence
and can allow costly individual steps.

[Counting, probability, and modular arithmetic](009-counting-and-probability.md) · [Solutions](../mathematics-solutions.md#chapter-010)
