# 041. Recoverable failures, invariants, and assertions

When two customers request the last item in stock, the second request
can fail during ordinary operation. The caller needs to see how many
units are available so it can make another choice. If the program
instead finds more units available than the inventory ever contained,
the stored numbers contradict the type's contract. These cases call
for different responses.

**Prerequisites.** [Resource guards](../part-05/025-raii-and-drop.md)
and [absence and recoverable failure](../part-06/028-option-and-result.md).
After this chapter, you should be able to classify failures at an API
boundary and prove that a rejected operation preserves the prior state.

## Decide what the caller can do

| Situation | What the caller can do | Representation |
| --- | --- | --- |
| A completed search found no match | Continue with an empty outcome | `Option<T>` |
| A request exceeds available stock | Try a smaller request or report the shortage | `Result<(), InsufficientStock>` |
| Private inventory fields contradict each other | Diagnose a defect in the implementation | A checked internal invariant, with an assertion when a panic fits the boundary |

Absence can itself be an error when the contract requires a value.
A search for an optional coupon may end with `None`. An order that
names a missing product can need `Err(UnknownProduct)`. The operation's
contract determines the meaning. Errors from untrusted input should
have a defined ordinary path, including malformed data and supported
sizes that exceed a configured limit.

## Validate before mutating

`Stock` holds total and available quantities in private fields. While
`available <= total`, the number held is `total - available`. The
reservation method checks subtraction before assigning the result.
The error carries the request and the available stock at rejection.

```rust,edition2021
{{#include ../../examples/reliability/src/stock.rs}}

let mut stock = Stock::new(5);
assert_eq!(stock.reserve(3), Ok(()));
let before = stock;
assert_eq!(stock.reserve(4), Err(InsufficientStock {
    requested: 4,
    available: 2,
}));
assert_eq!(stock, before);
assert_eq!(stock.held(), 3);
```

Initialization assigns `available = total`. Suppose the invariant
holds before a request. With sufficient stock, checked subtraction
yields a value between zero and the previous availability. This
preserves the upper bound. With insufficient stock, `?` returns
before the assignment, so both fields retain their values. Induction
therefore extends the argument to any sequence of public operations.
The method takes constant time and uses constant extra storage.

## Choose the assertion and its build behavior

The `debug_assert!` in `held` is a diagnostic for programmers. Whether
it executes is controlled by debug assertions in the chosen build
profile. The proof of subtraction safety depends on the private state
and the methods that maintain it. Parsing input or enforcing a bound
must still run in an optimized build.

An ordinary `assert!` checks its condition in release builds too.
Use it when a violation should trigger a panic at that boundary.
Both forms evaluate the condition when enabled, so keep state changes
outside the condition. With `assert_eq!` in tests, the compared values
give evidence about observable behavior without exposing private fields.

The tests enumerate all 65,536 pairs of `u8` initial counts and requests.
A separate sequence checks a rejection after a successful reservation.
The exhaustive count establishes coverage for the two-argument domain
of the first experiment. The invariant argument covers longer histories.

## Practice and assessment

{{#include ../../exercises/C041.md}}

## Summary and further reading

Model a normal shortage as a recoverable result. Constrain state with
private fields, and validate a proposed update before publication.
Consult the [Rust 1.75.0 book on panic and recovery](https://doc.rust-lang.org/1.75.0/book/ch09-03-to-panic-or-not-to-panic.html#guidelines-for-error-handling).

[Iterator API design](../part-08/040-iterator-api-design.md) · [Error propagation](042-propagation-and-conversion.md) · [Solutions](../reliability-solutions.md#chapter-041)
