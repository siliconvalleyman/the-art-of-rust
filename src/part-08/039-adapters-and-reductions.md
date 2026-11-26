# 039. Map, filter, fold, reduce, and short-circuiting

An input file yields fields that must be parsed and
combined. Some operations select or transform individual
values. Others compress a whole prefix into one accumulator.
A reduction also needs a rule for empty input and an
explicit decision about whether failure stops later work.

**Prerequisites.** [Option and Result](../part-06/028-option-and-result.md)
and [iterator contracts](038-iterators-and-laziness.md).
After this chapter, you should be able to trace a chain
of adapters, pick an appropriate reduction, and prove that
a fallible reduction stops on the first relevant error.

## Separate selection from transformation

`map` produces one transformed item for each item requested
from a source. `filter` emits only items satisfying a
predicate. A request to a filter can advance through
several rejected items before a match. These operations
do not in general commute. Mapping a signed reading to
its magnitude before filtering for positive numbers
can admit negative readings that would have been rejected
by filtering first.

```rust,edition2021
let selected: Vec<_> = [-3_i16, 0, 4].into_iter()
    .filter(|&reading| reading > 0)
    .map(i16::abs)
    .collect();
let magnitudes: Vec<_> = [-3_i16, 0, 4].into_iter()
    .map(i16::abs)
    .filter(|&reading| reading > 0)
    .collect();
assert_eq!(selected, [4]);
assert_eq!(magnitudes, [3, 4]);
```

The bounds of the demonstration exclude the signed minimum,
whose positive magnitude does not fit in `i16`. For
arbitrary input, choose a representation or operation that
can handle that boundary. Changing adapter order may also
change which operations encounter a problematic value.

## Choose an initial accumulator deliberately

`fold(initial, combine)` starts with a supplied
accumulator and updates it for each item in sequence.
For a count, zero represents the empty prefix. For
an output String, the accumulator might be an empty
buffer. The accumulator's type can differ from the
item type.

`reduce(combine)` uses the first item as its
initial state and returns an Option. This helps
when there is no natural result for empty input. In
the greatest-byte example, zero is a possible valid
input. Returning None on empty input keeps the
distinction from a byte list with greatest value zero.

```rust,edition2021
{{#include ../../examples/functional/src/reductions.rs}}

assert_eq!(positive_count(&[-4, 0, 2, 3]), 2);
assert_eq!(greatest(&[]), None);
assert_eq!(greatest(&[0]), Some(0));
assert_eq!(greatest(&[2, 9, 1]), Some(9));
```

For the greatest value, the nonempty-prefix invariant
is that the accumulator equals the maximum of the
processed prefix. Taking the maximum with the next item
preserves it. A prefix count similarly maintains the
number of selected items processed so far.
Both operations use constant additional accumulator space
and take time proportional to the input length when
running to completion on these slice examples.

## Preserve the first failure

The field sum parses each supplied text using the
standard `u32` parser. A parse failure is recorded
with the field index. Each successfully parsed number
is added with checked arithmetic. An overflow is also
recorded with the current index.

```rust,edition2021
{{#include ../../examples/functional/src/reductions.rs}}

assert_eq!(sum_fields(&[]), Ok(0));
assert_eq!(sum_fields(&["2", "3"]), Ok(5));
assert_eq!(sum_fields(&["4294967295", "1", "bad"]), Err(SumError::Overflow(1)));
```

Here `map` only parses a field when the downstream
operation requests it. `try_fold` stops on the first
error returned by its combining closure. A parse
error short-circuits the closure at `field?`. An
arithmetic error short-circuits because checked addition
returns failure. After a valid prefix the accumulator
equals its mathematical sum. Each failure identifies
the earliest index where that step cannot perform
its contract. Later fields are not requested by
this operation.

An eager intermediate parser collecting a list before
adding could instead notice a later parse error before
the earlier overflow. Reordering those phases affects
diagnostics. The lazy chain checks one field and
then tries to apply it. This order belongs to the
interface when callers use the failing index.

## Practice and assessment

{{#include ../../exercises/C039.md}}

## Summary and further reading

Map changes items and filter changes which items
flow downstream. Fold supplies an initial accumulator.
Reduce needs a first item. Fallible consumers can stop
an otherwise lazy chain as soon as an error matters.
[Rust 1.75.0 Iterator methods](https://doc.rust-lang.org/1.75.0/std/iter/trait.Iterator.html).

[Iterators and laziness](038-iterators-and-laziness.md) · [Solutions](../functional-solutions.md#chapter-039) · [Iterator API design and cost](040-iterator-api-design.md)
