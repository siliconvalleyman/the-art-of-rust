# 004. CPU architecture, memory, and assembly

Two programs can add the same values and make the same number of
additions. If one keeps retrieving nearby bytes while the other
jumps between distant regions, they can place different demands
on memory. To explain the difference, first separate what a
processor promises about instructions from how an implementation
gets those instructions and data to the right place.

**Prerequisites.** [Chapter 3](003-logic.md), including
[Chapter 2](002-bits.md) for integer representation.

After this chapter, you should be able to trace an RV32I instruction
sequence, explain how an instruction becomes bits, and derive
cache hits and misses for a specified address trace.

## The instruction set and the processor

The **central processing unit** (CPU) executes instructions and
manages the state required to carry them out. The **instruction
set architecture** (ISA) specifies visible state and the meaning
and encoding of instructions. A processor's **microarchitecture**
is its organization for implementing that contract. A pipeline
can begin decoding one instruction while an earlier operation is
still running. A core may also predict which address a branch
will choose. Those strategies must preserve the architectural
effects required for the completed instructions.

The register file holds locations that many instructions name
directly. An **arithmetic logic unit** (ALU) performs arithmetic
and Boolean operations. Control logic determines such details as
which register is read, whether a result is stored, and which
instruction address comes next. A memory system provides
instructions and data to the core. A CPU can have several
cores, each with its own instruction activity. The examples
here trace one instruction stream and omit simultaneous agents.

An address selects a byte in the memory model used here.
**Random access memory** (RAM) supports addressing locations
directly. The phrase describes an addressing capability. The
cost of an access on a particular system can still depend on
what was accessed recently, where the address maps, and what
other work is queued. Main memory in common computers is
volatile. A file on persistent storage can outlive shutdown,
though unsaved or unflushed changes can be lost. The next
chapter explains which of these aspects the operating system
mediates.

## Read a real instruction set

In the base 32-bit RISC-V integer ISA, **RV32I**, integer
registers `x0` through `x31` hold 32-bit values. Register
`x0` reads as zero, and writes to it have no effect.
Instruction bytes live at addresses, so the program counter
here is a byte address. The base instructions shown occupy
four bytes each. Unless a branch or another control effect
changes it, the next PC advances by four. The examples assume
a little-endian RV32I execution environment with the indicated
aligned addresses mapped and accessible. Chapter 2 explains
byte order. The RV32I specification defines the machine
rules used here. [RISC-V unprivileged ISA, 20191213, chapter 2](https://docs.riscv.org/reference/isa/v20191213/_attachments/riscv-unprivileged.pdf).

**Assembly** uses symbols for operations, registers, and other
operands. An **assembler** translates the symbols to machine
instructions and handles addresses or the information needed to
resolve them later. A **label** names a position in the program.
Here a branch label names an instruction at a relative offset
from the branch. Plain RV32I arithmetic instructions like
`add` use registers. A `lw` instruction loads a four-byte word
from memory, and a `sw` instruction stores one. The effective
address for these examples adds a small constant to the value
in a base register.

**Platform-specific illustration.** This handwritten RV32I
fragment has no operating system setup or call convention.
Assume `x4 = 0x1000` and aligned words at `0x1000` and
`0x1004` contain 23 and 18. The branch tests whether the sum
is zero. When it is zero, the fragment leaves the destination
word unchanged.

```text
address   instruction
0x200     lw   x5, 0(x4)
0x204     lw   x6, 4(x4)
0x208     add  x7, x5, x6
0x20c     beq  x7, x0, done
0x210     sw   x7, 8(x4)
0x214   done:
```

| After address | Next PC | Observable change |
| --- | --- | --- |
| `0x200` | `0x204` | `x5 ← 23` |
| `0x204` | `0x208` | `x6 ← 18` |
| `0x208` | `0x20c` | `x7 ← 41` |
| `0x20c` | `0x210` | 41 differs from 0, so the branch falls through |
| `0x210` | `0x214` | Four bytes beginning at `0x1008` hold `29 00 00 00` in hex |

If both original words are zero, `x7` becomes zero and
`beq` changes the PC directly to `0x214`. The word at
`0x1008` retains its old value in this single-stream model.
This is an important contract choice. A program that needs to
store a sum even when it is zero must execute the store on both
paths. The `add` instruction keeps the low 32 bits of the sum.
An input pair whose mathematical sum is `2^32` also takes the
zero branch. A program that needs that distinction must track
overflow or use a wider representation.

## Derive the encoding of one instruction

For an RV32I register-register `add`, the instruction word is
partitioned into six **fields**. A field is a range of bit
positions with a meaning fixed by the format. In the table,
`rs1` and `rs2` are source register numbers, `rd` is the
destination register number, and `funct3` and `funct7`
refine the operation designated by the opcode.

| Positions | Field | Bits for `add x7, x5, x6` |
| --- | --- | --- |
| 31–25 | funct7 | `0000000` |
| 24–20 | rs2 | `00110` |
| 19–15 | rs1 | `00101` |
| 14–12 | funct3 | `000` |
| 11–7 | rd | `00111` |
| 6–0 | opcode | `0110011` |

Concatenate fields in that order, then regroup the result from
the right in groups of four. The resulting instruction word is
`0x006283b3`. Equivalently, the nonzero fields contribute
`6 × 2^20 + 5 × 2^15 + 7 × 2^7 + 0x33`. The least
significant byte is `b3`, so the successive bytes in this
example's memory are `b3 83 62 00`. This is a derivation from
the ISA format and field values. No compiler output is needed
to establish the example.

Five bits can encode 32 different register numbers. A name like
`x32` is outside the format's register domain. Assemblers
diagnose invalid register operands before producing a working
instruction for them. By contrast, `add x7, x6, x5` is
encodable and has the same arithmetic effect here: addition is
commutative. A test of result values alone cannot tell which
source was encoded into which field. To verify a decoder,
inspect both source numbers separately.

## Why a cache can change the cost

A **cache** retains copies of contents from another level of
the memory system so that some accesses can be served closer to
the core. Cache storage typically moves blocks called **lines**.
When a needed line is already present, the access is a **hit**.
When the line must be obtained from another level, it is a
**miss** at the level being considered. A processor may have
separate instruction and data caches near each core, with
additional shared or private levels. Line sizes, associativity,
write policies, and eviction policies are properties of a
design. The ISA example above makes no assumptions about
them. [Intel system programming guide, memory cache control](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html).

In a hierarchy whose levels are called L1, L2, and L3, the
number indicates the order in which a request can find a
cached copy. A typical organization has a small, close L1,
additional cache capacity deeper in the hierarchy, and RAM
as backing memory. A request that misses at L1 can still hit
at L2. Saying “cache miss” without naming the level leaves
the distance to the data undetermined. Stores can also
interact with the hierarchy differently from reads. A
write-back cache can mark the updated line dirty and later
transfer its contents when replacing it. A write-through
cache updates a lower level as part of the write policy.
Writing to a missing line raises a separate question about
whether to bring the line into the given cache. The trace
model below deliberately describes only reads.

An abstract direct-mapped cache has two slots, each able to
hold one four-byte line. An address `a` belongs to line
`a div 4`, where `div` divides and discards the remainder.
The line belongs to slot `line mod 2`, where `mod` is the
remainder. Each slot remembers which line it holds, so finding
an occupied slot alone does not imply a hit. The cache begins
empty. Each miss installs the requested line, replacing the
previous line in that slot.

| Address | Line | Slot | Prior line in slot | Result | Line after |
| ---: | ---: | ---: | --- | --- | ---: |
| 0 | 0 | 0 | empty | miss | 0 |
| 1 | 0 | 0 | 0 | hit | 0 |
| 8 | 2 | 0 | 0 | miss | 2 |
| 0 | 0 | 0 | 2 | miss | 0 |
| 5 | 1 | 1 | empty | miss | 1 |
| 7 | 1 | 1 | 1 | hit | 1 |

*Figure 1. Each cache decision depends on the requested line
and the slot's previous line. All addresses and cache sizes are
illustrative.*

Access 1 follows nearby address 0, which is an example of
**spatial locality**. Repeatedly accessing one address in a
short interval is **temporal locality**. The second access to
address 0 misses because the intervening request for line 2
replaced line 0 in the same slot. Slot 1 was still empty.
Allowing more than one possible slot for a line can reduce such
conflicts, at a cost to the organization needed to look up and
replace lines. The right trade-off depends on access patterns
and implementation.

For a cold cache and a sequence of byte accesses at addresses
0 through `n−1`, the model misses once per distinct line.
Each line is read in one uninterrupted group, and its first
address must miss because the line has never been requested.
The remaining addresses on that line hit before it is ever
replaced. There are `ceil(n / 4)` such lines, including zero
lines when `n = 0`. A sequence alternating addresses 0 and
8 misses on every access, despite repeatedly using just two
bytes. These are exact counts for the model. Real hardware may
prefetch, change layouts, or hide part of the latency by
overlapping independent accesses. To infer measured performance,
record the actual access pattern and machine.

## Diagnose the right layer

When an assembly trace yields an unexpected value, start with
the input registers, mapped memory, operand widths, and the
branch conditions. When the result is right but execution is
slow, compare address sequences and look for repeatedly fetched
lines. These are different investigations. A cache ordinarily
changes how data is supplied, while an invalid address or wrong
word width changes whether the architectural operation can
produce the intended value.

An instruction can refer to an address and still be unable to
access it. Real execution environments define which addresses
are available, which operations are permitted, and which faults
must be handled. Chapter 5 applies that boundary to separate
processes.

## Practice and assessment

{{#include ../../exercises/C004.md}}

## Summary and further reading

An ISA specifies visible effects. Assembly spells out
instructions for that ISA and can be converted to fields in a
machine word. A processor's hierarchy can affect access costs.
The cache model illustrates why both addresses and order matter
when analyzing those costs.

[Previous: Boolean algebra and digital logic](003-logic.md) · [Solutions](../foundations-solutions.md#chapter-004) · [Next: Operating systems, translation, and models of computation](005-operating-systems.md)
