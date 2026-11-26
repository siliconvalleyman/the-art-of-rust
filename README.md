# The Art of Rust

## From Zero to Professional

**Zavqiddin Hasanov** · **First Edition, 2023** · English

*Master the Language. Engineer the Future.*

The Art of Rust starts with how computers represent information, then
builds toward Rust programming, mathematical correctness, and the
design of systems that handle real workloads and failures. The edition
label identifies the technical scope through December 31, 2023. The
curriculum uses Rust 2021 and the Rust 1.75.0 reference toolchain.

Start with the [preface](src/preface.md), [introduction](src/introduction.md),
or [table of contents](src/SUMMARY.md). The [curriculum](CURRICULUM.md)
lists prerequisites and planned evidence for all 25 parts. The
[progress record](PROGRESS.md) records the material available today.

## Build and verify locally

Install [rustup](https://rustup.rs/) and ensure your host has a linker.
The repository selects Rust 1.75.0, rustfmt, and Clippy through
`rust-toolchain.toml`. Install mdBook 0.4.36 from its
[release page](https://github.com/rust-lang/mdBook/releases/tag/v0.4.36)
or run `cargo install mdbook --version 0.4.36 --locked`. Keep both
tools on your shell's executable search path. The repository validator
requires Python 3.11 or newer.

```sh
mdbook serve
```

`mdbook serve` prints the address for the local reader. Run the same
checks as the validation workflow with `python scripts/validate.py`.
See [Contributing](CONTRIBUTING.md) for narrower checks and content
conventions.
