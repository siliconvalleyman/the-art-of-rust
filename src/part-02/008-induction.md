# 008. Induction, strong induction, and recurrence relations

A program splits a batch into nearly equal halves, solves each
half, and then reads every item to combine the results. If the
batch has an odd length, the two halves differ by an item.
Testing lengths 2, 4, and 8 will never exercise that
case. We need an argument that covers every length and
counts the work even when the split is uneven.

**Prerequisite.** [Sets, relations, functions, and graphs](007-sets-and-graphs.md),
including [propositions and proof](006-propositions.md).

After this chapter, you should be able to construct an
induction proof, choose a strong induction or structural
step, prove that a recursive computation terminates, and
derive an exact operation count from a recurrence.

## Induction connects one size to the next

Use **mathematical induction** to prove propositions
`P(0), P(1), P(2), …` indexed by nonnegative integers.
Establish `P(0)` as the base case. For an arbitrary
nonnegative integer `k`, assume `P(k)` and use it to
prove `P(k+1)`. This is the inductive step. If a false
case existed, the nonnegative integers would contain a least
false index. The base case rules out zero. Its predecessor
would satisfy the proposition, and the step would force the
alleged false case to satisfy it too. This contradiction
explains why the two obligations cover the full domain.

Suppose `S(n)` is the sum of integers 1 through `n`,
with `S(0) = 0`. Derive its formula by pairing each term
with its partner in a reversed copy of the sum.

```text
first copy      1      2      3     ...     n
reversed copy   n     n-1    n-2    ...     1
column sum     n+1    n+1    n+1    ...    n+1
```

*Figure 1. There are n columns. Each adds to n + 1, and
both rows have the same total. Half the doubled total is
the sum of the original row.*

Check the resulting formula `S(n) = n(n+1)/2` by
induction. At zero both sides are zero. For the step,
assume `S(k) = k(k+1)/2`. The next sum adds precisely
`k+1`. Substituting the assumption gives
`k(k+1)/2 + (k+1) = (k+1)(k+2)/2`, the desired
formula at index `k+1`. This proof names the exact step
where it uses the previous claim. Checking many small
examples can help discover a formula and can catch errors,
while the proof demonstrates the recurrence is satisfied at
all indices starting from the base value.

A constant-operation implementation of the formula still
needs safe intermediate arithmetic. With input of type
`u32`, widen first. The largest product for this formula
is `(2^32−1) × 2^32 = 2^64−2^32`, which fits
`u64`. Either factor is safe after widening. Computing
`n+1` in the input type would exceed its maximum for
`n = 2^32−1`. Mathematical correctness supplies a
bound for each operation only when the implementation
computes the same quantities in that order.

## What strong induction adds

**Strong induction** lets the step assume that all prior
cases, within the stated domain, satisfy the proposition.
It suffices because a smallest false case would still have
all earlier cases true. This formulation is helpful when
the definitions call for results several indices smaller,
or when the appropriate predecessor depends on the input.

Consider producing a postage amount using denominations 3
and 5. To prove every amount at least 8 can be represented,
give three base cases: `8 = 3 + 5`, `9 = 3 + 3 + 3`,
and `10 = 5 + 5`. For an arbitrary `n ≥ 11`, assume
the claim for every integer amount `k` with
`8 ≤ k < n`. Since `n − 3` lies in that range,
its representation exists. Append one more 3 to obtain
a representation for `n`. The step uses an earlier
case three places back. A proof using only the immediately
preceding claim does not furnish that earlier case
without enriching the proposition.

There is a constructive implementation hidden in the
proof. For an input `n ≥ 8`, choose the base from 8, 9,
or 10 having the same remainder as `n` on division
by 3. The difference is a nonnegative multiple of 3.
Start with the base representation and add the quotient
number of threes. If `n = 20`, use base 8 and four
extra threes, giving five threes and one five. Greedily
taking the largest available denomination can fail even
when a representation exists. It takes a 5 for 9 and
leaves 4, whereas three 3s work.

## Structural induction follows a definition

Some objects are defined by constructors rather than a
single size. A binary tree in this section is either a
leaf or an internal node with exactly two children.
**Structural induction** proves a claim for each simple
constructor and then each composite constructor, assuming
the claim for its constituent objects. The definition
builds only finite trees here. Repeated descent reaches a
leaf, so the proof can also be expressed as strong
induction on the total number of nodes.

Let `L(t)` count leaves and `I(t)` count internal
nodes in tree `t`. We claim `L(t) = I(t) + 1`.
A leaf gives `L = 1` and `I = 0`. For a node
with children `left` and `right`, its leaf count is
`L(left) + L(right)`. By substitution this is
`I(left) + I(right) + 2`. The node's own internal
count is `I(left) + I(right) + 1`, so the required
equality follows. A node permitted to have just one
child would change this proof. The count property relies
on the exact constructor rules.

For a recursive implementation on this tree, the measure
“number of nodes in the current subtree” is positive
and decreases on every recursive edge. It establishes
termination for the finite tree. The maximum number of
simultaneously active calls can still grow with the
longest path. On an untrusted arbitrary-depth input,
request a depth bound or use explicit traversal storage
with a documented capacity policy. Terminating in
mathematics and fitting the call stack are separate
engineering obligations.

## Solve the balanced-work recurrence

A **recurrence** specifies a quantity through smaller
instances and base values. Let `T(n)` count the number
of counted item visits in a split computation. An empty
batch or singleton needs no combining visits, so
`T(0) = T(1) = 0`. For `n ≥ 2` the split sizes
are `floor(n/2)` and `ceil(n/2)`. Here floor rounds
down and ceiling rounds up. After both subproblems finish,
the current batch incurs exactly `n` visits. Consequently
`T(n) = T(floor(n/2)) + T(ceil(n/2)) + n`.
The smaller arguments are each strictly less than `n`
for `n ≥ 2`, which proves that the recursion reaches
its base cases.

For positive `n`, let `h` be the greatest integer
with `2^h ≤ n`. Each time a group is divided, the
sizes of its children differ by at most one. More
strongly, every group at depth `d` of a still-complete
level has size `floor(n / 2^d)` or `ceil(n / 2^d)`.
To verify that property, start with the single size
`n`. If the sizes at a level are equal, splitting
them gives at most two adjacent sizes. If adjacent
sizes are `q` and `q+1`, the smallest child size is
`floor(q/2)` and the largest is
`ceil((q+1)/2)`, which differ by at most one
whether `q` is odd or even. Since the sizes add to
`n` and there are `2^d` groups at a complete
level, the only possible sizes are the floor and
ceiling of that average.

For depths 0 through `h−1`, the smallest group has
at least two items. Each group splits, and the costs
sum to `n` on each of those `h` levels. At depth
`h` there are `2^h` groups of size one or two.
Starting with one item per group leaves `n − 2^h`
extra items, each making one two-item group. They
contribute two visits each and generate only singleton
children. Thus the exact cost for positive `n` is
`n × h + 2 × (n − 2^h)`.

| Input size | Visits by levels | Total |
| ---: | --- | ---: |
| 1 | none | 0 |
| 4 | 4 + 4 | 8 |
| 5 | 5 + 5 + 2 | 12 |
| 7 | 7 + 7 + 6 | 20 |
| 8 | 8 + 8 + 8 | 24 |

The recurrence gives the same number of item visits for
all batches of one length because the stated split and
combine work do not depend on the item values. For
`n ≥ 2`, the term `n × h` is a lower bound, and
`n × (h+1)` is an upper bound because the final
nonempty level visits at most `n` items. Hence the
count grows on the order of `n log₂ n`. These are
visits incurred by the modeled split computation. The
closed-form predictor below performs a fixed number of
word operations on a `u32` input.

**Executable Rust preview.** The number of leading zero
bits locates the most significant 1 of a nonzero word,
which gives `h`. The routine widens `n` before
multiplication.

```rust,edition2021
{{#include ../../mathematics/src/induction.rs:balanced_visits}}
assert_eq!(balanced_visits(0), 0);
assert_eq!(balanced_visits(5), 12);
assert_eq!(balanced_visits(8), 24);
```

For a 32-bit input `h ≤ 31`, and the result is at most
`32 × n`, which fits the output type. The code handles
zero before finding the leading 1. Tests compare the formula
with a recursive definition on both sides of split
boundaries. This checks how code expresses the model under
representative sizes. The proof connects the formula to
all sizes in the mathematical domain.

## Practice and assessment

{{#include ../../exercises/C008.md}}

## Summary and further reading

Specify the index or constructor that makes an inductive
proof progress. State the base case and identify where the
step uses an earlier result. A recurrence reports what a
recursive design costs under its chosen unit, while a
separate decreasing measure proves the recursion reaches a
base case.

[Sets, relations, functions, and graphs](007-sets-and-graphs.md) · [Solutions](../mathematics-solutions.md#chapter-008) · [Counting, probability, and modular arithmetic](009-counting-and-probability.md)
