# Progress

## Current checkpoint

- Phase: Data Structures continues at C048. The first three engineering
  projects are implemented and published with verified behavior.
  Windows Application Control still blocks a full validation run.
- Curriculum: 25 parts, 125 chapter specifications, and 25 project
  specifications. See [CURRICULUM.md](CURRICULUM.md).
- Chapters: C001–C040 are COMPLETE. C041–C047 are TESTING pending
  the full mdBook test gate. C048–C125 are NOT_STARTED.
- Practice: 235 exercise/solution pairs, 94 interview question/answer
  pairs, a scored Part II assessment, the Fundamentals midterm, and
  the Core Rust midterm. The planned inventory has 390 exercises
  and 156 interview questions remaining.
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
  Part IX reviews covered recoverable failures, invariant preservation,
  explicit conversion, source-chain lifetimes, panic scope, and
  bounded transactional updates with deterministic error precedence.
  The first two Part X chapters reviewed geometric array growth,
  overflow-free ring positions, ownership of rejected queue items,
  chain re-linking, and logical traversal across two ring spans.
- Projects: P01–P03 are implemented, with 13 passing integration
  tests and three passing guide examples. Their full workspace gate
  remains TESTING. P04–P25 are NOT_STARTED.
- Licensing: selection of a repository license awaits the owner's
  authorization.
- Previous checkpoint committed and verified on remote `main`:
  `5cd95ee` (first three engineering projects).
  Locate the commit carrying this record with
  `git log -1 -- PROGRESS.md`. Compare it with
  `git ls-remote origin refs/heads/main` to verify publication.
- Next action: begin C048 with the `Eq`/`Hash` contract and a
  collision fixture, then derive Bloom filter false-positive
  probabilities and check an LRU eviction trace. Resume the full
  workspace and mdBook test gates on a host that can launch the
  pinned tools and generated test programs.

## Validation evidence

On November 26, 2026, validation used Rust 1.75.0 on
`x86_64-pc-windows-gnu`, mdBook 0.4.36, and Python 3.13. Before
registering P01–P03, Rustfmt,
Cargo check, Clippy with warnings denied, 62 Rust integration tests,
one Rust documentation test, the release build, and the mdBook build
passed. The full `mdbook test` command encountered linker launch
failures and later Windows Application Control blocks during targeted
retries. All 20 executable Rust blocks in Part IX and its solutions
passed in a single separately compiled Rust test harness, including
the Core Rust midterm answers. The Chapter 039 accumulator example
passed its individual doctest after specifying the fold seed type.
The byte-adder differential check covered all 65,536 input pairs. The data-model
tests exercised all 769 routes and all 1,000 three-digit strings.
Part VIII checked each of 256 countdown starts and 7,680 combinations
of sample readings, minimums, and result budgets. Part IX checked all
65,536 initial-stock and single-request combinations. Twenty-one compiler
fixtures verified their intended diagnostics, including trait bounds,
coherence, object safety, overlapping mutable loans, closure capture
moves, missing error conversion, and unwind-unsafe captures.
The first-program test checked stdout,
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
The Windows process sandbox initially prevented GCC from starting its
bundled linker. The later run used the approved unsandboxed command.

With P01–P03 registered, Rustfmt, Cargo check, the release build,
the documentation inventory, the mdBook build, and rendered link
checks passed. All 13 new integration tests passed in the release
profile, including invocations of the three real executables. The
three guide examples also passed in a separately compiled Rust test
harness. Windows Application Control blocked Clippy and specific
generated programs in the full Cargo and mdBook runs (OS error 4551).
These launches remain unverified until a full run succeeds.

For C046–C047, Cargo check, rustfmt, the documentation inventory,
mdBook build, and rendered links passed. Six new Cargo integration
tests passed in release mode. They included 8,748 queue histories,
2,187 list histories, and a 25,000-value list drop test. All 11
executable Rust blocks in the two chapters and their solutions passed
in a separate harness. Targeted mdBook tests still encountered
Windows Application Control blocks when launching some generated
executables.

The foundations examples use the standard library only. Assembly,
cache, clock, scheduling, and address diagrams identify their model
or target assumptions. The instruction words are derived from
the ISA format. The chapter reviews covered technical sources,
terminology, traces, edge cases, resource limits, and navigation.

GitHub Actions reported that its job could not start because the
account is locked due to a billing issue. This blocks verification
on the hosted Linux runner.
[Hosted workflow annotation](https://github.com/siliconvalleyman/the-art-of-rust/actions/runs/45808140604/job/139868681170).
