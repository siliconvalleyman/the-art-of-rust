# Editorial and verification guide

## Teach the mechanism

Open with a concrete problem, prerequisites, and measurable outcomes.
Introduce each necessary term before using it to explain a harder one.
Trace at least one representative input and a boundary case. Where
applicable, explain representation, compilation, execution, guarantees
and limits, failure, performance, and security. End with misconceptions
that can be diagnosed, independent practice, an assessment, and
prerequisite-aware navigation.

Distinguish language and library guarantees, implementation details,
platform behavior, mathematical results, observations, and engineering
judgment. Establish performance bounds through a stated cost model,
or report measurements with toolchain, target, hardware, workload,
sample count, variability, and build flags. Identify conceptual traces
and illustrative addresses close to their figures. Record toolchain,
target, optimization level and command for any actual compiler output.

## Prove the claim readers implement

Specify domains, preconditions, postconditions, constraints and failure
conditions. Define any notation, derive the approach, and make the
correctness argument. For loops, state the invariant, prove
initialization and preservation, give a terminating variant, and use
the exit condition to prove the postcondition. Explain where the code
expresses the invariant and where finite-width arithmetic matters.
Count work and storage under an explicit cost model. Make probability
assumptions before deriving an average; distinguish upper bounds from
tight bounds. Use tests for edge cases, representative cases, small
exhaustive domains, and agreement with a reference where practical.

## Publish and classify code

Use `rust` or `rust,edition2021` fences for executable examples and
`rust,compile_fail,edition2021` for intentional compiler rejections.
Check the intended diagnostic in a targeted fixture when rejection
for some unrelated reason could satisfy the example. Put
`rust,ignore` examples behind an explicit label of **Illustrative
fragment**, **Platform-specific**, or **Toolchain-specific**. Use
`text` for **Pseudocode** and distinguish input, expected output, and
captured output in accompanying prose. Use `{{#include}}` to share
tested source in published code blocks. Every unsafe operation needs
a precise local `SAFETY` argument.

Use an informative caption for each diagram and include the important
relationships as text or in a table. Keep local Markdown references
relative. Use root-relative repository paths only as prose code spans.

## Assess and review

Give chapter exercises stable IDs of the form `EX-CNNN-NN`. Give
interview questions separate `IQ-CNNN-NN` IDs. State the objective,
difficulty, evaluation criteria, reasoning, and common wrong answer.
Require concrete inputs and outputs in programming exercises. Put a
complete matching solution under the same ID in `solutions/`.

Review each chapter for learning progression, mathematics, Rust and
compiler semantics, safety, performance, copy and layout, and quality
assurance. Record evidence and applicable gate results in
[PROGRESS.md](PROGRESS.md). Use the status vocabulary below.

| Status | Meaning |
| --- | --- |
| NOT_STARTED | Scope and prerequisites identified |
| RESEARCHING | Primary sources and historical compatibility under review |
| DRAFTING | Chapter prose, diagrams, or assessments in progress |
| IMPLEMENTING | Examples, labs, or project code in progress |
| PROOF_REVIEW | Mathematical and operational arguments under review |
| TESTING | Automated or practical checks in progress |
| EDITORIAL_REVIEW | Terminology, citations, structure, and navigation under review |
| BLOCKED | A named external dependency prevents the next gate |
| VALIDATED | All applicable checks passed and evidence is recorded |
| COMPLETE | Validation and editorial review are complete |
