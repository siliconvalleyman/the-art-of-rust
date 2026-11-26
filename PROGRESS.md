# Progress

## Current checkpoint

- Phase: Rust Fundamentals begins at C011.
- Curriculum: 25 parts, 125 chapter specifications, and 25 project
  specifications. See [CURRICULUM.md](CURRICULUM.md).
- Chapters: C001–C010 are COMPLETE. C011–C125 are NOT_STARTED.
- Practice: 50 exercise/solution pairs, 20 interview
  question/answer pairs, and a separate scored Part II
  assessment. The planned inventory has 575 exercises
  and 230 interview questions remaining.
- Correctness: reviewed the byte-column invariant, gate composition,
  machine instruction bound, cache state and miss arguments, parity
  invariant, and bounded nesting. Part II reviews covered quantified
  predicates, equivalent guards, relation composition, induction,
  balanced recurrences, modular arithmetic, GCD, expectation, and
  aggregate and potential-method bounds.
- Projects: P01–P25 are NOT_STARTED.
- Licensing: selection of a repository license awaits the owner's
  authorization.
- Previous checkpoint committed and verified on remote `main`:
  `540037b` (Part I). Locate the commit carrying this record with
  `git log -1 -- PROGRESS.md`. Compare it with
  `git ls-remote origin refs/heads/main` to verify publication.
- Next action: write C011, walk through every meaningful token of a
  first Rust program, and verify its stdout and a compiler diagnostic.

## Validation evidence

On November 26, 2026, the full repository validator passed with
Rust 1.75.0 on `x86_64-pc-windows-gnu`, mdBook 0.4.36, and Python
3.13. It ran rustfmt, Cargo check, Clippy with warnings denied,
26 Rust integration tests, one Rust documentation test, the release
build, and the mdBook build and test commands. The byte-adder
differential check covered all 65,536 input pairs. The literal
fixture verified the intended `overflowing_literals` diagnostic.
The generated book passed the internal link and anchor check.
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
