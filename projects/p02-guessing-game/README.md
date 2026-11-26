# P02. Number guessing game

The `art-guess` executable runs a seven-round game. Its core accepts
an injected seed, input reader, and output writer. The
[project guide](../../src/projects/p02-guessing-game.md) explains line
limits, reproducibility, error handling, and the test boundaries.

From the workspace root, use `cargo run --locked -p art-guess -- --seed 1`
and enter `62` to win. Run `cargo test --locked -p art-guess` for
the deterministic engine and process-level tests.
