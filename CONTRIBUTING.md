# Contributing

Read the [curriculum](CURRICULUM.md), [progress record](PROGRESS.md),
and [editorial guide](STYLE_GUIDE.md) before changing a chapter.
Changes to examples, exercises, or projects should include the chapter ID
and the command used to verify the change.

`python scripts/validate.py` runs the repository structure checks,
Cargo workspace checks, the book build, and mdBook's Rust examples.
Use `python scripts/validate.py --docs-only` for the Python checks
alone. Use `python scripts/validate.py --manifest` to export the
curriculum as JSON. The complete checks require Rust 1.75.0 and
mdBook 0.4.36.

Before opening a pull request, review the diff, remove generated output,
record measurement or diagnostic metadata where applicable, and update
the progress record. Keep personal data and credentials out of examples,
fixtures, and command transcripts.
