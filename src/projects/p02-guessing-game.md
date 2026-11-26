# P02. Number guessing game

Build a terminal game that tells the player whether a guess is too
low, too high, or correct. The input stream may end early or contain
malformed text. The game must finish after at most seven guesses
and let a tester replay its exact choice of number. This project
uses [control flow](../part-03/015-control-flow.md) and
[optional and fallible values](../part-06/028-option-and-result.md).

## Specify rounds, input, and outcomes

The secret is an integer from 1 through 100. Each round prints and
flushes one prompt, then reads one line. A final line without a newline
is also a guess. A line of at most 32 bytes, counting any line ending,
is eligible to parse. The parser requires UTF-8, trims surrounding
whitespace, and accepts a decimal `u8` from 1 through 100. Invalid
lines consume a round and get the same short prompt to try again.
The program ends immediately on a match. After seven unsuccessful
rounds it reveals the number.

| Outcome | CLI status | Observable result |
| --- | --- | --- |
| Correct guess | `0` | Prints the success message |
| Seven unsuccessful rounds | `1` | Reveals the secret |
| EOF before a guess, oversized line, or invalid options | `2` | Prints a diagnostic on stderr |
| I/O error | `1` | Reports the failure on stderr if available |

The default seed uses the system clock. `--seed <decimal>` supplies
an explicit `u64`. Both modes use the same integer transform and
bounded mapping. The resulting number is reproducible with its
seed and should be treated as predictable. The game does not use the
seed to protect any data.

## Derive the core before adding the terminal

1. Map the seed to a bounded integer. Updating a `u64` with shifts
   and XOR uses a fixed-width state. Remap zero to one, then apply
   the three updates. For every resulting state `s`, `s % 100` lies
   in `[0, 99]`, so adding one is safe before converting to `u8`.
2. Keep the byte limit at the read boundary. Wrap the buffered
   reader in `take(33)` before asking it to read a line. The extra
   byte distinguishes an exact-limit line from an oversized input.
   End the game if the limit is exceeded so a later round cannot
   interpret just the tail of that line.
3. Store a bounded round index and compare one accepted guess with
   the secret. The loop decrements its remaining budget after each
   unsuccessful iteration. On success, EOF, or a long line it
   returns immediately. The game ends once the loop budget runs out.
4. Give `play` a reader and writer. The library neither reads the
   clock nor opens terminal handles. The binary chooses the seed,
   opens its streams, and converts the result to a process status.

```rust,edition2021
{{#include ../../projects/p02-guessing-game/src/lib.rs}}

let mut output = Vec::new();
assert_eq!(secret_for_seed(1), 62);
assert_eq!(play(1, &b"1\n100\n62\n"[..], &mut output).unwrap(), Outcome::Won);
assert!(output.ends_with(b"Correct.\n"));
```

A guess of 1 gives `Too low.` and a guess of 100 gives `Too high.`
for seed 1. Every eligible line either gives one comparison result
or an invalid-input message. If all seven lines are invalid, the
game still consumes its finite budget and returns `Exhausted`.
With an empty input it emits the first prompt then returns
`EndOfInput`. Tests exercise the 32-byte and 33-byte boundaries and
check that a line after the last round is left unread by the core.

## Run and verify

With the pinned Rust toolchain installed, run these from the
repository root.

```sh
cargo run --locked -p art-guess -- --seed 1
cargo test --locked -p art-guess
cargo build --locked --release -p art-guess
```

Enter `62` to win the sample run. To use a clock-derived seed, omit
the arguments after `--`. The release executable is in
`target/release/`, with an `.exe` suffix on Windows. The integration
tests launch the binary with redirected streams and verify the exits.

## Review range, cost, and limitations

One complete round examines at most 33 bytes and retains only those
bytes in the reused line buffer. With `R` rounds and a fixed bound
`B` on accepted bytes per round, work is `O(R(B + 1))` and extra
input storage is `O(B + 1)`, excluding upstream buffering and time
spent waiting on I/O. An idle interactive stream can still block.
Because the game flushes prompts, it can be played through a pipe.

The mapping by remainder can introduce bias. Among `2^64` consecutive
integer states, a residue modulo 100 can occur once more than another
because 100 does not divide `2^64`. The clock does not supply a uniform
seed in the first place. For a competitive game, specify a verified
random source and range-sampling method. With reliable bounds on
feedback, a player can use binary search. At each step the largest
possible remainder is at most half the prior candidate count rounded
down, so seven guesses can resolve a range of 100.

See [Marsaglia's xorshift paper](https://www.jstatsoft.org/article/view/v008i14)
and the Rust 1.75.0 [buffered line API](https://doc.rust-lang.org/1.75.0/std/io/trait.BufRead.html#method.read_until)
and [byte-limit adapter](https://doc.rust-lang.org/1.75.0/std/io/trait.Read.html#method.take).
