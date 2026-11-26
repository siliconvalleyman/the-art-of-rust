# 034. Monomorphization, trait objects, and dispatch

A reporting pipeline applies arithmetic adjustments. One
call always triples a reading. Another obtains its next
adjustment from a list that can hold several different
operations. Both calls can present the same interface.
They differ in when the implementation is selected.

**Prerequisite.** [Coherence and orphan rules](033-coherence-and-orphan-rules.md).
After this chapter, you should be able to choose between
static and dynamic dispatch, explain the representation
of a borrowed trait object, and justify a performance
claim under a stated build or workload.

## Static dispatch resolves a concrete implementation

For a call such as `via_generic(&Scale(3), value)`, the
generic parameter is Scale. The trait bound guarantees that
Scale provides the necessary method. The compiler can select
that implementation without inspecting a runtime type tag.
That call mechanism is **static dispatch**.

Rustc typically uses **monomorphization** to generate
concrete code for generic uses. It substitutes arguments such
as Scale or Offset into the generic implementation. This
permits optimizations such as inlining a known method or
propagating a constant. A compiler may also share or
discard generated code as appropriate. The existence,
size, and naming of emitted symbols are implementation
observations rather than requirements on the source type.

```rust,edition2021
{{#include ../../examples/abstractions/src/dispatch.rs}}

assert_eq!(via_generic(&Scale(3), 7), 21);
assert_eq!(via_generic(&Offset(1), u32::MAX), 0);
```

The operations use explicit wrapping arithmetic, so both
debug and release builds have the same arithmetic contract.
The generic declaration controls which operations are
available, while the call's concrete parameter identifies
the implementation. A generic function can still make
dynamic calls inside its body when it uses trait objects.

## A trait object preserves operations and erases the concrete type

An `&dyn Adjust` value is a borrowed view of an
implementing object. It combines a pointer to the concrete
data with metadata that enables the trait operations.
The usual Rustc model for a trait object uses a **vtable**,
which supplies information including callable method
implementations. The vtable layout is a compiler detail.
Do not treat its offsets as a stable binary interface.

```text
&dyn Adjust
  data pointer -------> concrete adjustment, such as Offset(1)
  metadata -----------> operations for that adjustment's Adjust implementation
```

*Figure 1. The conceptual components let one call site
act on more than one concrete adjustment.*

A call through this view uses **dynamic dispatch**.
The call site can choose an implementation using that
metadata at runtime. The reference can borrow data that
already exist. Creating `&dyn Adjust` does not itself
allocate a heap object. Different owning pointer types
have their own storage and allocation contracts.

```rust,edition2021
{{#include ../../examples/abstractions/src/dispatch.rs}}

let offset = Offset(1);
let scale = Scale(3);
let forward: [&dyn Adjust; 2] = [&offset, &scale];
let backward: [&dyn Adjust; 2] = [&scale, &offset];
assert_eq!(pipeline(&forward, 7), 24);
assert_eq!(pipeline(&backward, 7), 22);
assert_eq!(pipeline(&[], 7), 7);
```

The slice elements all have one view type even though the
referenced values have different concrete types. The loop
invariant says that after each iteration the accumulator is
the result of applying exactly the preceding adjustments in
order. Initially no adjustments apply. The next call adds
one application of the required trait operation. Advancing
through a finite slice ends at the full composition. The
code issues at most one dynamic operation per entry and
keeps one running accumulator beyond the input.

## Check which traits can be used as objects

The historical Rust 1.75 documentation calls this property
**object safety**. A trait used as `dyn Trait` needs a
representation that works when its concrete implementing
type is erased. For example, a dispatchable method returning
bare `Self` would demand returning a value of a type the
caller does not know. A method with its own generic type
parameter also cannot be represented as one ordinary slot
representing all possible monomorphizations.

**Compile-fail Rust.** The reference compiler reports
E0038 for the use of `dyn Duplicate` in this example.

```rust,compile_fail,edition2021
{{#include ../../tests/fixtures/non_object_safe.rs}}
```

An operation that requires the concrete type can be marked
with a `where Self: Sized` restriction if the rest of the
trait is suitable for object use. The restriction removes
that method from object dispatch. Returning an explicitly
type-erased result instead can be an alternative if the API
intends callers to use that result through the trait.

## Inspect the two call sites on one target

The following excerpts were captured from Rust 1.75.0
(`82e1608df`, LLVM 17.0.6) for `x86_64-pc-windows-gnu`.
The source imports the same adjustment code as this chapter.
The two named functions make it possible to identify the
selected instructions in the assembly file.

```powershell
rustc +1.75.0 --edition=2021 --crate-type=lib --target x86_64-pc-windows-gnu -C opt-level=2 --emit=asm -D warnings examples/abstractions/assembly/dispatch.rs -o abstraction_dispatch.s
```

```text
triple_static:
	leal	(%rcx,%rcx,2), %eax
	retq

adjust_dynamic:
	movq	24(%rdx), %rax
	movl	%r8d, %edx
	rex64 jmpq	*%rax
```

The first body computes the low 32 bits of three times
the input with a scaled addition. There is no method call
in the shown body. The second loads a method address
relative to the incoming metadata, prepares a register
argument, and jumps indirectly to the loaded address.
The observed offset is specific to this build. Neither
excerpt establishes a timing result or a stable layout.

## Compare costs without making an absolute promise

Static selection can give the optimizer more information
and generate separate code for multiple concrete uses.
Dynamic selection can keep one call path for heterogeneous
inputs. It may require an indirect call and can affect
inlining opportunities. A compiler can remove a dynamic
call if it proves which implementation is selected.
Generated code size depends on how many uses are emitted
and what the optimizer can deduplicate.

An abstraction that lets a compiler generate the same
operations as a direct implementation is sometimes called
**zero-cost** for that comparison. The comparison requires
stated observables, toolchain, and cost model. It does not
establish that every use of a trait has no overhead.
Test semantic equivalence separately from any timing or
size comparison.

## Practice and assessment

{{#include ../../exercises/C034.md}}

## Summary and further reading

Choose static dispatch when concrete behavior can be selected
at the call and generic specialization fits the interface.
Use a borrowed trait object when a uniform view of
heterogeneous values is useful. Evaluate performance on
the actual configuration that matters.
[Rust 1.75.0 Reference on trait objects](https://doc.rust-lang.org/1.75.0/reference/types/trait-object.html) · [Object safety](https://doc.rust-lang.org/1.75.0/reference/items/traits.html#object-safety).

[Coherence and orphan rules](033-coherence-and-orphan-rules.md) · [Solutions](../abstraction-solutions.md#chapter-034) · [PhantomData and lending](035-markers-and-lending.md)
