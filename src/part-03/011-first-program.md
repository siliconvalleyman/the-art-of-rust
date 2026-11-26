# 011. Rust history, rustup, rustc, Cargo, and the first program

You are about to write a small program that sends one
line of text to standard output. The text is simple
enough to make every step visible, from the source file
you save to the moment the operating system runs the
result. You will build it twice, once directly with the
compiler and once with Cargo. That gives you a concrete
way to tell which tool produced a result or reported a
failure.

**Prerequisite.** [Operating systems, translation, and models of computation](../part-01/005-operating-systems.md).
You should be able to distinguish a source file, an
executable, and a running process. This chapter assumes
no prior Rust experience.

After this chapter, you should be able to select Rust
1.75.0, create and run a Rust 2021 binary package, explain
every meaningful token of its first program, and tell a
compilation failure from a program that ran and returned
an unexpected result.

## Why Rust has a toolchain and an edition

Rust 1.0 was released in May 2015. Its release emphasized
systems programming with memory-safety and concurrency
checks alongside control over representation and costs.
Later chapters will make those ideas concrete. You will
learn to show what Rust checks statically, when programs
still check conditions at runtime, and what depends on
an API or target. [Rust 1.0 announcement](https://blog.rust-lang.org/2015/05/15/Rust-1.0/).

A **toolchain** is the set of tools and library files
used to build Rust code for one or more targets. This
book uses Rust 1.75.0, which the Rust release team
published on December 28, 2023. An **edition** opts a
package into a group of language conventions that Rust
supports together. The package in this chapter declares
edition 2021. The number 1.75.0 selects a compiler release
and the number 2021 selects an edition that compiler
understands. For example, the same compiler can build a
package written for an earlier supported edition.
[Rust 1.75.0 release announcement](https://blog.rust-lang.org/2023/12/28/Rust-1.75.0/).

| Tool | Immediate responsibility |
| --- | --- |
| `rustup` | Install and select compiler releases, components, and target support |
| `rustc` | Analyze source code and produce the requested compiler output |
| `cargo` | Read the package manifest and coordinate builds, tests, and execution |
| `rustfmt` | Format source according to Rust formatting rules and configuration |
| Clippy | Check additional lints that can reveal probable mistakes or unclear code |

The **host** is the system where a tool runs. The
**target** describes the system for which code is built.
We will run the result on the same computer, so the
host toolchain's default target is appropriate. In a
later chapter you can build for another target and run
the result there.

## Install and verify the tools

Use the official Rust installer for your platform. A
native build needs a linker and, depending on the
platform and toolchain, system development files. On
Windows with the MSVC toolchain, follow the official
instructions for the required C++ build tools. On
macOS, the native linker comes from Apple's command-line
developer tools. On Linux, use your distribution's
compiler and linker setup. Open a fresh terminal after
installation so the shell can find the tools.
[Official Rust installation instructions](https://www.rust-lang.org/tools/install).

A **terminal** displays text, and a **shell** reads
commands and launches programs from it. Your shell has
a current directory for resolving relative file names
and a search path for finding commands such as `cargo`.
Work in a directory where you can create a new folder.
The commands below work in PowerShell or a typical
Unix shell unless they are specifically labeled.

```sh
rustup toolchain install 1.75.0 --profile minimal --component rustfmt --component clippy
rustc +1.75.0 --version
cargo +1.75.0 --version
```

Through rustup, the `+1.75.0` argument selects that
toolchain for the command. An explicit selector is useful
when other projects need different versions. In the book
repository a `rust-toolchain.toml` file selects Rust
1.75.0 for ordinary commands launched there. If a version
differs from the one you expect, `rustup show` reports
the active toolchain and information about its selection.

The compiler version line observed for this chapter was
`rustc 1.75.0 (82e1608df 2023-12-21)`. The date inside
the parentheses is compiler build metadata. The release
announcement has its own date. To record the host
architecture as well, use `rustc +1.75.0 -vV` and read
the `host` line. Book validation used
`x86_64-pc-windows-gnu`.
[Rustup toolchain overrides](https://rust-lang.github.io/rustup/overrides.html).

A plain text editor is sufficient. An editor can add
syntax coloring, project navigation, and diagnostics.
Those conveniences work best when its Rust integration
uses the same project directory and toolchain as your
shell. When investigating a discrepancy, save the file
and reproduce it by running the recorded command.

## Save exactly this source

Create a plain text file named `main.rs`. The `.rs`
extension marks conventional Rust source files for tools
and developers. The source code is the text below. Use
the straight ASCII quotation marks shown in it. Some
word processors substitute curly ones, which have
different character values.

**Executable Rust.**

```rust,edition2021
{{#include ../../examples/first-program/src/main.rs}}
```

Here is what the compiler sees, from the outside of
the function to its statement.

| Source element | Role |
| --- | --- |
| `fn` | Rust keyword introducing a function definition |
| `main` | Function name used as the ordinary Rust binary's entry function |
| `()` after `main` | Empty parameter list for this function |
| `{` and `}` | Start and end of the block forming the function body |
| `println` | Name of the standard output macro called here |
| `!` after `println` | Marks the invocation as a macro call in this syntax |
| Parentheses after `println!` | Delimit the input tokens of this macro invocation |
| `"Hello, Rust!"` | A string literal supplying characters, including the comma, space, and exclamation mark |
| `;` after the invocation | Ends the invocation as a statement in the body |

A **function** is a named unit of behavior. This
definition gives the behavior of a function named
`main`. When the binary starts under the standard
Rust runtime, startup code eventually calls that
function. The empty list means this definition takes
no parameters directly. Program arguments can still
be obtained through a library API later. There is no
explicit return type in this definition, so its return
type is the unit type, written `()`. A block groups
one or more steps and can also be empty. This block
runs one printing statement and then reaches its end.

A **macro** is an instruction to the compiler to expand
input syntax using the macro's rules. Macro expansion
occurs as part of compilation. The code produced for
`println!` arranges for the text and trailing newline
to be written during execution. The exclamation mark
following the macro name has a syntactic role. The other
exclamation mark lies inside the literal and is a
character that the program writes. The quotes delimit
the literal in source. The displayed output contains
the characters between them.

The semicolon in the source punctuates the statement. It
does not produce a character in output. Later Rust
programs will use a block's final expression to compute
a value, and a semicolon can affect that value. In
this chapter put semicolons after the printing
statements. Indentation and newlines make the source
easy to read. The meaningful delimiters here are
tokens such as parentheses, braces, and semicolons.

## Build with the compiler

From the directory containing your file, run the direct
compiler command. The explicit edition makes the build
independent of rustc's default for a standalone input.
The `-o` option chooses an output file name.

Use these commands on Linux or macOS.

```sh
rustc +1.75.0 --edition=2021 main.rs -o greeting
./greeting
```

Use these commands in Windows PowerShell.

```powershell
rustc +1.75.0 --edition=2021 main.rs -o greeting.exe
.\greeting.exe
```

The leading `./` or `.\` says the executable is in
the current directory. The compile command translates
the source and invokes the link step. The next command
asks the operating system to load and run the resulting
file. You can run the executable again without compiling
if the source and build settings have stayed the same.

**Expected standard output.** The program writes one
line ending after the last exclamation mark.

```text
Hello, Rust!
```

**Standard output** is the process's conventional stream
for normal results. The shell can connect it to your
terminal, capture it in a file, or send it to another
process. **Standard error** is a separate stream
typically used for diagnostics. The toolchain commands
can report build information separately from the text
your program produced. The integration test runs the
executable, captures both streams and its exit status,
and verifies all three.

```text
source file        compiler and linker        executable file
   main.rs   --->   parse/check and build  --->   greeting
                                                    |
                                             OS starts process
                                                    |
                                      main writes to standard output
```

*Figure 1. Building creates an executable file. Running
it starts a process whose execution makes the write.*

The literal has a known byte sequence at compile time.
During execution, the generated printing code can pass
that text to the output stream. There is no user input,
no loop, and the number of bytes requested by this
statement is fixed. A write can still fail or wait
because its destination is outside this computation.
The size of a compiled binary and its exact instructions
depend on target, compiler, and build configuration.
The first program needs none of those details to
specify its output.

## Build the same program with Cargo

Cargo packages describe source, dependencies, and build
settings in a **manifest**, a file named `Cargo.toml`.
Here you create a package with a single binary target.
The manifest's `[package]` table gives it a name,
a package version, and a Rust edition. The Rust compiler
version is selected by rustup. The package version
describes the package itself.

From a directory where a new `greeting` folder may
be created, run the following. Then replace the
generated `src/main.rs` with the source from this
chapter before running the last command.

```sh
cargo +1.75.0 new --bin --edition 2021 --vcs none greeting
cd greeting
cargo +1.75.0 run
```

The `--bin` argument requests a runnable binary
package. Cargo uses `src/main.rs` as this binary's
source entry by convention. The `--vcs none` argument
keeps the exercise focused on the package and skips
initializing a version-control repository. `cargo run`
builds the binary if needed and runs it. `cargo build`
performs the build alone. Cargo writes ordinary debug
build outputs under `target/debug` in a standalone
package, or the workspace's target directory when
you build inside a workspace. The book example can be
run from the repository root with
`cargo run --quiet --locked -p first-program`.

Cargo gives the same program a reproducible entry point
for adding tests, introducing dependencies, and naming
build targets. Calling rustc directly still helps when
reducing a compiler question to one file. Compare
failures at the step that produced them. If the build
fails, a previously built executable might still be
present. Check the build result before using any output
file as evidence for your latest edit.
[Cargo new command reference](https://doc.rust-lang.org/cargo/commands/cargo-new.html).

## Read a compile-time failure

Start by reading the first error from an actual failing
build. Identify the file, the line, and the syntax or
type that Rust expected. Then make one small edit and
rebuild. Later error messages can reflect the first
error, so fixing several things at once can hide which
change mattered.

**Compile-fail Rust.** Here the invocation has lost
the `!` following the macro name. Rust 1.75.0 gives
diagnostic E0423 because this call syntax expects a
function and `println` is a macro in the example.
The validator checks that diagnostic against the
reference compiler. Restore the symbol and verify that
the executable again emits the intended line.

```rust,compile_fail,edition2021
{{#include ../../tests/fixtures/macro_called_as_function.rs}}
```

An unexpected output such as `Hello, rust!` is a
different kind of failure. The compiler accepts the
string literal and the program can finish successfully.
The desired output specifies a capital R, so a runtime
comparison is needed to detect the discrepancy. Compile
checks, execution tests, and the human specification
each check a particular part of the result.

## Practice and assessment

{{#include ../../exercises/C011.md}}

## Summary

Rustup selects a toolchain. Rustc checks and translates
source. Cargo coordinates a package build. The executable
starts through the operating system and eventually runs
`main`. Reading which step failed narrows the question
you should investigate.

[Asymptotic reasoning and cost models](../part-02/010-asymptotic-work.md) · [Solutions](../rust-fundamentals-solutions.md#chapter-011)
