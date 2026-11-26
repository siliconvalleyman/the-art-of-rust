# 043. Custom errors, source chains, and context

A parser can tell a spreadsheet importer that a quantity was
invalid. The importer knows the row. A good error represents both
facts so it can report a location and preserve the underlying cause.

**Prerequisite.** [Error propagation and conversion](042-propagation-and-conversion.md).
After this chapter, you should be able to design an error type, explain
which fields belong in its message, and inspect its source chain.

## Give each layer enough information to act

`RowError` stores the line number and the original `ParseIntError`.
The caller supplies a one-based line number. The type implements
`Display` for an actionable summary and `Error::source` to expose the
cause to code that walks or classifies the chain. This example leaves
the input value out of the error entirely. A row may contain private
or unexpectedly long content. A location can be enough to find it
in the original file.

```rust,edition2021
{{#include ../../examples/reliability/src/row_errors.rs}}

let error = parse_row(12, "private input").unwrap_err();
assert_eq!(error.line(), 12);
assert_eq!(error.to_string(), "invalid quantity on line 12");
assert!(error.source().unwrap().downcast_ref::<ParseIntError>().is_some());
```

A call to `Display` through formatting shows the message for that
error. Displaying a wrapper does not automatically walk `source`.
At a boundary that controls diagnostic output, a reporter can choose
to inspect the rest of the chain. Apply the disclosure policy to each
source before displaying it. Payload-free wrapper text alone cannot
control how a source formats itself.

## Read the lifetime in the source signature

The outer reference in `Option<&(dyn Error + 'static)>` has the
lifetime of the borrow from `self`. The `+ 'static` bounds the erased
error type. The caller cannot hold onto the reference after dropping
the `RowError`. This distinction also lets `Error` offer a concrete
downcast for sources with a known type, as in the example.

Machine-readable decisions should use stable structure. A program can
match an error variant, inspect a line number, or compare a documented
parse kind. It should avoid searching the human-facing text for a
particular English phrase. If a public enum exposes all its variants
to an exhaustive match, adding one can require caller code changes.
An opaque error with methods lets the author expose a narrower
classification contract.

Wrapping a parse failure here adds constant-size context with no copy
of the rejected field. Parsing visits at most the provided slice.
Formatting the message has its own cost, paid when someone asks for
text. The test verifies both the outer message and the underlying
concrete cause.

## Practice and assessment

{{#include ../../exercises/C043.md}}

## Summary and further reading

Keep a useful location at the layer that knows it. Give the caller
both a display message and a source it can inspect.
See [Rust 1.75.0 Error::source](https://doc.rust-lang.org/1.75.0/std/error/trait.Error.html#method.source).

[Error propagation](042-propagation-and-conversion.md) · [Panic boundaries](044-panic-boundaries.md) · [Solutions](../reliability-solutions.md#chapter-043)
