# 003. Boolean algebra and digital logic

An addition column must turn two input bits and a carry into two
output bits. A branch must decide whether a value is zero. Both
are functions of bits, so their possible inputs can be listed and
their behavior checked. The same technique helps explain why a
program branches incorrectly when two conditions are combined.

**Prerequisite.** [Chapter 2](002-bits.md), including place values
and the addition column rule.

After this chapter, you should be able to construct a truth table,
derive a gate network from a desired function, prove the local
behavior of a full adder, and explain the role of state and the
clock in a register.

## Logic values and electrical signals

**Boolean algebra** works with two values, false and true, often
represented by 0 and 1. A **Boolean function** maps Boolean inputs
to a Boolean output. A **truth table** lists every input assignment
and its output. With two inputs there are four assignments. With
`n` inputs there are `2^n`, because adding an input doubles
the number of assignments. A truth table is practical for a
local rule with three inputs. An exhaustive table becomes unwieldy
when a function has dozens of inputs, so larger designs combine
smaller functions and prove properties of the composition.

A digital **gate** implements a Boolean function in hardware.
In a common positive-logic convention, one range of voltages
represents logical 0 and another represents logical 1. The actual
thresholds, delay, and energy depend on the technology. In a circuit
the signal takes time to settle after a change. Boolean reasoning
describes the intended stable output. Timing analysis must also
show the receiving part samples a usable value.

| a | b | NOT a | a AND b | a OR b | a XOR b | a NAND b |
| ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| 0 | 0 | 1 | 0 | 0 | 0 | 1 |
| 0 | 1 | 1 | 0 | 1 | 1 | 1 |
| 1 | 0 | 0 | 0 | 1 | 1 | 1 |
| 1 | 1 | 0 | 1 | 1 | 0 | 0 |

NOT flips a bit. AND requires both inputs. OR requires at least
one. XOR, or exclusive OR, requires exactly one of two inputs.
NAND applies NOT to the AND result. To detect that either of two
alarms requests attention, OR keeps the signal high when both
request attention. XOR would lower it on that input. To find
whether two one-bit values disagree, XOR supplies the right truth
table. Write the expected result for the both-high case when a
prose requirement uses “either.”

## Derive circuits from truth tables

Suppose a controller should emit 1 only when `a = 1` and
`b = 0`. A row with `b = 0` can be selected by `NOT b`.
The expression `a AND (NOT b)` therefore selects the desired
row. To select more rows, OR their row expressions. Each row
expression chooses the original input if that input must be 1,
or its complement if it must be 0. Only the expression belonging
to the current assignment can be true. OR of all selected
expressions therefore reproduces the specified truth table.

This construction also gives a way to compare circuits. XOR has
two high rows, so
`(a AND (NOT b)) OR ((NOT a) AND b)` implements it.
For a fixed assignment, either version yields the same stable bit.
An engineer can choose a gate implementation based on the cell
library, timing, area, or readability of the specification.
Equivalence of truth tables alone leaves those questions open.

One useful identity is `NOT (a OR b) = (NOT a) AND (NOT b)`.
The left side is true when `a OR b` is false, which happens
only when both inputs are false. The right side states that same
condition directly. The dual version exchanges AND and OR.
These are De Morgan's laws. In software, they help explain why a
guard that rejects a request on either of two failed conditions
accepts a request only when both conditions pass. When translating
such an identity to a program, consider observable evaluations.
Short-circuit evaluation can skip later work. A stable truth
table alone says nothing about when a function call happens or
whether it can fail.

NAND by itself can build NOT, AND, and OR. Duplicate a bit at the
two inputs of NAND to invert it. Invert a NAND output to recover
AND. Apply NAND to `NOT a` and `NOT b` to obtain OR by
De Morgan's law. This matters for learning and hardware design.
One primitive suffices to express these Boolean functions, but a
particular implementation may still choose cells with other named
functions and different timing. A hands-on gate derivation appears
in [Nand to Tetris Project 1](https://www.nand2tetris.org/project01).

## Add a column with gates

A **half adder** adds two bits. Its low result is `a XOR b`,
since exactly one input yields an odd total. Its carry is
`a AND b`, since both inputs must be 1 for a total of 2.
A **full adder** also receives a carry `c` from the column below.

```text
[a, b] ---> [XOR] ---> p ----+---> [XOR with c] ---> sum
                             +---> [AND with c] ---> h --+
[a, b] ---> [AND] ---> g -------------------------------+--> [OR] ---> carry out
```

*Figure 1. In the equivalent signal equations, `p = a XOR b`
and `g = a AND b`. The sum is `p XOR c`. The carry is
`g OR (p AND c)`. The equations specify each connection.*

Derive the carry formula in two steps. The first half adder
produces `p` and `g`, with `a + b = p + 2g` in ordinary
arithmetic. The second adds `p` to `c`, giving sum bit `s`
and carry `h = p AND c`. Thus `a + b + c = s + 2(g+h)`.
When `g` is 1, `a` and `b` are both 1, so `p` is 0
and `h` is 0. When `g` is 0, `h` can be either bit.
Consequently `g+h` is a single bit and equals `g OR h`.
The composed gates satisfy the arithmetic column contract.

| a | b | Carry in | Sum | Carry out |
| ---: | ---: | ---: | ---: | ---: |
| 0 | 0 | 0 | 0 | 0 |
| 0 | 0 | 1 | 1 | 0 |
| 0 | 1 | 0 | 1 | 0 |
| 0 | 1 | 1 | 0 | 1 |
| 1 | 0 | 0 | 1 | 0 |
| 1 | 0 | 1 | 0 | 1 |
| 1 | 1 | 0 | 0 | 1 |
| 1 | 1 | 1 | 1 | 1 |

Connect each carry out to the next column's carry in to obtain a
**ripple-carry adder**. The composition is correct by the invariant
in Chapter 2. To see the timing implication, add 1 to a word
of all ones. The carry starts in the lowest column. Every higher
column propagates it to the next position. In a design that waits
for each carry in order, that dependency chain can cross all
columns. A design that combines groups to compute carries sooner
has a different propagation structure and may require more
hardware. This is a reasoning argument about path length.
Measuring an actual delay requires a specific circuit and
technology.

## Select, store, and advance

A **multiplexer** selects an output from several inputs. For
two bits `old` and `new`, a selector `enable` can give the
output `((NOT enable) AND old) OR (enable AND new)`. At
`enable = 0` the first term supplies `old`, and the second
term is 0. At `enable = 1` the first term is 0 and the
second supplies `new`. Apply the same selector to each bit
in a byte to select one complete byte.

A gate network whose output depends on current inputs is
**combinational**. A **sequential** element also has stored
state. A clocked register can be modeled as storing the bit
present at its input at a designated clock edge, then exposing
that new bit at its output after a delay. This model assumes the
input is stable across the required timing interval. The
electrical implementation also imposes timing requirements, which
an abstract trace does not calculate.

Two logically equivalent expressions can pass all truth-table
tests and react differently while inputs change. Suppose the
multiplexer has `old = new = 1` and its enable switches from 0
to 1. The stable output is 1 on both sides of the transition.
In an implementation of the two-term expression, the first
term might fall before the second rises because their paths
have different delays. The output can briefly fall to 0 in
that interval. A receiver that samples during the interval
can see the transient. A clocked circuit must ensure correct
sampling under its stated timing bounds. The truth table
contains only the stable cases and supplies a different
kind of evidence.

```text
new value -----------> [ select new when enabled ] ---> [ clocked register ] ---> current value
current value -------> [ select current otherwise ]
enable --------------> [ selector ]
clock edge --------------------------------------------> [ update event ]
```

*Figure 2. A register with write enable feeds its existing value
back into a selector. The selector chooses the next value. The
clocked element retains the value between update events.*

| Event | Value before edge | Input | Enable | Value after edge |
| --- | ---: | ---: | ---: | ---: |
| First edge | 0 | 1 | 0 | 0 |
| Second edge | 0 | 1 | 1 | 1 |
| Third edge | 1 | 0 | 0 | 1 |
| Fourth edge | 1 | 0 | 1 | 0 |

Two registers can both read the previous state and accept new
values at the same clock edge. A simulation must compute all
next values before overwriting any current values when that is
what the modeled transition specifies. For example, to swap
two stored bits simultaneously, each register's next input is
the other's old output. Evaluating the first assignment,
overwriting its value, and then reading that value for the
second assignment produces two copies of one bit. The bug is
in the simulation's update order.

The monitor's branch is now expressible in terms of circuits.
OR all eight bits of `A` together, then invert the result.
The output is 1 precisely when each bit is 0. A selector can
use that condition to choose the jump target or the next
sequential address. The program counter retains the chosen
address at its update event. The chapter's purpose is to relate
the observed state transition to a possible hardware structure.
A real CPU can implement the same architectural branch rule
with a different timing and circuit organization.

## Practice and assessment

{{#include ../../exercises/C003.md}}

## Summary and further reading

Truth tables specify local Boolean functions. You can combine
gates to obtain an arithmetic bit, build a selector from an
enable condition, and combine selection with clocked storage
to preserve state across steps.

[Previous: Bits, binary, hexadecimal, and bounded arithmetic](002-bits.md) · [Solutions](../foundations-solutions.md#chapter-003) · [Next: CPU architecture, memory, and assembly](004-architecture.md)
