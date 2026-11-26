# P01. Command-line calculator

Build a command that takes one operation and produces one result.
The program can be used by another command, so its output, diagnostics,
and exit status need distinct roles. The calculation uses the signed
64-bit integer domain studied in [Rust's numeric types](../part-03/013-numeric-conversions.md),
and its errors follow the [Part IX recovery contract](../part-09/045-bounded-recovery.md).

## Specify the command before choosing functions

The grammar is `art-calc <left> <operator> <right>`. Both operands
use Rust's `i64` decimal parser on Unicode command arguments. An
optional sign is allowed. Inputs such as floating-point numbers,
leading or trailing whitespace, and operands outside the range
of `i64` are invalid. Operators are `+`, `-`, `*`, `/`, and `%`.
Pass the operator as a quoted string when using a shell that expands
wildcards. Division truncates toward zero. The remainder has the
sign of the dividend if nonzero. For example, dividing `-7` by `3`
gives quotient `-2`, remainder `-1`, and the identity
`-7 = 3 * (-2) + (-1)`.

| Outcome | Standard output | Standard error | Exit status |
| --- | --- | --- | --- |
| Success | One decimal integer and newline | Empty | `0` |
| Wrong number of arguments | Empty | Usage message | `2` |
| Invalid operand or operator | Empty | Fixed diagnostic | `2` |
| Zero divisor or arithmetic overflow | Empty | Fixed diagnostic | `1` |
| Result write or flush failure | May contain a partial write | Diagnostic if stderr is available | `1` |

Once the number of arguments is valid, parse left operand, operator,
and right operand in that order. Report the first error encountered.
A zero divisor for `/` or `%` is distinguished from overflow. The
`i64::MIN` and `-1` pair reports overflow for both operators under
Rust's checked arithmetic conventions. Even though the mathematical
remainder is zero, the signed quotient for this pair is outside the
type's range.

## Derive the architecture

1. Extract exactly three OS arguments at the process entry point.
   `args_os` lets the parser reject unrepresentable argument text
   through the same typed error path as other invalid text.
2. Convert the three arguments to `i64`, `Operator`, and `i64`.
   `Operator` gives the evaluator a finite set of choices. Use a typed
   error for each outcome the process boundary distinguishes.
3. Evaluate the operation with checked integer methods. These give
   the same response in debug and release builds. Only this layer
   needs to know arithmetic bounds.
4. Format either the result or a fixed diagnostic and return a status.
   A library caller can reuse the parser and evaluator without writing
   to the invoking terminal.

The core is shared with the executable and its tests. This example
exercises the same entry point used after argument extraction.

```rust,edition2021
{{#include ../../projects/p01-calculator/src/lib.rs}}

let [left, op, right] = ["-7", "%", "3"].map(OsStr::new);
assert_eq!(calculate(left, op, right), Ok(-1));
assert_eq!(Operator::Divide.evaluate(i64::MIN, -1), Err(CalcError::Overflow));
```

The `Result` variant determines which output path can run. No
arithmetic occurs after a failed parse. The evaluator returns success
only when the selected checked method yields a representable result.
Test the evaluator's exact endpoints and the binary's status and
streams independently. A test that checks only the numeric output
would miss a CLI that prints an extra diagnostic on success.

## Run and verify

With Rust 1.75.0 installed through rustup, run these from the
repository root. Cargo builds both the library and the executable.

```sh
cargo run --locked -p art-calc -- -7 '*' 9
cargo test --locked -p art-calc
cargo build --locked --release -p art-calc
```

The first command sends `-63` and a newline to stdout. The release
build places `art-calc` in `target/release/`, with an `.exe` suffix
on Windows. Run that binary directly when checking its exit status.

## Review limits and trade-offs

The command interprets only the selected arithmetic operation.
Its diagnostics omit argument contents, and malformed arguments have
a controlled error path. The program runs with the operating system's
argument limits. Parsing performs at most linear work in the argument
text examined. The checked operation has constant cost on the fixed
integer domain. The application adds constant-size evaluation state
beyond the arguments, excluding the operating system's argument storage
and the standard library's output buffering. No timing benchmark is
claimed.

The one-operation grammar leaves fractions, parentheses, and operator
precedence to future versions. A version accepting an expression needs
a grammar, source positions, and an explicit bound on token count and
nesting. Preserve stable statuses for callers that invoke the binary
from another program.

See the [Rust 1.75.0 checked remainder contract](https://doc.rust-lang.org/1.75.0/std/primitive.i64.html#method.checked_rem)
and [OS argument iteration](https://doc.rust-lang.org/1.75.0/std/env/fn.args_os.html).
