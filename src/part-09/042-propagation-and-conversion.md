# 042. Propagation, question mark, and error conversion

A quantity loader reads a source and parses its text. It can fail
while reading UTF-8 or while interpreting the resulting string as a
number. A caller may be able to retry or replace an input source for
one kind of failure, and correct the content for the other.

**Prerequisites.** [Coherence](../part-07/033-coherence-and-orphan-rules.md)
and [recoverable failures](041-failures-and-invariants.md).
After this chapter, you should be able to expand `?` on `Result`,
trace the first failing operation, and choose an error conversion.

## Name the possible error variants

The loader accepts any `Read`, which lets tests supply a byte slice or
a reader that generates a specific failure. The return type uses
separate variants for the two error types. Implementations of `From`
tell Rust how to build the enclosing error without losing the original.
`read_to_string` reads through end of input and validates UTF-8. The
example assumes a short, finite input. For a stream or a caller-supplied
file with an unknown size, set a read budget before allocating.

```rust,edition2021
{{#include ../../examples/reliability/src/quantities.rs}}

assert_eq!(load_quantity(&b" 65535\r\n"[..]).unwrap(), u16::MAX);
assert!(matches!(load_quantity(&b"65536"[..]), Err(LoadQuantityError::Parse(_))));
assert!(matches!(load_quantity(&[0xff][..]), Err(LoadQuantityError::Read(_))));
```

For a `Result<T, E>`, an expression of the form `operation()?`
yields the successful `T` or returns early with the converted error.
In this function the first application can be expanded as follows.

```rust,edition2021
{{#include ../../examples/reliability/src/quantities.rs}}

fn explicit(mut input: impl io::Read) -> Result<u16, LoadQuantityError> {
    let mut text = String::new();
    match input.read_to_string(&mut text) {
        Ok(_) => (),
        Err(error) => return Err(LoadQuantityError::from(error)),
    }
    match text.trim().parse::<u16>() {
        Ok(quantity) => Ok(quantity),
        Err(error) => Err(LoadQuantityError::from(error)),
    }
}

assert_eq!(explicit(&b" 7 "[..]).unwrap(), 7);
```

The return is from the function or closure body containing `?`.
The final `Ok(...)` wraps a successful value in the enclosing result.
An early return stops the rest of the body and drops locals in the
ordinary return path. For a `Read` that already consumed bytes, the
return does not move the source cursor backward. Choose where to
commit changes if a later operation can fail.

## Put conversion at the right layer

A `From<E>` implementation provides one conventional, infallible
conversion into the destination type. If the same I/O failure needs a
different variant when opening and when writing, choose each mapping
at its call site with `map_err`. This also gives that site access to
context such as a row index or operation name.

An `Option<T>` produces a value on `Some` and returns `None` on `?`
in an `Option` function. To use absence as failure inside a `Result`
function, specify its meaning with `ok_or` or `ok_or_else`, then use
`?`. The enclosing function's error type still needs a compatible
conversion. The textbook expansion here covers `Result`; the language
reference lists the supported operand types and their behaviors.

If a finite source has `n` bytes, the loader fetches those bytes and
holds the resulting string. UTF-8 validation, trimming, and integer
parsing each traverse at most that input. Its local processing work
and storage are `O(n)`, separate from I/O latency and allocator costs.

## Practice and assessment

{{#include ../../exercises/C042.md}}

## Summary and further reading

Follow `?` as an early branch with conversion. Verify each branch
and attach contextual information at the boundary that knows it.
See the [Rust 1.75.0 reference on the question-mark operator](https://doc.rust-lang.org/1.75.0/reference/expressions/operator-expr.html#the-question-mark-operator).

[Failures and invariants](041-failures-and-invariants.md) · [Context and error sources](043-context-and-sources.md) · [Solutions](../reliability-solutions.md#chapter-042)
