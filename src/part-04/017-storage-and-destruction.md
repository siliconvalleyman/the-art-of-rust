# 017. Stack, heap, static storage, and destruction

A function creates a buffer, returns it to its caller,
and closes a temporary resource along the way. Several
questions are easy to conflate: where the bytes are stored,
who is responsible for them, how long the value remains
available, and when associated cleanup happens. Keep those
questions separate when reading a Rust program or designing
a resource interface.

**Prerequisites.** [Addresses, pointer values, and stack frames](016-addresses-and-stack-frames.md)
and [branches, loops, functions, and recursion](../part-03/015-control-flow.md).
After this chapter, you should be able to sketch storage
and ownership across a call, distinguish named constants
from static objects, and predict cleanup in ordinary nested
scopes.

## Three ways storage enters an execution

The **call stack** is a storage convention associated with
active calls. A compiler may use a stack frame when a call
needs space for locals, saved registers, or further calls.
The space associated with an ordinary returning call can
then be reused. Knowing the scope of a source binding
reveals when the program can use it. It does not require a
physical stack slot. A value can stay in a register or be
eliminated when lowering and optimization preserve the
specified behavior.

The **heap** is storage managed through allocation and
deallocation requests. An **allocator** tracks available
storage and returns a region that satisfies the requested
size and alignment. An allocation can outlive the call
that requested it, provided something continues to own
it or otherwise keeps it available. The allocator may
reuse or cache freed storage. Deallocation usually
does not imply immediate release of pages to the OS.
Separate this resource policy from Rust's ownership
rules when analyzing a program.

**Static storage** describes storage available for the
program's lifetime. A shared immutable `static` item
defines one object accessible through its name. An
executable can also map data, including string literal
contents, that remain available for the program's
duration. Which executable section and addresses are
used depends on target and toolchain.

| Concept | What to ask |
| --- | --- |
| Storage | What holds the bytes while they are needed? |
| Scope | Which region can refer to this name? |
| Ownership | Which value or component is responsible for cleanup? |
| Availability | Until when may the program use this resource? |

A large buffer can be allocated through a small local
handle. The handle's scope and the buffer's size answer
different rows in that table.

## Move the handle and keep the allocation

`Box<T>` is an owning handle for one value. For the
ordinary nonzero-sized `u64` used below, creating a box
provides heap storage for the number. Returning the box
transfers ownership to the caller. The function's local
binding ends, while the returned box still owns the
allocation.
[Rust standard library documentation for Box](https://doc.rust-lang.org/1.75.0/std/boxed/struct.Box.html).

```rust,edition2021
fn make_reading() -> Box<u64> {
    let pending = Box::new(42);
    pending
}

let reading = make_reading();
assert_eq!(*reading, 42);
```

The explicit return type determines the numerical type
accepted by the initializer. The final expression transfers
the box. The `*` in the assertion reads the contained
number through the box's safe dereference operation. Since
this number can be copied, the box retains the allocation
after that read. Reusing `pending` outside its function
would fail because the name is outside scope. More general
move and borrow rules are the focus of Part V.

```text
inside make_reading      pending (owning handle) ---> [ u64 42 ]
after returning          reading (owning handle) ---> [ u64 42 ]
after owner is dropped   allocation can be reclaimed
```

*Figure 1. The value in the depicted allocation stays
available after the local name ends because ownership
transfers to the result. The diagram makes no claim that
the handle occupies a physical stack slot.*

An equivalent small program could return the bare
`u64`, which avoids requiring an owning heap allocation
for that result. A box is useful when an interface needs
owning indirection, for example for a structure whose
size would otherwise contain itself recursively. Paying
for allocation solely to make a small result outlive
the function is unnecessary when returning it by value
already supplies the required ownership. A large array
or buffer needs a separate analysis of allocation failure,
memory use, and what callers need to own.

## Give static objects and constants distinct roles

A `const` declaration names a value available for use
where its expression is valid. A constant does not
promise one unique address for every use of that value.
A `static` item names one static object. In safe Rust,
an immutable static can serve as shared program-wide
configuration with a fixed lifetime.

```rust,edition2021
const MAX_BATCH: usize = 64;
static LABEL: [u8; 3] = *b"job";
assert_eq!(MAX_BATCH / 2, 32);
assert_eq!(&LABEL, b"job");
assert!(std::ptr::eq(&LABEL, &LABEL));
```

The `b"job"` byte string has a borrowed array type,
and the dereference supplies the three-byte array used
to initialize the static. Reading the static twice
refers to the same object in this example. The constant
supplies a number. The example's constant assertion
requires no particular allocation for it. A platform
can place read-only data in a shared executable mapping.
That mapping is a target behavior outside the chapter's
language-level rule.
[Rust Reference on static items](https://doc.rust-lang.org/1.75.0/reference/items/static-items.html).

## Destruction follows ownership

When a local owner reaches the end of its ordinary drop
scope, Rust runs destruction for that value. A
**destructor** performs cleanup associated with the value,
such as returning owned storage to the allocator. A
user-defined type can implement `Drop` to arrange
additional cleanup. A **guard** is an owner whose
destructor releases a resource or restores a state that
the constructor arranged.

The example below uses a `Cell<u8>` as a tiny shared
observation record. A cell supports reading and
replacing its copyable byte through a shared reference.
Later chapters explain how that special capability fits
the aliasing rules. Here each destruction appends one
digit to the trace.

```rust,edition2021
use std::cell::Cell;

struct Marker<'a> {
    digit: u8,
    trace: &'a Cell<u8>,
}

impl Drop for Marker<'_> {
    fn drop(&mut self) {
        self.trace.set(self.trace.get() * 10 + self.digit);
    }
}

let trace = Cell::new(0);
let first = Marker { digit: 1, trace: &trace };
{
    let _second = Marker { digit: 2, trace: &trace };
    assert_eq!(trace.get(), 0);
}
assert_eq!(trace.get(), 2);
drop(first);
assert_eq!(trace.get(), 21);
```

The reference in each marker links it to the shared
record. The initial trace value is zero, so the first
observation before destruction is zero. Leaving the
inner block destroys the second marker and writes 2.
Passing `first` to the standard `drop` function
transfers it into a call that consumes it, producing
21. The fixed trace uses two digits and stays below
256. Extending the same encoding to arbitrary scope
depth would require wider storage or a different trace.

The order shown depends on ordinary scope exit and a
specific explicit drop. Local bindings governed by the
same ordinary block scope are dropped in reverse
declaration order. Detailed drop behavior has further
rules for temporaries, unwinding, and partial moves.
Abrupt process termination can skip cleanup. If
durability depends on an observed flush or close
result, expose an explicit fallible operation and
verify its status before letting a destructor perform
any remaining cleanup.
[Rust Reference on drop scopes](https://doc.rust-lang.org/1.75.0/reference/destructors.html).

## Practice and assessment

{{#include ../../exercises/C017.md}}

## Summary

The location of storage, the visibility of a name, and
the owner responsible for cleanup are separate facts.
Returning a box extends availability by transferring
ownership. A static item supplies one long-lived object.
Destruction is useful for scoped cleanup, and failures
that callers must observe need an explicit result.

[Addresses, pointer values, and stack frames](016-addresses-and-stack-frames.md) · [Solutions](../memory-solutions.md#chapter-017) · [Alignment, padding, endianness, and layout](018-alignment-and-layout.md)
