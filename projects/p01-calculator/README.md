# P01. Command-line calculator

This package contains the `art-calc` executable, its reusable parser
and evaluator, and the tests for their public contracts. The
[project guide](../../src/projects/p01-calculator.md) explains the
grammar, checked arithmetic, diagnostic precedence, and deployment.

From the workspace root, use `cargo run --locked -p art-calc -- 9 '+' 5`
to print `14`. Run `cargo test --locked -p art-calc` to verify the
arithmetic boundaries and CLI output.
