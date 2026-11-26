# 001. How computers execute instructions

A room monitor receives two counts, 23 and 18, and displays 41.
To explain that result, you need more than the word “add.” You need
rules for obtaining the two counts, storing the first while obtaining
the second, producing the sum, and deciding when to stop. A computer
can apply those rules repeatedly. The challenge for a programmer is to
state them precisely enough that each step has a well-defined result.

**Prerequisites.** Ordinary whole-number arithmetic and familiarity with
reading a table.

After this chapter, you should be able to distinguish a program from a
running computation, identify what information the computer must
retain, trace a short instruction sequence, and explain how a program
can be wrong even when every individual instruction was executed
correctly.

## A program gives rules for changing state

An **input** is information supplied to the computation. An **output**
is information it exposes to its surroundings. The **state** is the
information needed to determine what can happen next. State can include
a number in a register, an input position, a flag that records a
condition, or the identity of the next instruction. A **program** is a
description of permitted steps. An **execution** is a particular
sequence of steps from a particular starting state.

For a deterministic rule, the same complete state and the same next
input produce the same next state. An incomplete description can
conceal part of the state. Suppose both executions have a running
total of 23. If one has read one value and the other has read two, a
rule to “read the next value” can lead to different results. The input
position belongs in the model whenever later behavior depends on it.
For a real application, the time, an environment variable, or the
order in which messages arrive can likewise change observed behavior.

Every model leaves something out. The point of the table below is to
predict the bytes the program will output. The time an electrical
signal takes to cross a wire is irrelevant to that prediction under
the table's assumptions. It becomes relevant when the engineering
question is whether the monitor can refresh by a deadline.

## A small instruction machine

Use a teaching machine whose data values are **bytes**, units with 256
possible values. For this chapter, read each byte as a whole number
from 0 through 255. When addition reaches 256 it returns 0, after
which counting starts again. This is the first rule that differs from
unbounded arithmetic on paper. Chapter 2 derives its representation.

The machine has a register `A`, a small storage location that
instructions access directly, and four addressable memory cells.
A **memory address** selects a cell. The addresses of the four cells
are 0, 1, 2, and 3. Each cell stores one byte independently of `A`.
The **program counter**, `PC`, holds the index of the next row to
execute. Program rows and data cells are separate in this model, so
writing data to cell 2 cannot replace instruction row 2.

```text
                      program rows
              PC ---> 0 READ
                      1 STORE 0
                      2 READ         steps read or update A
                      3 ADD 0       <----------------------+
                      4 WRITE                              |
                      5 HALT                    +----------v---------+
                                               | A (one-byte value) |
input bytes -- READ -------------------------->|                    |-- WRITE --> output bytes
                                               +----------+---------+
                                                          |
                                               LOAD / STORE / ADD
                                                          |
                                                 data cells 0 to 3
```

*Figure 1. The program counter selects an instruction. The register
and data cells carry values through the computation. The arrows show
logical data flow in a teaching model.*

One entry in the instruction set is an **operation**, such as
`STORE`, and sometimes an **operand**, such as the address 0.
The same operation can act on different cells when its operand
changes. The instruction descriptions below define the complete
behavior needed by the exercises.

| Instruction | Effect |
| --- | --- |
| `READ` | Copy the next input byte into `A` and advance the input position. Stop with a missing-input error when no byte remains. |
| `LOAD i` | Copy cell `i` into `A`. |
| `STORE i` | Copy `A` into cell `i`. |
| `ADD i` | Replace `A` by the wrapped sum of `A` and cell `i`. |
| `JUMP_IF_ZERO j` | Set `PC` to row `j` when `A` is zero. Otherwise go to the next row. |
| `WRITE` | Append the current byte in `A` to the output stream. |
| `HALT` | Stop successfully with `PC` at the halt row. |

Each ordinary instruction advances `PC` by one. The jump substitutes
its target for that advance when its condition holds. A missing row or
invalid cell address causes an execution error. For the exercises, the
machine checks a memory operand when its instruction executes, and a
taken jump to a missing row fails on the next fetch. A failure stops
the computation and preserves output already produced. `READ` and
`WRITE` abstract devices as streams of bytes. The model assumes
that output can accept a byte for every `WRITE`.

This is an **instruction set architecture** in miniature, a contract
between the steps a program may request and the state changes a
machine must provide. A hardware design or a software simulator can
implement that same contract. The design of a physical CPU may involve
instruction caches, execution pipelines, and several instructions in
flight. The simple architecture trace below concerns which state
changes the program can observe.

## Trace the room monitor

Begin with all four cells and `A` equal to 0, `PC` equal to 0,
and the input stream `[23, 18]`. Brackets list the byte values in
their order of arrival. The column “next input” counts how many bytes
have already been consumed, starting from zero.

| Executed row | Effect | Next PC | A | Cell 0 | Next input | Output |
| --- | --- | ---: | ---: | ---: | ---: | --- |
| Initial | Ready to read | 0 | 0 | 0 | 0 | `[]` |
| 0 | Read 23 into the register | 1 | 23 | 0 | 1 | `[]` |
| 1 | Save the first value | 2 | 23 | 23 | 1 | `[]` |
| 2 | Read 18, replacing the register | 3 | 18 | 23 | 2 | `[]` |
| 3 | Compute 18 + 23 | 4 | 41 | 23 | 2 | `[]` |
| 4 | Copy the sum to output | 5 | 41 | 23 | 2 | `[41]` |
| 5 | Halt | 5 | 41 | 23 | 2 | `[41]` |

The register is just one byte wide. Row 1 saves the first count
because row 2 overwrites that register. If row 1 accidentally used
`STORE 1`, row 3 would read the original value of cell 0 and produce
18. Both instructions would have done exactly what the program asked.
The defect is the broken relationship between the storage and its
later use.

Each instruction trace consists conceptually of **fetch**, **decode**,
and **execute**. Fetch identifies the instruction selected by `PC`.
Decode determines what operation and operands it specifies. Execute
applies the rule. These stages describe a dependency in the teaching
machine. An implementation may overlap work or combine stages as
long as the stated architectural effects are preserved.

## From a trace to a correctness argument

A trace establishes what the rules predict for one starting state.
To cover all inputs under consideration, call the first input
`x` and the second `y`. Both are in 0 through 255. Once row 1
finishes, cell 0 contains `x`. Row 2 changes `A` and the input
position, so cell 0 still contains `x` when row 3 starts with
`A = y`. Row 3 stores the remainder of `x + y` on division by
256. Row 4 appends it exactly once, and row 5 terminates. There
are no jumps, so the counter reaches row 5 in six fetched
instructions. This proves the behavior for each stream with at
least two bytes. Any further bytes remain unread. With zero or one
bytes, a read fails before the first write.

The intended room-monitor contract might instead require the displayed
value to be the ordinary sum of the two counts. Under that contract,
the program is correct when the sum is at most 255. On inputs
`[250, 20]`, the wrapped result is 14 because 270 is 256 plus 14.
Execution matched its rules. The design chose a representation too
small for the intended domain. One repair is to enforce an input
limit that makes 255 the greatest possible total. Another is to
keep more than one byte for the result. The choice depends on
whether the application should reject larger totals or support them.

For the fixed program, successful execution has a fixed cost of six
instruction steps and two consumed bytes in this cost model. That
count describes the model. For a physical machine, actual time also
depends on the processor, memory, device readiness, and how the
program exchanges data with its environment. A long wait for the
second input byte can dominate the measured time while leaving the
instruction count unchanged.

## Branches change the path

Consider replacing the original sequence with one that writes a
blank-marker byte, 0, when it receives a zero, and doubles any
nonzero input. This has two paths. The second branch below is
unconditional under an established condition: `LOAD 1` reads the
initial zero from cell 1, a cell the program never writes.

**Pseudocode.** The named rows are instructions of the teaching
machine. The comments explain how control reaches a row.

```text
0 READ
1 JUMP_IF_ZERO 7
2 STORE 0
3 ADD 0
4 WRITE
5 LOAD 1
6 JUMP_IF_ZERO 8
7 WRITE
8 HALT
```

With input 0, row 1 jumps to row 7, which writes 0 and proceeds to
the halt. With input 9, row 1 advances to row 2. After row 3,
`A` holds 18. Row 4 writes 18, row 5 changes `A` to 0, and row
6 takes the jump to row 8. The output stays 18 even though the
register changes after the write. An input of 128 is a separate
test case: it follows the nonzero path, and its doubled result is
0 in this representation. Matching outputs can arise from distinct
execution paths.

The code relies on the initial value of cell 1. In a machine where
that cell began with a previous computation's value, row 6 might
fall through and produce a second output. A clear initial-state
contract is part of the design. An instruction set with unconditional
jumps would let us express the intent directly and remove the
extra load. Changing the available operations changes the program's
structure even though the required result is the same.

One shorter program can read the byte, store it, add it to
itself, write it, and halt. It obtains the same results because
doubling 0 already produces the blank marker. It fetches five
instructions for every available input. The branch version
fetches four for a zero input and eight for a nonzero input.
These counts help compare the implementations under an
instruction-count cost model. If both paths need to perform
different work later, preserving the two branches can be
valuable. A test of observable output can permit either program
while a trace test for branch behavior checks a different
property.

### Extend the model explicitly

Imagine a device that can reject a write when its output queue
is full. The current contract says each `WRITE` appends the
byte. To model that device, include queue capacity or a rule
for readiness in the execution environment. Choose what happens
to the register and counter on a rejected write. One policy
waits and retries the same instruction after space is available.
A second policy stops with a write error and lets the caller
decide what to do with the state. The policies differ in whether
one execution can wait indefinitely. They also differ in what a
test must observe when a write has already succeeded.

The same question arises for a budgeted runner. Counting
instructions limits work only when each step has bounded cost.
An abstract `WRITE` with the append rule has one effect per
step. A real write can wait on an external device. A system
with an elapsed-time requirement may need a timeout or a way to
cancel an operation even when the remaining instruction count
is positive. A resource limit should say what quantity it bounds.

## Find the first incorrect state

When an output is wrong, reproduce the input and write down the
intermediate states you expected. Compare them in execution order
with the states you actually observed. The first divergence usually
points to a narrower question than the final output does. If cell 0
was wrong before the add, examine where it was last written and how
its value was determined. If cell 0 and `A` were right, inspect the
arithmetic representation and the branch or instruction encoding.

The same approach works when a program crashes: distinguish the
earliest invalid state you observed from the location that eventually
reported it. For example, a jump to a missing row changes the counter
successfully under this machine's rule, and the failure is reported
when the next fetch cannot find a row. The source of the bad address
is the taken jump. Keeping a trace of effects before a failure also
helps determine what a real program must clean up or record.

In a real system, externally supplied values may influence an address,
a length, or a decision to keep working. A memory operation requires
an address inside its permitted domain, and repeated work needs a
resource budget. Paper machines make those checks visible. Later
chapters add stronger representations and runtime boundaries for them.

## Practice and assessment

{{#include ../../exercises/C001.md}}

## Summary and further reading

An execution is a state trace determined by initial conditions,
inputs, and transition rules. A program's contract tells you whether
that trace achieves the intended effect. A fixed-width value can make
a perfectly valid execution return the wrong answer for a
real-world requirement.

For a course-scale example of joining a CPU, memory, and an
instruction set, see [Nand to Tetris Project 5](https://www.nand2tetris.org/project05).

[Introduction](../introduction.md) · [Solutions](../foundations-solutions.md#chapter-001) · [Next: Bits, binary, hexadecimal, and bounded arithmetic](002-bits.md)
