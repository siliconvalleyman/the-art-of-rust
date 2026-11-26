# 012. Names, mutation, primitive types, and expressions

A workshop counter begins at 40, receives two additional
sign-ins, and then prints 42. To write this program, we
need to say which values can change, which names refer to
them, and which numerical type can hold the result. We
also need to distinguish an operation that performs an
effect from an expression that computes a value.

**Prerequisites.** [The first Rust program](011-first-program.md)
and [bits and bounded arithmetic](../part-01/002-bits.md).
After this chapter, you should be able to trace a binding,
explain reassignment and shadowing, select suitable primitive
types, and predict the value of a block.

## A binding gives a value a name

The keyword `let` introduces a **binding**, a name that
the program can use to refer to a value within its scope.
The **scope** is the region where that particular name is
visible. The expression to the right of `=` is its
**initializer**. Rust analyzes the initializer and checks
that it provides a suitable value before later code uses
the name.

**Executable Rust.**

```rust,edition2021
let mut total = 40_u16;
let added: u16 = 2;
total = total + added;
assert_eq!(total, 42);
```

`40_u16` is an integer literal with a type suffix.
`u16` denotes a 16-bit unsigned integer with values
0 through 65,535. The other line uses a type
**annotation**, `: u16`, to specify the type of
`added`. The right side of the assignment reads the
two values and computes 42. The assignment stores it
as the new value associated with the same binding.
`assert_eq!` compares two values and fails execution
if they differ. Here it verifies the end of the trace.

| After this statement | total | added |
| --- | ---: | ---: |
| `let mut total = 40_u16;` | 40 | outside scope |
| `let added: u16 = 2;` | 40 | 2 |
| `total = total + added;` | 42 | 2 |

*Figure 1. A reassignment reads the old value and replaces
it with the result. The other binding is unaffected.*

A binding introduced by ordinary `let` is immutable.
Add the keyword `mut` when the algorithm needs to
reassign that binding. This allows the assignment and
does not change the type inferred for the name. The
compiler still checks the new value's type.

**Compile-fail Rust.** The first binding below lacks
`mut`. Rust 1.75.0 rejects the second assignment
with diagnostic E0384.

```rust,compile_fail,edition2021
{{#include ../../tests/fixtures/reassign_immutable.rs}}
```

Choose the repair that matches the intended state changes.
If further steps must update an existing total, `mut`
expresses that requirement. If the first number is input
and the new number is a derived result, separate names
can preserve that relationship across later code. These
rules apply to the program's bindings. The compiler
may hold a local value in a register, place it in memory,
or compute the final result earlier when the behavior is
unchanged. Writing `mut` alone gives no portable
guarantee about where the value is stored.

## Shadowing introduces another binding

A second `let` using the same name creates a fresh binding.
The new binding **shadows** the earlier one when both would
otherwise be visible. Its initializer can refer to the
earlier binding. Its type can differ if the two stages
of the calculation need different representations.

```rust,edition2021
let reading = 10_u8;
let reading = u16::from(reading) + 250;
assert_eq!(reading, 260_u16);

let label = "reading";
{
    let label = "preview";
    assert_eq!(label, "preview");
}
assert_eq!(label, "reading");
```

The first shadowing initializer reads the byte 10 and
widens it to 16 bits, where adding 250 fits. The second
`reading` is a new `u16` binding. A reassignment to a
`u8` binding would still require the same type and
could not hold 260. In the label example the block
limits the inner name's scope. After the closing
brace, the outer name is visible again.

Use shadowing when the same conceptual item passes
through successive well-defined forms, such as a raw
quantity and its widened value. When you need to compare
both forms later, keeping distinct names makes each
reference explicit.

## Primitive types and inference

A **type** specifies the permitted values and operations
at a point in the program. A primitive type is built
into the language. The table gives common starting
points. More detailed representation and conversions
follow in the numeric chapter.

| Family | Examples | What to choose |
| --- | --- | --- |
| Unsigned integers | `u8`, `u16`, `u32`, `u64`, `u128` | Nonnegative quantities with a specified width |
| Signed integers | `i8`, `i16`, `i32`, `i64`, `i128` | Quantities that need negative values as well |
| Target-sized integers | `usize`, `isize` | Operations whose range depends on the target, including indexing with `usize` |
| Floating point | `f32`, `f64` | Binary floating-point approximations for measurements and calculations |
| Boolean | `bool` | Exactly two logical values, `false` and `true` |
| Character | `char` | One Unicode scalar value, explained further in the text chapter |
| Unit | `()` | One value, also spelled `()`, for results with no additional information |

For a fixed-width signed integer, the sign changes the
range. For example, `i8` stores −128 through 127,
whereas `u8` stores 0 through 255. Choosing `u8`
because the input is a nonnegative count still requires
knowing its maximum. Using `usize` for an on-disk field
would let the representation depend on which target you
compile for unless you explicitly define a portable format.
[Rust Reference on numeric types](https://doc.rust-lang.org/reference/types/numeric.html).

**Type inference** fills in types from constraints. A suffix
or an annotation supplies a constraint directly. A later
use can supply one as well. In the following example, the
value assigned to `width` must fit the later `u16` binding
`columns`, so inference gives the literal and the binding
that type. By contrast, when an integer literal has no
stronger constraint, Rust can default it to `i32`.
An unconstrained float literal defaults to `f64`.

```rust,edition2021
let width = 12;
let columns: u16 = width;
let enabled = true;
let symbol = 'Q';
let ratio = 0.5;
assert_eq!(columns, 12_u16);
assert!(enabled);
assert_eq!(symbol, 'Q');
assert_eq!(ratio, 0.5_f64);
```

Single quotes delimit the character literal. Double
quotes delimit the string literals used in Chapter 11.
A character is one Unicode scalar value. A string can
represent multiple scalar values, and the text visible
to a reader may combine several scalar values into what
looks like one character. Those distinctions matter
when counting text or determining how many bytes a
field uses.

## Constants and block results

A `const` introduces a named constant. Write its
type explicitly. Its initializer must be a constant
expression that Rust can evaluate at compile time.
Declare a known policy value at an appropriate scope
and give it an informative name.

```rust,edition2021
const MAX_SEATS: u16 = 120;
let registered = 116_u16;
let remaining = {
    let occupied = registered + 2;
    MAX_SEATS - occupied
};
assert_eq!(remaining, 2);
```

An **expression** can evaluate to a value. The block
initializer contains a statement ending after the
declaration and a final expression without a trailing
semicolon. Rust executes the statement before evaluating
the final expression, which supplies the block's value.
The binding `occupied` is visible from its declaration
to the closing brace. The caller can use the value
of the block through `remaining`.

Adding a semicolon after `MAX_SEATS - occupied`
makes the subtraction an expression statement whose
result is discarded. The block then evaluates to unit.
In this example `assert_eq!` needs a number and
the compiler can report the disagreement. Reading the
type of the block helps identify the earlier cause.
[Rust Reference on block expressions](https://doc.rust-lang.org/reference/expressions/block-expr.html).

The example assumes the occupied count is at most 120.
If counts come from an external source, first prove or
check that the arithmetic stays in range. A Rust type
states an upper bound on what a value can represent.
The application may require a smaller domain such as
the capacity of a particular room. The program must
account for both at its boundary.

## Read grouping before debugging a result

An **operator** combines one or more expressions. The
operators `+` and `*` combine two numbers, while
`==` compares two values and yields a Boolean. The
operator precedence rules group multiplication before
addition in `2 + 3 * 4`, giving 14. Parentheses can
make a different combination explicit. In
`(2 + 3) * 4`, the result is 20.

If your arithmetic expression returns an unexpected
value, trace which operands are combined and which
types they have. Also check whether the result of a
block was accidentally discarded. Types can identify
some kinds of mistakes at compile time. A test and an
explicit expected value still help when both meanings
have the same type.

## Practice and assessment

{{#include ../../exercises/C012.md}}

## Summary

Use a binding to name a value. Reassignment updates an
existing mutable binding, while shadowing introduces a
new binding. Types constrain numbers, logical values,
characters, and block results. A final expression
supplies a block's value when it is left as the tail of
the block.

[The first Rust program](011-first-program.md) · [Solutions](../rust-fundamentals-solutions.md#chapter-012) · [Numeric conversions, overflow, and floating point](013-numeric-conversions.md)
