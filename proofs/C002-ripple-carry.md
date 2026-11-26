**Contract.** Given two unsigned values `a` and `b` of width
`w ≥ 1`, return `(s, c)`, where `0 ≤ s < 2^w`, `c` is 0
or 1, and `a + b = s + 2^w × c`. The algorithm accepts every
input pair in that domain. An interface that accepts wider
numbers must validate width or specify how it reduces the inputs.

**Derivation and invariant.** Let `a_i` and `b_i` be the input
bits at position `i`, and let `a_low(k)` and `b_low(k)`
be the values of their lowest `k` bits. After processing `k`
columns, let `s_k` be the output bits already written and
`c_k` the carry into the next column. The invariant is
`a_low(k) + b_low(k) = s_k + 2^k × c_k`. We require additionally
that `0 ≤ s_k < 2^k` and `c_k` is 0 or 1.

**Initialization.** For `k = 0`, each low part is empty with
value 0. The output is initially 0, and there is no carry.

**Maintenance.** The next column adds
`2^k × (a_k + b_k)` to the left side. Substitute the invariant.
The terms of weight `2^k` combine to form
`t = a_k + b_k + c_k`. Its range is 0 through 3. Write
`t = r + 2 × c_next`, where `r` is its remainder on
division by 2. Putting `r` in position `k` changes the
output to `s_k + 2^k × r`. The other term is
`2^(k+1) × c_next`. The new output fits in `k+1` bits,
and the new carry is 0 or 1. The invariant now holds with
`k+1` in place of `k`.

**Termination and postcondition.** Initially there are `w`
unprocessed positions. Each iteration processes one, decreasing
`w − k` until it reaches zero. At that point the two low
parts are the complete inputs. The invariant and its bounds
are exactly the contract.

**Costs and verification.** Counting constant-cost bit operations,
the loop evaluates exactly `w` columns for every input, so best,
average, and worst time are all proportional to `w`. It needs
one carry and one iteration position beyond input and output, with
no allocation or recursion in the byte implementation. That
implementation extracts each input bit, forms `t` in a byte
where it fits, and writes the remainder to an unoccupied bit
position. The test suite checks all 65,536 input pairs by widening
the sum to a type that holds 510. The finite check applies to the
tested byte implementation. The invariant explains the algorithm
for any stated positive width.
