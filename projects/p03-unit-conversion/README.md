# P03. Unit conversion utility

The `art-convert` executable converts nonnegative lengths between
`mm`, `cm`, `m`, and `km`. The
[project guide](../../src/projects/p03-unit-conversion.md) derives
the integer representation and the rounding rule.

From the workspace root, `cargo run --locked -p art-convert -- 150 cm m 0`
prints `2`. Run `cargo test --locked -p art-convert` for arithmetic
and process-level tests.
