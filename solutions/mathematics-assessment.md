## Integrated practice answers

1. The implementation computes `∃ r, r ≤ 255`. The
   intended nonnegative-batch rule is
   `nonempty ∧ (∀ r, r ≤ 255)`. For lengths `[1, 256]`
   the implementation accepts and the specification rejects.
   Check nonemptiness and then reject at the first length
   outside the range. Empty batches are rejected under the
   extended rule. Award one point each for the two quantified
   forms, the counterexample, and the fixed empty behavior.
   Checking only a first valid row cannot prove a universal.
2. The witnesses are B and C. For example, delete C→D
   and insert C→B as its replacement. Then B remains a
   witness, and a walk A→C→B→D has three edges. A direct
   A→D edge supplies no intermediary by itself. Award a
   point for each witness, the valid edge replacement, and
   the explanation of length. Counting just any walk cannot
   distinguish the query's fixed-length requirement.
3. At zero both sides are zero. Assuming a sum of `k²`,
   the next odd integer is `2k+1`. Adding it gives
   `k² + 2k + 1 = (k+1)²`. Widen `n` before the
   multiplication. Its maximum square is
   `(2^32−1)² < 2^64` but exceeds the 32-bit maximum.
   Award one point each for the base, the step, the bound,
   and locating the widening. A correct algebraic formula
   alone leaves the implementation width unresolved.
4. The unordered pairs are uniform because the ordered
   draws without replacement are uniform and each unordered
   pair has exactly two orders. Out of 28 unordered pairs,
   10 contain only good devices. The probability of at
   least one bad device is `18/28 = 9/14`. Each draw
   has a bad-device marginal of `3/8`, so the expected
   count is `3/4`. Award one point each for the sample
   space, complement, marginals, and expectation. Multiplying
   unconditional probabilities across the draws would assume
   independence that the experiment lacks.
5. Growth copies occur on insertions 2, 4, and 10.
   They copy 1, 3, and 9 elements, for a total of 13.
   The tenth insertion costs ten units. For `n ≥ 1`,
   final capacity `C` is the smallest power of three
   at least `n` and satisfies `C < 3n`. Copies sum
   to `(C−1)/2` by the geometric series identity.
   This is less than `3n/2`, so updates cost constant
   amortized work. Award one point each for the growth
   events, exact copy total, most costly insertion, and
   upper-bound derivation. A bound on an entire sequence
   alone cannot identify the cost of its most expensive step.
