# 030. Typestate and state transitions

A warehouse clerk opens a draft dispatch, validates its parcel
count, and records a shipment. If a dispatch has not been
validated, asking it to record a shipment is a programming
mistake. If a submitted parcel count is zero, the input is
invalid and the clerk should be able to correct it. A good
model can reject the first mistake at compile time and handle
the second as an ordinary runtime error.

**Prerequisites.** [RAII and resource invariants](../part-05/025-raii-and-drop.md)
and [newtypes and invariants](029-newtypes-and-invariants.md).
After this chapter, you should be able to define state
invariants, implement transitions that consume their source
state, and explain where dynamic checks still belong.

## Start with the allowed transitions

Use three states: Draft, Ready, and Shipped. A draft
stores an identifier. A ready dispatch also has a parcel
count in `1..=500`. A shipped dispatch additionally
holds a nonzero tracking number. This example records
shipment information in memory. Knowing a tracking number
is nonzero does not prove a carrier has accepted it.

```text
       prepare(valid count)         record_shipment(nonzero number)
Draft ------------------------> Ready ------------------------------> Shipped
  |
  +-- prepare(invalid count) --> same draft and the validation error
```

*Figure 1. A successful transition consumes its source state.
A failed preparation returns the draft for correction.*

Write the invariants before choosing method names. Every
state preserves the identifier. Ready and Shipped keep a
validated count. Shipped possesses a nonzero tracking number.
No public operation can construct these latter states
directly or mutate their fields. A number uniquely identifying
the shipment across a deployment would require other checks
and durable storage.

## Let the type select the available operations

**Typestate** puts the relevant state into a value's type.
Here `Dispatch<State>` has a type parameter named `State`,
which is a place for a type to be chosen. `Dispatch<Draft>`,
`Dispatch<Ready>`, and `Dispatch<Shipped>` name three
different concrete types. A common impl block can define
the identifier getter. Specialized impl blocks contribute
methods only to the types they name.

```rust,edition2021
{{#include ../../examples/data-modeling/src/dispatch.rs}}

let draft = Dispatch::new(17);
let ready = draft.prepare(5).unwrap();
let tracking = NonZeroU64::new(99).unwrap();
let shipped = ready.record_shipment(tracking);
assert_eq!(shipped.receipt(), (17, 5, tracking));
```

The state field stores the data needed at each stage. The
empty Draft marker is a unit struct. Ready holds its count.
Shipped adds tracking. The parameter does not require a
separate runtime state tag: each concrete type already knows
its state. Default field layout is still determined under
the Rust representation rules. Chapter 31 develops generic
types and impl blocks in more detail.

The `self` receivers move the original dispatch into
the transition. The successful result carries the
identifier and justified state data. The method does not
leave an independently usable earlier state behind.
If validation fails, the result returns ownership of the
same draft, together with the domain error.

## Prove the transition preserves each invariant

1. Construction supplies only Draft. No parcel count or
   tracking number is needed there.
2. The inclusive pattern `1..=500` supplies every legal
   count and rejects everything outside it. Invalid cases
   return the unchanged draft. Successful cases construct
   Ready with the original identifier.
3. Only a Ready value has `record_shipment`. Its validated
   count is copied into Shipped with the same identifier.
   The tracking type already excludes zero. The caller
   must validate the number before invoking this method.

Since these are the only public construction paths, each
preserves the destination invariant. The absence of Copy
and Clone implementations also means the example does not
supply a way to duplicate its transition token. Tests
check boundaries and verify that retry does not alter a
draft. An ordinary Rust integration test can check
accepted state transitions. A compile-fail example checks
a specific disallowed method call.

**Compile-fail Rust.** Calling `record_shipment`
on a draft produces E0599 with the reference compiler.
The fixture tests the same API used by the book.

```rust,compile_fail,edition2021
{{#include ../../examples/data-modeling/src/dispatch.rs}}

let draft = Dispatch::new(17);
let tracking = NonZeroU64::new(99).unwrap();
let _ = draft.record_shipment(tracking);
```

The refusal follows from the concrete receiver type.
There is no runtime branch to test whether a draft may
invoke that method. Runtime data still matters: a count
entered by an operator can be outside the accepted range,
and a tracking field can be zero. The API expresses
these separate decisions at their respective boundaries.

## Choose representation based on how states are used

Typestate works well when the path being followed is known
at the point of each call. A queue that contains drafts,
ready entries, and shipped entries needs a common element
type. It can use an enum whose payloads are the specialized
types. Reading a dispatch back from storage also requires
reading a tag and validating its fields. Rust's type
checker cannot certify data received from a file.

An enum with methods that validate transitions at runtime
can be simpler when a workflow is interpreted from requests
or changes frequently. Typestate makes invalid calls on
known concrete states unrepresentable through this API.
Neither design by itself provides atomicity for external
side effects. If recording a shipment requires remote
I/O, failures, retries, and durable acknowledgment need
their own contract.

The example's transitions do a fixed number of field
operations and allocate no heap storage. Their primary
benefit is establishing which data and operations are
available at each call site.

## Practice and assessment

{{#include ../../exercises/C030.md}}

## Summary and further reading

Write the state graph and each destination's invariant
before implementing methods. Move the source value
to transition, and return it explicitly when retry is
permitted. Use a runtime boundary when the next state
depends on external input.
[Rust 1.75.0 Reference on implementations](https://doc.rust-lang.org/1.75.0/reference/items/implementations.html) · [NonZeroU64](https://doc.rust-lang.org/1.75.0/std/num/struct.NonZeroU64.html).

[Newtypes and invariants](029-newtypes-and-invariants.md) · [Solutions](../data-modeling-solutions.md#chapter-030)
