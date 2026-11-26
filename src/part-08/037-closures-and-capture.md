# 037. Closures and capture

A callback sometimes needs data from the function that
creates it. It may read a lookup table, update a counter,
or hand an owned result to its caller. Rust models those
choices as captured environment and callable traits.
The capture and the way the callback uses it determine
the operations a caller may request.

**Prerequisites.** [Shared and mutable borrowing](../part-05/022-borrowing.md),
[traits and bounds](../part-07/032-traits-and-associated-items.md),
and [pure functions and composition](036-pure-functions-and-composition.md).
After this chapter, you should be able to trace captured
data, distinguish Fn, FnMut, and FnOnce, and choose a
bound based on how often a caller invokes a callback.

## A closure has code and an environment

A **closure** is a callable expression written with bars
around its parameters, such as `|value| value + 1`.
Its body can refer to variables from its surrounding
scope. Variables used that way can become part of the
closure's **captured environment**. A conceptual compiler
model stores that environment in an anonymous type.
Shared borrows, mutable borrows, and owned values require
different stored capabilities. The precise representation
is an implementation detail.

```rust,edition2021
{{#include ../../examples/functional/src/callbacks.rs}}

let lookup = [10, 20];
assert_eq!(sample_two(|index| lookup[usize::from(index)]), [10, 20]);
assert_eq!(lookup, [10, 20]);
```

The callback borrows the lookup data and reads the item
selected by its argument. The bound promises valid
invocations, and the helper's implementation supplies
indices zero and one. A general caller may pass other
values unless its contract says otherwise. Rust does
not prove the lookup index stays in range merely from
the closure's signature.

## Distinguish invocation permissions

| Trait | Conceptual call receiver | When useful |
| --- | --- | --- |
| `Fn` | shared borrow | Repeat calls with shared access to the environment |
| `FnMut` | exclusive borrow | Repeat calls that may update captured state |
| `FnOnce` | ownership | Permit a call that may consume captured values |

A closure implementing Fn also satisfies FnMut and
FnOnce. A closure implementing FnMut also satisfies
FnOnce. Choose the least demanding bound that still
supports the required call behavior. Every closure can
be invoked once through FnOnce. A function using
ownership of that callable cannot assume it can replay the
same closure instance.

```rust,edition2021
{{#include ../../examples/functional/src/callbacks.rs}}

let mut total = 0_u32;
visit_all(&[3, 5], |value| total += u32::from(value));
assert_eq!(total, 8);

let prepared = String::from("ready");
let result = make_once(move || prepared);
assert_eq!(result, "ready");
```

The first closure mutably borrows the accumulator.
The helper keeps a mutable callable binding and invokes
it once per slice element, in order. With empty input
it performs no calls. The second closure yields the
owned String from its environment, consuming that
captured value. There is no second owned copy to
return from the same closure.

The `move` keyword chooses capture by value. It
does not by itself imply that calling the closure
consumes that environment. For example,
`move || label.len()` moves a String into the
closure while each call only borrows the captured
String to read its length. That closure may be
called repeatedly. The original binding is still
moved-from.

**Compile-fail Rust.** The reference compiler reports
E0382 when the original String is used after it has
become an owned capture.

```rust,compile_fail,edition2021
{{#include ../../tests/fixtures/moved_closure_capture.rs}}
```

## Check repeatability and effects independently

Fn describes which access to the closure itself is
needed to invoke it. It does not prove that every call
returns the same result or performs no effects.
For example, a closure can use a shared reference to a
Cell and change the cell through that interface.
A function taking FnMut can call it zero or many times
according to its algorithm. Tell callers if count or
order matters, and test that aspect separately.

The loop in `visit_all` invokes exactly one callback
per encountered slice element under normal completion.
It uses fixed traversal storage. The callback can do
arbitrary work permitted by its body. A performance
claim for the composed operation needs that additional
cost model.

## Practice and assessment

{{#include ../../exercises/C037.md}}

## Summary and further reading

Closures combine callable code with captured data.
Choose shared, mutable, or consuming call bounds
according to the required operations. Track ownership
at capture separately from what happens during a call.
[Rust 1.75.0 Reference on closure types](https://doc.rust-lang.org/1.75.0/reference/types/closure.html) · [FnOnce](https://doc.rust-lang.org/1.75.0/std/ops/trait.FnOnce.html).

[Pure functions and composition](036-pure-functions-and-composition.md) · [Solutions](../functional-solutions.md#chapter-037) · [Iterators and laziness](038-iterators-and-laziness.md)
