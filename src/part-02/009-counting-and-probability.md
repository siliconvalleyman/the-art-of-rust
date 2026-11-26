# 009. Counting, probability, and modular arithmetic

A checker must test pairs of distinct devices. An assignment
chooses one slot in a circular buffer. A reliability analysis
asks how often a sample contains a faulty device. These
problems share a habit of thought: describe the possible
cases first, then count, assign probabilities, or group them
by remainder.

**Prerequisite.** [Induction, strong induction, and recurrence relations](008-induction.md),
including [sets and relations](007-sets-and-graphs.md).

After this chapter, you should be able to count ordered and
unordered selections, compute finite probabilities under
explicit assumptions, derive a safe modular index update,
and prove the remainder-based Euclidean algorithm.

## Count a well-defined collection

The **sum rule** adds the sizes of disjoint cases.
For example, if a batch consists of five checked devices and
three unchecked devices, and every device belongs to exactly
one case, the batch size is eight. When the cases overlap,
adding their sizes counts shared elements twice. For two
finite sets `A` and `B`, their union has size
`|A| + |B| − |A ∩ B|`, because subtracting the overlap
removes the extra copy of each shared member.

The **product rule** multiplies counts for successive
choices when each first choice admits the same number of
second choices. If there are four data formats and three
devices for each format, twelve format-device pairs are
possible. If one format supports just one device, split
by format and sum the resulting counts. A product of
counts describes choices. It does not imply that any
random variables used to choose them are independent.

For `n ≥ 2` distinct devices, choose the first of an
ordered pair in `n` ways. The second must be different, so
there are `n−1` choices. This gives `n(n−1)` ordered
pairs. If the order is immaterial, each unordered pair
has exactly two orderings. The count is
`n(n−1)/2`, also written `choose(n, 2)`. The count
is zero for `n = 0` or 1.

More generally an ordered choice of `k` distinct items
from `n`, with `0 ≤ k ≤ n`, has
`n(n−1)⋯(n−k+1)` possibilities. The symbol `⋯`
means continue the product through the stated last term.
For `k = 0` the empty product represents one empty
selection. An **arrangement** of all `n` distinct items
has `n!` possibilities, where `n!` is the product of
the integers 1 through `n` and `0! = 1`. Forgetting
positions turns ordered selections into **combinations**.
Each combination of `k` distinct items had `k!`
orders, so `choose(n, k) = n! / (k!(n−k)!)`. The
factorial expression helps derive the count. Computing each
factorial in a fixed-width type can overflow even when a
particular quotient fits.

## Probabilities depend on the sampling rule

A **sample space** is a set of possible outcomes under a
specified experiment. An **event** is a subset of those
outcomes. If the sample space is finite and its outcomes
are equally likely, an event's probability is the number
of outcomes in it divided by the total count. The
probability of an event's complement is one minus the
event's probability.

Suppose a rack has four known-good devices and two that
fail a check. A test selects two distinct devices uniformly.
If applied as two ordered selections without replacement,
the 30 ordered pairs each have probability `1/30`.
The unordered pairs are also uniform because each pair
accounts for two orders. There are `choose(6, 2) = 15)
such pairs. Both devices pass for
`choose(4, 2) = 6` pairs. The probability is `6/15`,
and at least one failure has probability `9/15`.

| Sampling design | Possible ordered draws | Draws with no failure |
| --- | ---: | ---: |
| Uniform without replacement | `6 × 5 = 30` | `4 × 3 = 12` |
| Uniform with replacement | `6 × 6 = 36` | `4 × 4 = 16` |

With replacement, the draws are independent when each
choice is made under the same uniform rule regardless of
what happened before it. Without replacement, knowing
that the first device passed leaves three passing devices
among the five available. That changes the probability for
the second draw. The **conditional probability** of event
`B` given `A`, written `Pr(B | A)`, is
`Pr(A ∩ B) / Pr(A)` when `Pr(A) > 0`. Here
the probability of the first pass and then the second is
`(4/6) × (3/5)`. Multiplying the unconditional pass
probabilities would give the with-replacement answer.

A numerical outcome defines a **random variable**. The
**expected value** is the sum of each numerical value
weighted by its probability. Let `X` count failures
in the sample without replacement. It is the sum of
two indicators, each 1 if its draw fails and 0 otherwise.
Each draw has marginal probability `2/6` of selecting a
failing device. On the second draw this follows from the
symmetry of the uniform ordered pairs. Each failing device
can occur second with each of five different first
devices, giving `2 × 5` failing second draws out of
30. The expected number of failures is
`2/6 + 2/6 = 2/3`. This use of linearity needs no
independence: summing `(X₁ + X₂) × probability` over
the same outcomes distributes into the two separate sums.
An expectation is a long-run weighted mean. A single
sample produces an integer count.

## Remainders classify integers

For an integer `x` and positive modulus `m`, Euclidean
division writes `x = qm + r` with `0 ≤ r < m`.
The integer `q` is the quotient and `r` is the
Euclidean remainder. The statement `x ≡ y (mod m)`
says that `x−y` is a multiple of `m`. Equivalently
they have the same Euclidean remainder. If the
remainders agree, subtracting their division equations
leaves a multiple of `m`. Conversely, if their
difference is a multiple, their remainder difference
is also a multiple, and its magnitude is less than
`m`. The only such multiple is zero.

Congruence classes group together integers with the same
remainder. Adding congruent values preserves congruence:
the difference of the sums is the sum of the original
differences, each a multiple of `m`. Multiplication
preserves it as well: writing `a = r + km` and
`b = s + lm` makes `ab−rs` a sum of terms
containing `m`. These justify reducing operands before
computing a result modulo `m). Division has extra
conditions. For example, 2 and 8 are congruent modulo
6, but their integer halves 1 and 4 are in different
classes modulo 6.

## Apply a modular index with bounded arithmetic

Let an index `i` satisfy `0 ≤ i < m`, where
`m > 0` is a ring's capacity, and let `d` be a
nonnegative advance. The desired output is the unique
remainder of `i+d` modulo `m`. First reduce
`r = d mod m`. A common mathematical implementation
would add `i+r` and subtract `m` if necessary.
That intermediate sum can exceed the maximum representable
integer when capacities are close to the type maximum.

Compare the reduced step with the remaining distance
`m−i` instead. If `r < m−i`, then `i+r < m`
and the addition fits. Otherwise the step reaches or passes
the boundary and the result is `r − (m−i)`, which lies
in `0..m` with the upper endpoint excluded. Both candidates
are congruent to `i+d` and in the required range. Neither
requires a nonrepresentable sum or a negative subtraction.
For `m = 8`, `i = 6`, and `d = 11`, the reduced
step is 3 and the remaining distance is 2. The output is 1.
If capacity is zero or the starting index is outside the
range, return a validation error instead of performing a
remainder operation.

This method has constant work under a fixed-width
arithmetic model. It specifies logical index advancement.
A real ring buffer also has occupancy state that distinguishes
empty from full and controls which accesses are allowed.

## Preserve common divisors while shrinking inputs

For two nonnegative integers `a) and `b), the
**greatest common divisor**, written `gcd(a,b)`, is the
greatest positive integer dividing both when at least one
input is positive. This implementation defines
`gcd(0,0) = 0` as a convention. For `b > 0` write
`a = qb + r`. If a positive integer divides both
`a` and `b`, it divides `r = a − qb`. If it
divides `b` and `r`, it divides
`a = qb + r`. The pairs `(a,b)` and `(b,r)`
therefore have the same common divisors.

The **Euclidean algorithm** repeatedly makes this
replacement until its second argument is zero. The invariant
is that the current pair has the same common divisors as
the original pair. Initially the pairs agree. The
division relation proves preservation. The second
argument strictly decreases, since `0 ≤ r < b`.
At termination every positive divisor of the first
argument also divides zero, so a positive first
argument is itself the greatest common divisor. A zero
first argument at the exit can arise only from the original
`(0,0)` case.

| Pair before step | Remainder | Pair after step |
| --- | ---: | --- |
| `(84, 30)` | 24 | `(30, 24)` |
| `(30, 24)` | 6 | `(24, 6)` |
| `(24, 6)` | 0 | `(6, 0)` |

When `b = 0` the loop takes no steps. When the first
remainder is zero, there is one step. For other
inputs, after at most two steps the second argument drops
below half its old value. If the first remainder is
less than half, the first step does that. If the remainder
is at least half, the next division subtracts it once
or more, leaving at most the old value minus that
remainder, and strictly less than half when the first
remainder equals half (then the next remainder is zero).
Thus the number of steps is bounded by a constant
times the number of bits in the original positive second
argument. The loop uses constant storage with a fixed
input width. A realistic average needs a distribution over
input pairs. The iteration bound requires none.

**Executable Rust preview.**

```rust,edition2021
{{#include ../../mathematics/src/counting.rs:gcd}}
assert_eq!(gcd(84, 30), 6);
assert_eq!(gcd(0, 17), 17);
assert_eq!(gcd(0, 0), 0);
```

The remainder-based variant uses the same divisibility
argument as the repeated-subtraction construction in
[Euclid's Elements, Book VII, Proposition 2](https://mathcs.clarku.edu/~djoyce/java/elements/bookVII/propVII2.html).

## Practice and assessment

{{#include ../../exercises/C009.md}}

## Summary

Count cases from their generating rule. State replacement
and uniformity assumptions before computing probabilities.
Reduce remainders under a clearly stated modulus. For
Euclid's algorithm, common divisors stay fixed while the
second argument decreases.

[Induction, strong induction, and recurrence relations](008-induction.md) · [Solutions](../mathematics-solutions.md#chapter-009) · [Asymptotic reasoning and cost models](010-asymptotic-work.md)
