# Progress

## Current checkpoint

- Phase: Data Modeling begins at C026.
- Curriculum: 25 parts, 125 chapter specifications, and 25 project
  specifications. See [CURRICULUM.md](CURRICULUM.md).
- Chapters: C001–C025 are COMPLETE. C026–C125 are NOT_STARTED.
- Practice: 125 exercise/solution pairs, 50 interview
  question/answer pairs, a scored Part II assessment, and the
  Fundamentals midterm. The planned inventory has 500 exercises
  and 200 interview questions remaining.
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
- Projects: P01–P25 are NOT_STARTED.
- Licensing: selection of a repository license awaits the owner's
  authorization.
- Previous checkpoint committed and verified on remote `main`:
  `44d13e0` (Memory and Computer Architecture).
  Locate the commit carrying this record with
  `git log -1 -- PROGRESS.md`. Compare it with
  `git ls-remote origin refs/heads/main` to verify publication.
- Next action: start C026 on structs, product types, methods, and
  associated functions. Derive a record from two independent fields,
  compare named, tuple, and unit structs, and verify constructor and
  receiver examples on Rust 1.75.0 before adding exercises and solutions.

## Validation evidence

On November 26, 2026, the full repository validator passed with
Rust 1.75.0 on `x86_64-pc-windows-gnu`, mdBook 0.4.36, and Python
3.13. It ran rustfmt, Cargo check, Clippy with warnings denied,
27 Rust integration tests, one Rust documentation test, the release
build, and the mdBook build and test commands. The byte-adder
differential check covered all 65,536 input pairs. Ten compiler
fixtures verified their intended diagnostic codes, including move,
borrow, and lifetime errors. The first-program test checked stdout,
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
[Hosted workflow annotation](https://github.com/siliconvalleyman/the-art-of-rust/actions/runs/45756945894/job/139697681875).
