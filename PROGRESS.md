# Progress

## Current checkpoint

- Phase: Functional Programming begins at C036.
- Curriculum: 25 parts, 125 chapter specifications, and 25 project
  specifications. See [CURRICULUM.md](CURRICULUM.md).
- Chapters: C001–C035 are COMPLETE. C036–C125 are NOT_STARTED.
- Practice: 175 exercise/solution pairs, 70 interview
  question/answer pairs, a scored Part II assessment, and the
  Fundamentals midterm. The planned inventory has 450 exercises
  and 180 interview questions remaining.
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
- Projects: P01–P25 are NOT_STARTED.
- Licensing: selection of a repository license awaits the owner's
  authorization.
- Previous checkpoint committed and verified on remote `main`:
  `009f2ea` (Data Modeling).
  Locate the commit carrying this record with
  `git log -1 -- PROGRESS.md`. Compare it with
  `git ls-remote origin refs/heads/main` to verify publication.
- Next action: finish and validate C036–C040. Begin with a pure
  quote composed from bounded arithmetic, then verify capture,
  lazy invocation, early failures, and bounded iterator demand.

## Validation evidence

On November 26, 2026, the full repository validator passed with
Rust 1.75.0 on `x86_64-pc-windows-gnu`, mdBook 0.4.36, and Python
3.13. It ran rustfmt, Cargo check, Clippy with warnings denied,
46 Rust integration tests, one Rust documentation test, the release
build, and the mdBook build and test commands. The byte-adder
differential check covered all 65,536 input pairs. The data-model
tests exercised all 769 routes and all 1,000 three-digit strings.
Eighteen compiler fixtures verified their intended diagnostics,
including trait bounds, coherence, object safety, and overlapping
mutable loans. The first-program test checked stdout,
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
