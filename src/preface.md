# Preface

**The Art of Rust · From Zero to Professional**

**Zavqiddin Hasanov · First Edition, 2023**

*Master the Language. Engineer the Future.*

Consider a program that reads a measurement, converts its units, and
saves the result. A useful implementation depends on decisions that
come before its first line of Rust. Which inputs are valid? What
does the number represent? How much memory can the program use?
What should happen if the value is out of range or writing fails?
Can someone else reproduce the failure? These decisions recur in
larger systems, where the consequences of guessing are harder to see.

The book starts with such questions at the level of machine state,
then revisits them as the reader acquires better tools: mathematical
proofs, Rust's type system, tests, measurements, and observations of
systems under load. Its practical exercises move from paper traces to
implementations whose documented limits can be checked.

The edition label sets a historical technical scope ending December
31, 2023. Examples in the main curriculum use Rust 2021 and a
reference compiler of Rust 1.75.0, released on December 28, 2023.
[Rust 1.75.0 release announcement](https://blog.rust-lang.org/2023/12/28/Rust-1.75.0/).

Continue with the [introduction](introduction.md).
