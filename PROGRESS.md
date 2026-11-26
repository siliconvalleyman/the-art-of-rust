# Progress

## Current checkpoint

- Phase: Error Handling and Reliability begins at C041.
- Curriculum: 25 parts, 125 chapter specifications, and 25 project
  specifications. See [CURRICULUM.md](CURRICULUM.md).
- Chapters: C001–C040 are COMPLETE. C041–C125 are NOT_STARTED.
- Practice: 200 exercise/solution pairs, 80 interview
  question/answer pairs, a scored Part II assessment, and the
  Fundamentals midterm. The planned inventory has 425 exercises
  and 170 interview questions remaining.
- Correctness: reviewed the byte-column invariant, gate composition,
  machine instruction bound, cache state and miss arguments, parity
  invariant, and bounded nesting. Part II reviews covered quantified
  predicates, equivalent guards, relation composition, induction,
  balanced recurrences, modular arithmetic, GCD, expectation, and
  aggregate and potential-method bounds. Part III reviews covered
  finite-width arithmetic, UTF-8 prefix maximality, and the
  termination and correctness of iterative and recursive scans.
  Part IV reviews covered field offsets and byte order, minimality
  of first-fit search, cache residency, and covered-page bounds.
  Part V reviews covered ownership transfer, disjoint slice ranges,
  lifetime relationships, slot invariance, and RAII permit accounting.
  Part VI reviews covered product cardinality, exhaustive case
  analysis, scanner error precedence, and invariants for validated
  weights and typed workflow transitions.
  Part VII reviews covered parametric guarantees, trait-bound sums,
  implementation overlap, ordered dispatch, and disjointness in
  time for overlapping mutable windows.
  Part VIII reviews covered rounding and effect boundaries, closure
  capture, iterator exhaustion, callback order, early failure, and
  bounded demand compared against an eager reference.
- Projects: P01–P25 are NOT_STARTED.
- Licensing: selection of a repository license awaits the owner's
  authorization.
- Previous checkpoint committed and verified on remote `main`:
  `a07c1ae` (Type Systems and Abstraction).
  Locate the commit carrying this record with
  `git log -1 -- PROGRESS.md`. Compare it with
  `git ls-remote origin refs/heads/main` to verify publication.
- Next action: finish and validate C041–C045. Begin by distinguishing
  recoverable failure, absence, and violated invariants with a checked
  stock update, then verify conversion, source chains, panic behavior,
  and an all-or-nothing batch.

## Validation evidence

On November 26, 2026, validation used Rust 1.75.0 on
`x86_64-pc-windows-gnu`, mdBook 0.4.36, and Python 3.13. The
Chapter 039 accumulator example passed its individual doctest after
adding an explicit type to the fold seed. The full run passed the
other checks, including rustfmt, Cargo check, Clippy with warnings denied,
54 Rust integration tests, one Rust documentation test, the release
build, and the mdBook build and test commands. The byte-adder
differential check covered all 65,536 input pairs. The data-model
tests exercised all 769 routes and all 1,000 three-digit strings.
Part VIII checked each of 256 countdown starts and 7,680 combinations
of sample readings, minimums, and result budgets. Nineteen compiler
fixtures verified their intended diagnostics, including trait bounds,
coherence, object safety, overlapping mutable loans, and closure
capture moves. The first-program test checked stdout,
stderr, and exit status. Direct rustc and a standalone Cargo build
both produced `Hello, Rust!`.
The generated book passed the internal link and anchor check.
The Windows x64 assembly excerpt was captured using Rust 1.75.0,
LLVM 17.0.6, and optimization level 2. The separate record-layout
program ran at optimization level 0 and reported size 12, alignment
4, and offsets 0, 4, and 8. Archived Rust 1.75.0 documentation URLs
and referenced section anchors were checked before pinning citations.
The Part VII assembly capture uses the same compiler and target at
optimization level 2 and shows the specialized arithmetic and an
indirect trait-object call. It does not report runtime performance.
The Windows process sandbox prevented GCC from starting its bundled
linker. The complete test run used the approved unsandboxed command.

The foundations examples use the standard library only. Assembly,
cache, clock, scheduling, and address diagrams identify their model
or target assumptions. The instruction words are derived from
the ISA format. The chapter reviews covered technical sources,
terminology, traces, edge cases, resource limits, and navigation.

GitHub Actions reported that its job could not start because the
account is locked due to a billing issue. This blocks verification
on the hosted Linux runner. Local validation remains passing.
[Hosted workflow annotation](https://github.com/siliconvalleyman/the-art-of-rust/actions/runs/45766633020/job/139728983467).
