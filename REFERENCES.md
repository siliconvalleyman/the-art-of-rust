# References

## Rust and book tooling

- Rust Project. [Announcing Rust 1.0](https://blog.rust-lang.org/2015/05/15/Rust-1.0/), May 15, 2015.
- Rust Release Team. [Announcing Rust 1.75.0](https://blog.rust-lang.org/2023/12/28/Rust-1.75.0/), December 28, 2023.
- Rust Project. [Rust 1.75.0 documentation index](https://doc.rust-lang.org/1.75.0/). For historical claims, prefer the pinned library, language, and Cargo documentation shipped with this toolchain.
- Rust Project. [Rust source at the 1.75.0 tag](https://github.com/rust-lang/rust/tree/1.75.0). Source details describe that implementation.
- Rust Project. [Numeric types](https://doc.rust-lang.org/1.75.0/reference/types/numeric.html), [operator expressions](https://doc.rust-lang.org/1.75.0/reference/expressions/operator-expr.html), and [block expressions](https://doc.rust-lang.org/1.75.0/reference/expressions/block-expr.html). The book checks its examples against the pinned compiler.
- Rust Project. [Static items](https://doc.rust-lang.org/1.75.0/reference/items/static-items.html) and [destructors](https://doc.rust-lang.org/1.75.0/reference/destructors.html), for static storage and scope-based cleanup.
- Rust Project. [Type layout](https://doc.rust-lang.org/1.75.0/reference/type-layout.html), for field placement and alignment, and [Vec::try_reserve](https://doc.rust-lang.org/1.75.0/std/vec/struct.Vec.html#method.try_reserve), for fallible capacity reservations.
- Rust Project. [Moved and copied types](https://doc.rust-lang.org/1.75.0/reference/expressions.html#moved-and-copied-types), [Copy](https://doc.rust-lang.org/1.75.0/std/marker/trait.Copy.html), and [Clone](https://doc.rust-lang.org/1.75.0/std/clone/trait.Clone.html), for ownership transfers and duplication.
- Rust Project. [Pointer types](https://doc.rust-lang.org/1.75.0/reference/types/pointer.html) and [split_at_mut](https://doc.rust-lang.org/1.75.0/std/primitive.slice.html#method.split_at_mut), for shared references, mutable references, and disjoint borrowing.
- Rust Project. [Lifetime elision](https://doc.rust-lang.org/1.75.0/reference/lifetime-elision.html), [lifetime bounds](https://doc.rust-lang.org/1.75.0/reference/trait-bounds.html), and [subtyping and variance](https://doc.rust-lang.org/1.75.0/reference/subtyping.html), for historical signature rules.
- Rust Project. [Drop](https://doc.rust-lang.org/1.75.0/std/ops/trait.Drop.html) and [forget](https://doc.rust-lang.org/1.75.0/std/mem/fn.forget.html), for destructor calls and deliberate leakage.
- Rust Project. [Structs](https://doc.rust-lang.org/1.75.0/reference/items/structs.html), [associated items](https://doc.rust-lang.org/1.75.0/reference/items/associated-items.html), and [implementations](https://doc.rust-lang.org/1.75.0/reference/items/implementations.html), for type declarations, receivers, and specialized implementations.
- Rust Project. [Enumerations](https://doc.rust-lang.org/1.75.0/reference/items/enumerations.html), [match expressions](https://doc.rust-lang.org/1.75.0/reference/expressions/match-expr.html), [Option](https://doc.rust-lang.org/1.75.0/std/option/index.html), and [Result](https://doc.rust-lang.org/1.75.0/std/result/enum.Result.html), for domain alternatives and their representation.
- Rust Project. [Generic parameters](https://doc.rust-lang.org/1.75.0/reference/items/generics.html), [traits](https://doc.rust-lang.org/1.75.0/reference/items/traits.html), [trait implementation coherence](https://doc.rust-lang.org/1.75.0/reference/items/implementations.html#trait-implementation-coherence), and [trait objects](https://doc.rust-lang.org/1.75.0/reference/types/trait-object.html), for abstraction and dispatch.
- Rust Project. [PhantomData](https://doc.rust-lang.org/1.75.0/std/marker/struct.PhantomData.html), [higher-ranked trait bounds](https://doc.rust-lang.org/1.75.0/reference/trait-bounds.html#higher-ranked-trait-bounds), and [associated types](https://doc.rust-lang.org/1.75.0/reference/items/associated-items.html#associated-types), for markers and lending interfaces.
- Rust Project. [Closure types](https://doc.rust-lang.org/1.75.0/reference/types/closure.html), [FnOnce](https://doc.rust-lang.org/1.75.0/std/ops/trait.FnOnce.html), [Iterator](https://doc.rust-lang.org/1.75.0/std/iter/trait.Iterator.html), [FusedIterator](https://doc.rust-lang.org/1.75.0/std/iter/trait.FusedIterator.html), and [impl Trait](https://doc.rust-lang.org/1.75.0/reference/types/impl-trait.html), for capture, traversal, and opaque iterator types.
- Rust Project. [Error handling guidelines](https://doc.rust-lang.org/1.75.0/book/ch09-03-to-panic-or-not-to-panic.html#guidelines-for-error-handling), [question-mark operator](https://doc.rust-lang.org/1.75.0/reference/expressions/operator-expr.html#the-question-mark-operator), and [Error::source](https://doc.rust-lang.org/1.75.0/std/error/trait.Error.html#method.source), for recoverable outcomes, conversion, and source chains.
- Rust Project. [catch_unwind](https://doc.rust-lang.org/1.75.0/std/panic/fn.catch_unwind.html#notes) and [str::lines](https://doc.rust-lang.org/1.75.0/std/primitive.str.html#method.lines), for panic boundaries and line framing.
- Cargo Project. [Profiles](https://doc.rust-lang.org/1.75.0/cargo/reference/profiles.html), for build settings, overflow checks, and the [panic strategy](https://doc.rust-lang.org/1.75.0/cargo/reference/profiles.html#panic).
- mdBook Project. [mdBook 0.4.36](https://github.com/rust-lang/mdBook/releases/tag/v0.4.36).
- Rustup Project. [Toolchain overrides](https://rust-lang.github.io/rustup/overrides.html).

## Computer science foundations

- Noam Nisan and Shimon Schocken. [Nand to Tetris Project 1](https://www.nand2tetris.org/project01) and [Project 5](https://www.nand2tetris.org/project05). These primary course materials describe constructing gates and a complete educational CPU.
- RISC-V Foundation. [The RISC-V Instruction Set Manual, Volume I, version 20191213](https://docs.riscv.org/reference/isa/v20191213/_attachments/riscv-unprivileged.pdf), chapter 2, for the base RV32I architecture.
- Intel. [Intel 64 and IA-32 Architectures Software Developer’s Manuals](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html), system programming guide, for architecture-specific caching and address translation.
- The Open Group. [POSIX.1-2017 read](https://pubs.opengroup.org/onlinepubs/9699919799/functions/read.html), for descriptor-based reads on conforming platforms.
- Microsoft. [About Processes and Threads](https://learn.microsoft.com/en-us/windows/win32/procthread/about-processes-and-threads), for process resources and thread context on Windows.
- Microsoft. [x64 calling convention](https://learn.microsoft.com/en-us/cpp/build/x64-calling-convention), for argument registers, result registers, and stack requirements at the Windows x64 call boundary.
- Microsoft. [Virtual Address Space](https://learn.microsoft.com/en-us/windows/win32/memory/virtual-address-space), for Windows virtual address spaces.
- J. Postel. [RFC 791, Internet Protocol](https://www.rfc-editor.org/rfc/rfc791), Appendix B, for data transmission order and multioctet number fields.
- F. Yergeau. [RFC 3629, UTF-8](https://www.rfc-editor.org/rfc/rfc3629), for valid UTF-8 byte sequences.
- Unicode Consortium. [Unicode Standard Annex #29, revision 43](https://www.unicode.org/reports/tr29/tr29-43.html), for text segmentation and grapheme boundaries.
- A. M. Turing. [On Computable Numbers, with an Application to the Entscheidungsproblem](https://cs.stanford.edu/people/trevisan/cs172-07/turing36.pdf), proceedings volume 42, pp. 230–265, for a formal model of computation.

## Mathematical foundations

- Tom Leighton and Marten van Dijk. [Mathematics for Computer Science](https://ocw.mit.edu/courses/6-042j-mathematics-for-computer-science-fall-2010/), MIT 6.042J course materials, Fall 2010, for logic, proof techniques, graphs, counting, and probability.
- Euclid. [Elements, Book VII, Proposition 2](https://mathcs.clarku.edu/~djoyce/java/elements/bookVII/propVII2.html), for the greatest common measure by successive reduction.
