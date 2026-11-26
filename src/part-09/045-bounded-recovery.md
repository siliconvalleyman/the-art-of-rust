# 045. Robust APIs and defensive recovery

An inventory editor accepts one quantity per line and reserves the
requested units. If any line fails, the editor should see which
request failed and the inventory should retain all of its prior
quantities. A small input budget bounds work even for failing calls.

**Prerequisites.** [Typed transitions](../part-06/030-typestate-and-transitions.md),
[bounded iterator demand](../part-08/040-iterator-api-design.md),
and [panic boundaries](044-panic-boundaries.md).
After this chapter, you should be able to specify limits and error
precedence, then verify that success and failure publish distinct
state outcomes.

## Define the input and the order of errors

The caller supplies UTF-8 as `&str`. At most 64 bytes and four lines
are accepted. `str::lines` recognizes line feed and carriage return
followed by line feed. A final line ending does not add another line,
and the empty string means zero operations. Interior empty lines
are evaluated as fields. The numeric grammar is Rust 1.75.0's
`str::parse::<u8>()`, so leading plus and leading zeros are accepted,
with whitespace and values outside zero through 255 rejected.

1. Reject excess bytes before interpretation. The limit counts
   encoded bytes, including line delimiters.
2. Visit lines in order. If a fifth line is reached, report the line
   limit before interpreting that line.
3. For each admitted line, report its parse error first, if any.
   Otherwise report a reservation failure using the stock that would
   remain after earlier requests in the same batch.
4. Publish the complete draft only after every line succeeds.

An invalid second line therefore takes priority over a fifth line.
The byte count takes priority over both. This rule lets tests verify
inputs that break multiple rules at once without relying on incidental
evaluation order.

```rust,edition2021
mod row_errors {
    {{#include ../../examples/reliability/src/row_errors.rs}}
}
mod stock {
    {{#include ../../examples/reliability/src/stock.rs}}
}
{{#include ../../examples/reliability/src/batches.rs}}

# fn main() {
    let mut inventory = Stock::new(6);
    assert!(apply_reservations(&mut inventory, "2\r\n3\r\n").is_ok());
    assert_eq!(inventory.available(), 1);
    let snapshot = inventory;
    assert!(matches!(apply_reservations(&mut inventory, "1\n1"),
        Err(BatchError::Stock { line: 2, .. })));
    assert_eq!(inventory, snapshot);
# }
```

## Prove what stays private until commit

The draft starts as an exact copy of the prior stock. After `i`
successful rows, its availability is the starting availability minus
the sum of the first `i` requests. The initial case subtracts zero.
Each successful checked subtraction gives the next step. On every
error return the assignment to the caller's state is still ahead,
so the prior stock survives. After the last success the assignment
publishes the draft for all the rows. `&mut Stock` gives the function
exclusive access during this call.

If the input has `n` admitted bytes, iterating and parsing take
`O(n)` local work. The byte-limit test on the already borrowed slice
takes constant time, including for a rejected oversized slice. The
function allocates no per-row storage and uses a constant-size draft.
A network reader or file loader needs its own bound before it creates
the slice. This example commits a single in-memory value and has no
external storage effects.

Test the exact byte limit and its successor, line-limit precedence,
an interior empty line, CRLF, and a failure following successful
staging. Where recovery means a retry, use a corrected request. After
a successful batch, submitting the same input again requests another
reservation. Repeat-safe external requests require an identity and a
separate deduplication policy.

## Practice and assessment

{{#include ../../exercises/C045.md}}

## Summary and further reading

Define a size limit, error precedence, and a publication point before
writing a fallible update. Verify how the previous state survives each
failure path. See [Rust 1.75.0 str::lines](https://doc.rust-lang.org/1.75.0/std/primitive.str.html#method.lines).

[Panic boundaries](044-panic-boundaries.md) · [Core Rust midterm](../core-rust-midterm.md) · [Arrays, stacks, and queues](../part-10/046-arrays-stacks-and-queues.md) · [Solutions](../reliability-solutions.md#chapter-045)
