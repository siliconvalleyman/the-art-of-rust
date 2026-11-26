# 016. Addresses, pointer values, and stack frames

Suppose a function receives two integers, adds them, and
returns a result. Where are the arguments during the call?
Where does execution resume? The answers come from
different layers. Rust specifies what the program can
observe. A calling convention specifies where a target
expects values at a machine-level call boundary. The
compiler decides which additional storage this particular
function needs.

**Prerequisites.** [CPU architecture, memory, and assembly](../part-01/004-architecture.md)
and [branches, loops, functions, and recursion](../part-03/015-control-flow.md).
After this chapter, you should be able to distinguish a
value from its address, trace a small call stack, and name
the platform assumptions behind a real assembly excerpt.

## An address identifies a location

Take a model of byte-addressed memory. The address
`0x1000` identifies a byte location, and the location
`0x1001` is one byte farther along. If a two-byte number
starts at `0x1000`, its representation occupies those
two adjacent locations. The value of the number depends
on its stored bytes, their meaning, and the selected byte
order. The address gives the position. Chapter 5 explains
why the same numerical virtual address can name a
different physical location in another process.

A **pointer** is a value that can refer to a location.
Reading or writing through a pointer is called
**dereferencing**. A Rust reference is a checked language
form whose use is constrained by validity, lifetime, and
aliasing rules. A **raw pointer** has type `*const T` or
`*mut T` for its pointee type `T`, with fewer guarantees
attached to the value itself. Safe Rust can obtain and
compare raw pointers. Dereferencing one requires a safety
argument and an unsafe context. Chapter 76 will cover the
full obligations.

An address alone does not show that an access is permitted.
The access must use an appropriate allocation, be aligned
as required, and meet the language's validity and permission
rules for that operation. Two pointer values can have the
same numerical address while the program's authority to
use them differs. As an initial mental model, keep the
address and the access conditions separate. Later chapters
will formalize which conditions are relevant in unsafe Rust.
[Rust standard library pointer documentation](https://doc.rust-lang.org/1.75.0/std/primitive.pointer.html).

## Derive element spacing

An array stores equal-sized elements consecutively. If
the first element starts at illustrative byte address
`0x1000` and each `u16` occupies two bytes, the three
element starts are `0x1000`, `0x1002`, and `0x1004`.

```text
address        0x1000  0x1001 | 0x1002  0x1003 | 0x1004  0x1005
array index         0        |        1       |        2
byte offset         0    1   |    2     3     |    4      5
```

*Figure 1. In this illustrative array of three u16 values,
the byte offset equals the index multiplied by two.*

In general, with element size `s), the byte offset for
index `i` is `i × s`. A one-past position has offset
`len × s` and can be useful for representing an end
boundary. It is outside the set of element positions.
Before doing address arithmetic on values from another
source, validate its logical index and ensure that any
finite-width calculations can represent their results.
When reading real array data in safe Rust, indexing or
`get` already maintains the language's array bounds.

**Executable Rust.** The example obtains raw pointers
from different safe operations and compares them. Nothing
in the example reads memory through a raw pointer.

```rust,edition2021
let measurements = [10_u16, 20, 30];
let start = measurements.as_ptr();
let second = std::ptr::addr_of!(measurements[1]);
assert_eq!(start.wrapping_add(1), second);
assert_eq!((second as usize) - (start as usize), 2);
assert_eq!(measurements.get(1), Some(&20));
```

`as_ptr()` gives the starting pointer, and the
`addr_of!` expression obtains a raw pointer to the
selected array element. For this pointer type,
`wrapping_add(1)` advances by the size of one `u16`.
It is used solely to form a pointer for comparison.
The next assertion checks the distance between two
observed addresses in the same small array. It does
not assume an exact address. The last line reads
through safe array access. Constructing an out-of-range
pointer would not establish permission to read from it.

## A stack of active calls records continuations

When one function calls another, the caller must be able
to continue when control returns. The point at which it
continues is a **return address** at the machine level.
The **stack pointer** is a register tracking an active
position in the machine's stack convention. A **stack
frame** is the stack storage associated with an active
call, which can include saved registers, local storage,
outgoing argument space, or other bookkeeping. Some of
the information associated with a call can be kept
outside that storage, including values held in registers.

As a conceptual example, `report` calls `compute`,
which calls `read_sample`. Once `read_sample` returns,
execution continues in `compute`, then eventually back
in `report`. The most recent active call finishes first
in this uninterrupted successful trace.

```text
after nested call       after read_sample     after compute
+----------------+     +----------------+    +----------------+
| read_sample    |     | compute        |    | report         |
+----------------+     +----------------+    +----------------+
| compute        |     | report         |
+----------------+     +----------------+
| report         |
+----------------+
```

*Figure 2. The drawing shows logical call nesting. The
top tracks which function can run next. Box size and
orientation make no claim about physical stack addresses.*

Different architectures can store a return address in
different places. An x86-64 `call` instruction records
the next address on the stack. Other instruction sets can
use a register first and spill it to memory when nesting
requires that. A **spill** saves a register value in memory
so the register can be reused. A compiler can also inline
a function, including its work inside the caller. A
separate source function then does not imply a separate
runtime call. To predict actual storage use, inspect the
compiled program for the target and build options at hand.

## Observe a real calling convention

An **application binary interface**, or ABI, defines
binary-level conventions such as argument and return
locations. For the Windows x64 C ABI, the first two
integer argument locations for this example are RCX and
RDX, and the result is returned in RAX. This rule
describes the external boundary. What the compiler does
with those registers within the function can vary.
[Microsoft x64 calling convention](https://learn.microsoft.com/en-us/cpp/build/x64-calling-convention).

`extern "C"` selects
the target's C ABI. The attribute keeps the emitted
symbol name easy to find for this standalone assembly
capture. The operation requests modular addition, so a
large result still has defined behavior.

**Toolchain-specific source.**

```rust,ignore
{{#include ../../examples/memory/call_abi.rs}}
```

The output excerpt below was generated with Rust 1.75.0
(`82e1608df`, LLVM 17.0.6) for
`x86_64-pc-windows-gnu`. The command was run from the
repository root with `-C opt-level=2`. No executable
was linked because the requested output was assembly.

```powershell
rustc +1.75.0 --edition=2021 --crate-type=lib --target x86_64-pc-windows-gnu -C opt-level=2 --emit=asm examples/memory/call_abi.rs -o call_abi.s
```

**Captured assembly excerpt.**

```text
add_words:
	leaq	(%rcx,%rdx), %rax
	retq
```

`leaq` computes the effective address expression,
here used as an integer addition. It puts the low
64 bits of the sum in RAX without reading data from
the computed address. `retq` resumes the caller using
the saved return address. This emitted body is a
**leaf** function: it makes no further calls. The shown
instructions do not adjust the stack pointer to reserve
local storage. The calling convention still applies,
including the caller's obligation to provide home space
for register arguments. A different target, optimization
level, or source body can change the emitted instructions.
This example illustrates those facts for one output, with
no cross-target performance claim.

For a call with inputs 23 and 18, the incoming registers
are RCX=23 and RDX=18, and after `leaq` the return
register is 41. For inputs `u64::MAX` and 1, the
mathematical sum is `2^64`, and the low 64 bits
are zero. The returned word matches the explicit
`wrapping_add` contract in both cases.

## Practice and assessment

{{#include ../../exercises/C016.md}}

## Summary

An address locates a position and a valid access requires
more information. Arrays provide a concrete element-spacing
model. A source-level call can have register and stack
effects determined by an ABI and by compiler choices.
Analyze a real instruction trace using the toolchain
and target that produced it.

[Branches, loops, functions, and recursion](../part-03/015-control-flow.md) · [Solutions](../memory-solutions.md#chapter-016) · [Stack, heap, static storage, and destruction](017-storage-and-destruction.md)
