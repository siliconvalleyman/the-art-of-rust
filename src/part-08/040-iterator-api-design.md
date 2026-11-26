# 040. Iterator API design and cost

A report needs the first two readings at or above a
threshold. An eager implementation can collect all qualified
readings and then discard the rest. A streaming version can
stop after the requested prefix. Both can satisfy the same
output specification. Their work and resource use depend
on how demand reaches the input.

**Prerequisites.** [Asymptotic cost models](../part-02/010-asymptotic-work.md),
[dispatch and trait objects](../part-07/034-dispatch-and-trait-objects.md),
and [map, filter, fold, and short-circuiting](039-adapters-and-reductions.md).
After this chapter, you should be able to state order and
evaluation guarantees, derive a traversal cost, and compare
an opaque iterator interface to a materialized collection.

## Specify the sequence independently of the implementation

Let the input contain `n` readings. The result is the
first `k` values at least `minimum`, in input order.
If fewer qualify, return all of them. A duplicate reading
counts as another position in the input. A zero budget
returns an empty sequence.

The pipeline first creates a shared slice iterator, then
copies each requested numeric item. Filtering tests the
inclusive threshold. Taking at most `k` stops the output
in the natural order. Placing `take` before `filter`
would inspect a prefix of the input, which can omit
eligible later positions even when the result is short.

```rust,edition2021
{{#include ../../examples/functional/src/prefixes.rs}}

let readings = [1, 7, 2, 8, 7];
let result: Vec<_> = qualified_prefix(&readings, 7, 2).collect();
assert_eq!(result, [7, 8]);
assert!(qualified_prefix(&readings, 7, 0).next().is_none());
```

The result declaration uses `impl Iterator` to expose
an **opaque** concrete iterator type. Callers know it
implements the named trait and that its item is u16.
They do not need to name the nested adapter type.
The `+ '_` includes the borrow from the input slice,
so the iterator cannot outlive its data. The contents
returned per item are copied integers. The API does
not clone or allocate a replacement input slice.

## Derive the amount of work from the stop condition

For zero demand, the chain can return absence without
inspecting an input. For positive demand `k`, let
`p` be the position, starting from one, of the kth
qualifying value. If that value exists and the caller
consumes the entire bounded result, the filter evaluates
the threshold on exactly the first `p` inputs.
Otherwise it inspects all `n` inputs. The reason is
that obtaining each filtered item consumes and tests
the preceding unprocessed inputs until a match, and
`take` has exhausted its budget after the kth match.

| Strategy | Threshold inspections | Additional materialized storage |
| --- | --- | --- |
| Scan all and collect before truncating | `n` | Holds all matched values before truncation |
| Consume a bounded iterator without collecting | At most `n`, with the stop rule above | Constant traversal state |
| Collect the bounded result | Same bounded inspection rule | Holds the returned values |

The eager storage comparison describes an implementation
that actually materializes all matches before truncation.
Rust does not promise a Vec's exact growth factors or
capacity. If a caller requests only the first item of
an iterator, the work can stop even earlier. Where
callbacks have effects, both traversal and stopping order
may therefore be part of the API contract.

The fixed chain can use static dispatch, allowing the
compiler to specialize the adapters for its known types.
This is an optimization opportunity rather than a timing
result. A boxed trait-object iterator can be useful when
an implementation must return different iterator types
depending on runtime choices. That design changes the
ownership and dispatch model. Another option is a custom
enum iterator for the finite set of alternatives.

## Verify semantics with a different implementation

For a finite domain, derive a reference by scanning the
entire input into a list and truncating it afterward.
Compared with the lazy implementation, the eager
version has a different control strategy. Agreement tests
the same selection, order, and handling of duplicates.
The supplied tests exhaust lists of four items from
0 through 3 and vary both the minimum and maximum
result size. These tests do not measure performance.

To confirm how many predicate calls occur for a particular
shape, use an instrumented predicate separately. A
counter introduces an effect that you can intentionally
observe. To compare real runtimes, also state the target,
build settings, input distributions, and what is
materialized, because those variables can dominate the
cost.

## Practice and assessment

{{#include ../../exercises/C040.md}}

## Summary and further reading

Describe a sequence's order, inclusiveness, and empty
cases. Choose between returning a borrow-dependent iterator
and allocating an output collection. Derive work from
the input traversed and the demand; test semantics and
measure costs separately.
[Rust 1.75.0 Reference on impl Trait](https://doc.rust-lang.org/1.75.0/reference/types/impl-trait.html).

[Adapters and reductions](039-adapters-and-reductions.md) · [Failures and invariants](../part-09/041-failures-and-invariants.md) · [Solutions](../functional-solutions.md#chapter-040)
