# Progress

## Current checkpoint

- Phase: Ownership and Borrowing begins at C021.
- Curriculum: 25 parts, 125 chapter specifications, and 25 project
  specifications. See [CURRICULUM.md](CURRICULUM.md).
- Chapters: C001–C020 are COMPLETE. C021–C125 are NOT_STARTED.
- Practice: 100 exercise/solution pairs, 40 interview
  question/answer pairs, a scored Part II assessment, and the
  Fundamentals midterm. The planned inventory has 525 exercises
  and 210 interview questions remaining.
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
- Projects: P01–P25 are NOT_STARTED.
- Licensing: selection of a repository license awaits the owner's
  authorization.
- Previous checkpoint committed and verified on remote `main`:
  `58b8a4e` (Rust Fundamentals and the midterm).
  Locate the commit carrying this record with
  `git log -1 -- PROGRESS.md`. Compare it with
  `git ls-remote origin refs/heads/main` to verify publication.
- Next action: write C021 on ownership, moves, Copy, Clone, and partial
  moves. Start with a single-owner transfer trace and an E0382
  compile-fail fixture, then compare a Copy value with an explicit
  heap-owning clone and verify cleanup behavior.

## Validation evidence

On November 26, 2026, the full repository validator passed with
Rust 1.75.0 on `x86_64-pc-windows-gnu`, mdBook 0.4.36, and Python
3.13. It ran rustfmt, Cargo check, Clippy with warnings denied,
27 Rust integration tests, one Rust documentation test, the release
build, and the mdBook build and test commands. The byte-adder
differential check covered all 65,536 input pairs. The literal
fixtures verified the intended `overflowing_literals`, `E0423`,
`E0384`, and `E0308` diagnostics. The first-program test checked stdout,
stderr, and exit status. Direct rustc and a standalone Cargo build
both produced `Hello, Rust!`.
The generated book passed the internal link and anchor check.
The Windows x64 assembly excerpt was captured using Rust 1.75.0,
LLVM 17.0.6, and optimization level 2. The separate record-layout
program ran at optimization level 0 and reported size 12, alignment
4, and offsets 0, 4, and 8. Archived Rust 1.75.0 documentation URLs
and referenced section anchors were checked before pinning citations.
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
[First hosted workflow run](https://github.com/siliconvalleyman/the-art-of-rust/actions/runs/45748943367).
