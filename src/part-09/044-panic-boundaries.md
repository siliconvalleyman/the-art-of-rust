# 044. Panic, unwinding, and recovery boundaries

A score renderer publishes a new value only when its worker finishes.
If a controlled worker demonstration panics, the publication boundary
can keep the previous score. A worker that wrote to a file before it
panicked would already have produced an external effect. A panic
boundary cannot retroactively remove that effect.

**Prerequisites.** [Resource guards](../part-05/025-raii-and-drop.md)
and [error sources](043-context-and-sources.md).
After this chapter, you should be able to distinguish unwinding from
abort, predict destructor behavior, and identify what a catch boundary
actually protects.

## Distinguish stack unwinding from process abort

With unwinding enabled, a panic can unwind to an enclosing catch
boundary. In ordinary unwinding, values leaving scope are dropped.
A panic during unwinding can itself cause abort. With the abort
strategy a Rust panic terminates the process without unwinding the
stack. A catch boundary does not see such an abort. Check the selected
target and Cargo profile before depending on the unwind strategy.

| Event at the worker | Effect at this publication boundary |
| --- | --- |
| Return a computed value | Replace the previously published score |
| Unwind with a static string payload | Return `Err(WorkerPanicked)` and preserve the score |
| Unwind with another payload type | Resume unwinding before publishing |
| Abort the process | Execution does not return to the caller |

`catch_unwind` calls the closure once. On an unwind it returns a boxed
panic payload. This example handles a static string payload with
straightforward destruction. It forwards an unfamiliar payload with
`resume_unwind` because arbitrary payloads can themselves have code
that runs on drop. The application owns the policy for which workers
and panic types it accepts.

```rust,edition2021
{{#include ../../examples/reliability/src/boundaries.rs}}

let mut score = 7;
assert_eq!(publish_score(&mut score, || 12), Ok(()));
assert_eq!(score, 12);

#[cfg(panic = "unwind")]
{
    let old = score;
    let outcome = publish_score(&mut score, || {
        std::panic::panic_any("controlled demonstration");
    });
    assert_eq!(outcome, Err(WorkerPanicked));
    assert_eq!(score, old);
}
```

The panic hook normally runs when the panic starts, before the catch
receives the payload. A test can therefore observe a panic message
even when execution subsequently returns through `catch_unwind`.
This example keeps the hook configured by the application.

## Inspect state crossing the boundary

`UnwindSafe` marks types whose use across a catch does not raise the
standard library's unwind-safety warning. A closure capturing an
external `&mut` may have partially changed the target before a panic.
Rust rejects it by default at this boundary. `AssertUnwindSafe`
allows the programmer to accept responsibility after reviewing the
boundary. It adds no rollback behavior. Similarly, interior effects
through captures can be possible even if the bound is satisfied.

The score is written only after a successful worker return because
the destination is outside the closure. That is the proof of the
example's limited publication guarantee. The test separately records
a worker-local destructor during unwinding and checks that an
unfamiliar payload reaches an outer boundary. For ordinary expected
failures such as missing data, use a typed result from the worker.

## Practice and assessment

{{#include ../../exercises/C044.md}}

## Summary and further reading

State the panic strategy and identify which effects survive an unwind.
Keep publication outside the region that can fail.
See [Rust 1.75.0 catch_unwind](https://doc.rust-lang.org/1.75.0/std/panic/fn.catch_unwind.html#notes)
and [Cargo's panic profile setting](https://doc.rust-lang.org/1.75.0/cargo/reference/profiles.html#panic).

[Context and sources](043-context-and-sources.md) · [Robust APIs](045-bounded-recovery.md) · [Solutions](../reliability-solutions.md#chapter-044)
