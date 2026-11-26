# 005. Operating systems, translation, and models of computation

The monitor eventually needs to read an input device and send
its result somewhere. If another program is running at the same
time, each needs access to the machine under a set of rules.
Those rules explain why a correct computation can wait, why a
program can be denied access, and how two programs can use the
same numerical address without sharing a value.

**Prerequisite.** [Chapter 4](004-architecture.md), including
[Chapter 1](001-machine.md) for states and traces.

After this chapter, you should be able to distinguish a file,
process, and thread, trace a small virtual memory example,
explain the steps from source code to execution, and choose
an appropriate abstract model for a computation.

## The operating system manages resources

An **operating system** coordinates resources such as CPU time,
memory, files, and devices. Its **kernel** runs with privileges
needed to manage the machine. Ordinary applications normally run
with narrower permissions. A **system call** asks the kernel to
perform an operation under those permissions. For example, a
program can request bytes from an available file. The kernel can
reject the request, provide a portion of the requested bytes, or
arrange for the requesting work to wait until data is available.
Library calls can package several such operations or satisfy a
request using their own stored state.

The kernel usually works with a resource reference supplied
by the caller. POSIX systems expose many I/O resources through
integer file descriptors whose meaning is relative to the
process. Windows exposes many OS-managed resources through
handles. A closed reference cannot be used as an open one.
The OS may reuse its numeric value, so copying the number into
unrelated application state is insufficient to track resource
ownership. Later chapters return to the Rust types that tie
such a resource to its lifetime.
[The Open Group, read](https://pubs.opengroup.org/onlinepubs/9699919799/functions/read.html).

A file can contain source text, machine instructions, or data.
An **executable file** follows a format that supplies information
for starting a program on a target system. A **process** is an
instance of executing a program with its associated resources
and protection context. Starting the same executable twice can
create two separate instances with independent input and output.
A **thread** has an execution context that includes an instruction
position, registers to resume, and a stack for active calls.
Threads in the same process can share its address space and
resources. The distinction matters when choosing whether state
must be communicated or synchronized. [Microsoft, About Processes and Threads](https://learn.microsoft.com/en-us/windows/win32/procthread/about-processes-and-threads).

## Scheduling and waiting

The **scheduler** decides which eligible thread gets CPU time.
A thread with instructions ready to run can be called runnable.
A thread waiting for an event, such as a read to finish, can be
blocked. A running thread can stop using a core after it waits
or after the scheduler takes the core away, called preemption.
A **context switch** saves enough state of the previous thread
and restores enough of the next to let both continue from the
right points. A real scheduler uses platform-specific policies
and may manage many cores. The table uses one core with one
abstract time unit per row.

| Interval | Work on the core | Waiting and readiness | Result |
| --- | --- | --- | --- |
| 0–1 | A starts a read | B is runnable | A begins waiting for input |
| 1–2 | B computes a result | A is blocked | B remains runnable |
| 2–3 | B finishes | A becomes runnable at time 3 | Core becomes available |
| 3–4 | A resumes with received bytes | B has finished | A computes and emits a result |

*Figure 1. B uses the core while A waits. A's elapsed time
includes its waiting time as well as its work on the core.*

**Concurrency** means executions overlap in lifetime and can
interleave. **Parallelism** means some of their work runs at
the same instant. The diagram has concurrency on one core.
With more cores, independent work may also run in parallel.
Threads that share a value need an access protocol when a
step in one thread can change what another reads. Consider
each thread performing a conceptual read, add, and write on
a single counter. If both reads see 0 and both writes store
1, a required result of 2 would be lost. This is a trace of
an abstract split operation. Implementing a shared counter
in Rust requires an appropriate synchronized representation,
introduced later.

## Virtual addresses and protection

Code ordinarily uses **virtual addresses**. A virtual address
identifies a location in an address space. For two processes
with independent mappings, the same numeric virtual address
can identify two different physical locations. A **page** is
a block of virtual addresses treated together by a mapping.
A **frame** is a corresponding block in physical memory.
A **page table** records mapping information. Hardware uses
it under the operating system's management, including rules
that can allow or deny reading, writing, or executing a page.
Processors often cache recent translations in a **translation
lookaside buffer** (TLB). A TLB miss can require looking up a
mapping, while a page fault concerns access that needs handling
under the architecture and current state. They have distinct
causes. [Microsoft, Virtual Address Space](https://learn.microsoft.com/en-us/windows/win32/memory/virtual-address-space).

Here is a model with 16-byte pages for arithmetic convenience.
Divide virtual address `0x23` by 16. The quotient `0x2`
identifies the page and remainder `0x3` gives the offset.
For process A, suppose page 2 maps to frame 9 with read and
write permission. The physical address in this model is
`9 × 16 + 3 = 147`, or `0x93`. For process B, suppose
page 2 maps to frame 12 with read permission. The same virtual
address identifies physical byte `0xc3` for an allowed read
in B. A write request from B fails the protection check.

```text
process A virtual 0x23 ---> [page 2, offset 3] ---> [frame  9, read/write] ---> 0x93
process B virtual 0x23 ---> [page 2, offset 3] ---> [frame 12, read      ] ---> 0xc3
```

*Figure 2. The process selects a mapping and the offset stays
the same. Page sizes, addresses, and permissions are illustrative.*

A fault can arise because a mapping has yet to be established,
data needs to be brought into memory, or the access is
forbidden. Some faults can be resolved and the instruction
retried. For example, the system can arrange for a valid
file-backed page to be available. When a program tries to
write to a forbidden page, the process may receive an error
or terminate, according to the platform and circumstances.
Successful resolution can be expensive when it has to wait
for storage. Fault counters and access patterns help explain
performance, but a count alone does not identify which case
occurred.

Processes can intentionally share pages or communicate through
OS-mediated resources. Mapping isolation limits ordinary
unapproved memory access. It does not by itself enforce the
application's rules about which records a particular request
may see. Those rules need checks at the application boundary
as well.

## From source code to an executing program

**Source code** describes a computation in a programming
language. A **compiler** translates a program into another
representation, commonly through several intermediate forms.
A compiler can read the text, parse it into a structure,
check language rules, and generate machine instructions for
a target. An **object file** can contain generated instructions
and information about names and addresses still to resolve.
A **linker** combines pieces and resolves references under
a target's executable or library format. A linker may leave
some work for libraries to be loaded later.

To start an executable, the system's **loader** arranges its
initial execution environment, which can include mappings for
code and data and setup for shared libraries. A thread begins
at a platform-specific entry address. Runtime startup code can
prepare language-level arguments and call the programmer's
entry function. At the end, the process reports its **exit
status**, a value that the caller can use to tell whether
execution succeeded under a documented convention. Input and
output connections can be chosen by the caller, so writing to
standard output may send bytes to a terminal, a file, or a
connected process.

| Stage | Artifact or operation | Example failure |
| --- | --- | --- |
| Read and compile source | Parsed program and generated representation | Invalid syntax or a rejected type |
| Link components | Executable or library for a target | Missing definition needed by another component |
| Load executable | Mappings, resources, and initial thread state | Missing library or unsupported file format |
| Execute | State changes, output, and eventual status | An input with an invalid field or an I/O error |

For example, a program could pass the compiler's checks and
then fail to start when a required library is unavailable.
It could also load successfully and fail while reading from
a closed input. Diagnosing those errors begins with identifying
the stage. Chapter 11 will make these commands concrete.

An **interpreter** carries out the semantics of an input
program by reading a representation of that program and
performing its required steps. The bounded machine runner from
Chapter 1 can act as an interpreter for the listed teaching
instructions. The runner itself can be compiled to execute as
native code. Some systems translate source to **bytecode**,
instructions for a virtual machine, then interpret those
instructions. Others take the additional step of generating
machine code during execution, often called just-in-time
compilation. “Compiled” and “interpreted” describe steps an
implementation performs. One execution can involve both.

## Pick the model that answers the question

A **finite-state machine** has a finite set of states and
rules for transitions. A simple example recognizes whether
the number of 1 bits read so far is even. Its states are
even and odd. It starts even. On 0 it keeps its state, and
on 1 it toggles. It accepts the complete input if it ends
in even. The empty input is accepted because its count of
1 bits is zero. The stream `1011` gives even, odd, odd,
even, odd and is rejected. One state bit suffices to remember
everything this computation needs from the processed prefix.

An **invariant** is a property that holds at each chosen
point in an execution. Here, after any processed prefix, the
state describes the parity of its count of 1 bits. The empty
prefix establishes the invariant. Appending 0 leaves the
count, and appending 1 changes its parity. These are the
machine's two transitions, so both preserve the invariant.
At the end the state supplies the decision for the entire
input. Processing `n` bits costs `n` transitions and one
bit of state beyond the input.

A requirement to recognize properly nested parentheses of
arbitrary depth needs a way to distinguish different numbers
of unmatched openings. To see why fixed finite state alone
cannot handle that requirement for all lengths, imagine
feeding more opening symbols than there are states. Some
two distinct lengths reach the same state, including the
state before any opening. Append enough closing symbols to
match the shorter length. The shorter prefix then forms a
valid string and the longer one leaves open symbols. Starting
in the same state and receiving the same suffix forces a
deterministic finite-state machine to give the same decision.
At least one decision must be wrong. A bounded nesting depth
can use a bounded counter, with an error on a deeper input.

A **Turing machine** is an abstract model with finite control,
a read/write head, and an unbounded tape of symbols. Each step
uses the control state and current symbol to decide what to
write, where to move, and what state to enter. The unbounded
tape supports questions about what can be computed as inputs
grow, independent of a particular memory size. Physical
computers have bounded resources. The idealized model helps
explain general limits, while an engineering implementation
must specify what happens when its own limits are reached.
[Turing, On Computable Numbers](https://cs.stanford.edu/people/trevisan/cs172-07/turing36.pdf).

## Practice and assessment

{{#include ../../exercises/C005.md}}

## Summary and further reading

An executable provides the stored material for a process, and
threads give that process places to continue work. Scheduling
accounts for interleaving and waiting. A virtual address needs
a mapping and permissions. Compilation and interpretation
describe ways to implement program semantics. A formal machine
model lets you reason about the memory needed for a class of
inputs.

[Previous: CPU architecture, memory, and assembly](004-architecture.md) · [Solutions](../foundations-solutions.md#chapter-005) · [Introduction](../introduction.md)
