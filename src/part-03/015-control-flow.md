# 015. Branches, loops, functions, and recursion

A queue reports the sizes of packages awaiting inspection.
An application needs to find the first package larger than
a configured limit, count all oversized packages, and
know whether those operations can stop early. The choice
of control flow follows from the result each operation
promises to return.

**Prerequisites.** [Induction and recurrence relations](../part-02/008-induction.md)
and [compound types and text](014-types-and-text.md).
After this chapter, you should be able to trace conditional
and repeated execution, write a function with an explicit
contract, and justify both the result and termination of
a recursive function.

## Choose among branches

A **condition** is an expression Rust checks as a Boolean.
When an `if` condition is true, Rust executes its first
block. Otherwise it executes the `else` block if there
is one. An `if` expression can also yield a value if
its branches have compatible result types.

```rust,edition2021
let size = 12_u16;
let limit = 10_u16;
let label = if size > limit {
    "inspect"
} else {
    "regular"
};
assert_eq!(label, "inspect");
```

Each branch's final expression gives a text view. Only the
selected branch evaluates. At equality the condition is
false, because “larger than” excludes the boundary. If
the policy means “at least as large,” write `>=` and
test equality as a separate case.

The operators `&&` and `||` evaluate from left to right
and short-circuit when the left operand determines the
answer. For example, in `divisor != 0 && total % divisor == 0`,
the remainder is evaluated only when the divisor is
nonzero. Reversing the two expressions would change
where the program can fail. Two predicates can have a
commutative mathematical truth table while their order
in a program still matters.
[Rust Reference on lazy Boolean operators](https://doc.rust-lang.org/1.75.0/reference/expressions/operator-expr.html#lazy-boolean-operators).

## Repeat with a purpose and an exit

A `while` loop evaluates its condition before each
iteration. It can execute zero times. A `for` loop
can visit the values of a range or sequence in order.
A `loop` repeats until control leaves it, for example
with `break` or a function return. The statement
`continue` skips the rest of the current iteration.
When an update is required for termination, ensure that
every continued path performs it.

```rust,edition2021
let mut total = 0_u16;
let mut next = 1_u16;
while next <= 4 {
    total += next;
    next += 1;
}
assert_eq!(total, 10);
```

| Before a condition check | next | total | Continue? |
| --- | ---: | ---: | --- |
| Initial | 1 | 0 | yes |
| After the first iteration | 2 | 1 | yes |
| After the second iteration | 3 | 3 | yes |
| After the third iteration | 4 | 6 | yes |
| After the fourth iteration | 5 | 10 | no |

*Figure 1. Each successful step adds the next integer
and then advances the counter.*

At each condition check, `total` is the sum of
integers from 1 through `next−1`. The empty sum
is 0 at initialization. Adding `next` and advancing
the counter preserves the relation. Before a successful
iteration, `5−next` is positive and decreases by
one. The first failed condition occurs at 5, so the
finished total covers 1 through 4. For a variable bound,
also show that incrementing past it remains representable,
or use a range with documented endpoints.

To apply a rule once per value of a slice, let a
`for` loop handle the traversal. The code below
uses a reference to compare each size without copying
the whole collection. The `&size` pattern gets a
copy of the individual `u16`, which is a small value.
If a size matches the requested condition, the count
increases by one.

```rust,edition2021
let sizes = [4_u16, 12, 10, 15];
let mut oversized = 0;
for &size in &sizes {
    if size > 10 {
        oversized += 1;
    }
}
assert_eq!(oversized, 2);
```

The invariant is that the count equals the number of
qualifying entries in the processed prefix. The loop
must visit each element to produce the exact count.
Returning at the first qualifying size can answer an
existence question, but it loses information needed
for an exact count.

## Use a function to name a contract

This function receives a borrowed slice and one limit.
The syntax `-> Option<usize>` declares its return type.
Here `Some(i)` supplies a valid position and `None`
reports no match. The function gives the earliest
oversized package in input order and never changes
the slice.

```rust,edition2021
fn first_oversized(sizes: &[u16], limit: u16) -> Option<usize> {
    for (index, &size) in sizes.iter().enumerate() {
        if size > limit {
            return Some(index);
        }
    }
    None
}

assert_eq!(first_oversized(&[4, 12, 10, 15], 10), Some(1));
assert_eq!(first_oversized(&[10, 9], 10), None);
assert_eq!(first_oversized(&[], 10), None);
```

The tuple pattern binds each position and value from
the enumeration. The `return` expression exits the
entire function immediately. A `break` without a
value would stop just the loop. If the loop reaches
its end, the final expression supplies `None`.
At the start of each iteration, no earlier index was
oversized. The current successful comparison therefore
returns the earliest possible answer. Empty input has
no candidate, so it yields absence. A list of length
`n` requires at most `n` comparisons and constant
traversal space.

## Choose recursion only after identifying the measure

A **recursive** function calls itself. Specify its
direct result on an easy input, called the base case,
and explain how each recursive call reduces a measure.
Consider asking whether an entire slice fits a size
limit. Split the slice into a first element and the
remaining slice. If there are no elements, the
universal condition holds. Otherwise check the first
value and recurse only when it fits.

```rust,edition2021
fn all_fit(sizes: &[u16], limit: u16) -> bool {
    if let Some((&first, rest)) = sizes.split_first() {
        first <= limit && all_fit(rest, limit)
    } else {
        true
    }
}

assert!(all_fit(&[], 10));
assert!(all_fit(&[2, 10, 7], 10));
assert!(!all_fit(&[2, 11, 7], 10));
```

The `if let` condition attempts to match the result
of splitting. If it matches, `first` holds the copied
element and `rest` refers to the original slice with
the first element omitted. No new elements are allocated.
The induction argument uses slice length. The empty
slice has no counterexample. On a nonempty slice the
first comparison must succeed, and correctness for its
strictly shorter tail follows from the hypothesis. The
function therefore returns true exactly when every
element obeys the bound.

In the worst case the routine checks all `n` items
and can have a chain of `n+1` calls, including the
base case. Rust 1.75.0 does not guarantee removal of
recursive calls through tail-call optimization. A
large input can therefore exhaust available stack
space. An iterative scan with early failure provides
the same decision with constant auxiliary space. The
recursive formulation is useful for making the proof
structure visible. For a long linear sequence, the
iterative formulation avoids a depth-dependent call
chain.

An observed infinite loop usually points to a condition
that can never become false or to a path that omitted
its progress step. Test empty and single-element inputs
before tracing longer ones. For a recursive function,
inspect both the base case and whether the first call
passes a genuinely smaller problem. A compiler may
flag an unconditionally recursive function, but
the termination argument for an input-dependent
recursion belongs to the program's design.

## Practice and assessment

{{#include ../../exercises/C015.md}}

## Summary

A branch selects a path or computes a value. A loop
needs a progress argument when its length depends on
state, and its invariant relates the result to the
work completed. A function names a contract. A
recursive implementation needs a decreasing measure
and a resource argument for the active calls.

[Compound types, Unicode, and strings](014-types-and-text.md) · [Solutions](../rust-fundamentals-solutions.md#chapter-015) · [Fundamentals midterm](../fundamentals-midterm.md) · [Addresses, pointer values, and stack frames](../part-04/016-addresses-and-stack-frames.md)
