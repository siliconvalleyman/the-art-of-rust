# 035. PhantomData, higher-ranked bounds, and GATs

A program can use a type parameter without storing an
ordinary value of that type. It can also ask for a callback
that accepts a fresh borrow at each call, and design an
interface whose output type depends on that particular
borrow. These mechanisms answer different questions about
which values a type describes and how long its uses may
last.

**Prerequisites.** [Variance and aliasing](../part-05/024-variance-and-aliasing.md)
and [monomorphization, trait objects, and dispatch](034-dispatch-and-trait-objects.md).
After this chapter, you should be able to explain a marker
field, read a universally quantified lifetime bound, and
prove an overlapping-window interface preserves exclusive
access.

## Carry a type marker without carrying a quantity of it

`PhantomData<Unit>` is a zero-sized marker. Including
it lets a type declaration refer to Unit even when
its stored number carries the entire numeric payload.
`Quantity<Grams>` and `Quantity<Milliliters>` can
then be different types with the same scalar stored
representation. The unit choice is part of the type.
There is no runtime integer field representing a unit code.

```rust,edition2021
{{#include ../../examples/abstractions/src/lending.rs}}

struct Grams;
let amount = Quantity::<Grams>::new(17);
assert_eq!(amount.amount(), 17);
assert_eq!(std::mem::size_of::<Quantity<Grams>>(), std::mem::size_of::<u32>());
```

The explicit transparent layout in this example gives
the wrapper the representation of its nonempty quantity
field under the documented conditions. The marker adds
no size and has alignment one. That is a separate fact
from the distinct type identity.

Marker shapes have semantic effects. `PhantomData<T>`
connects the enclosing type to T for analysis such as
variance and auto traits. An **auto trait** is one the
compiler may implement automatically based on the type
and its components, such as Send. Other marker shapes
can express different relationships. Choose a marker
that describes the role of the parameter. Equal sizes
alone cannot establish that two choices have the same
borrowing or thread-transfer behavior.
[Rust 1.75.0 documentation for PhantomData](https://doc.rust-lang.org/1.75.0/std/marker/struct.PhantomData.html).

## Quantify inside a callable bound

Read `for<'a> Fn(&'a str) -> usize` as a callable
that accepts a shared string reference for every
choice of lifetime `'a` relevant to the call.
`Fn` is the callable capability used here. The
typical order matters: the enclosing function receives
one callback and can choose the reference lifetime anew
for each invocation. A function taking an already
selected `&'a str` with an outer generic parameter
would have a different relationship.

```rust,edition2021
{{#include ../../examples/abstractions/src/lending.rs}}

assert_eq!(inspect_two(str::len), [4, 5]);
```

The two strings are created at different times.
The second owner ends before the enclosing function
returns. Each call lends a view and returns a plain
usize, so the array does not borrow either local
string. The higher-ranked bound gives the function a
callback it can apply to each independently borrowed
input. A callback that requires a static reference
could not accept those inputs.

This is **higher-ranked** because the lifetime is
quantified within the bound. Ordinary elided callback
signatures can express the same pattern in common cases.
Writing the quantifier is useful when checking precisely
who chooses a lifetime. The sample uses local String
values so their scopes are visible. Their allocations
are unrelated to the bound itself.
[Rust 1.75.0 Reference on higher-ranked trait bounds](https://doc.rust-lang.org/1.75.0/reference/trait-bounds.html#higher-ranked-trait-bounds).

## Give an associated type its own parameter

A **generic associated type**, or GAT, is an
associated type with its own generic parameters.
`Lending::Item<'a>` describes a family indexed by
a borrowed region. Its own `'a` is different for each
call to `next`. The bound `where Self: 'a` says that
the source type must be valid for the borrow used
to lend an item.

Consider a three-byte mutable slice and windows of
width two. The first window covers indices 0 and 1.
The second covers 1 and 2. Their intersection contains
index 1, so both cannot be exposed as simultaneously
usable mutable views. The return type of `next`
is tied to the current exclusive borrow of the source.
The next call cannot coexist with a previous live
window from that source.

```rust,edition2021
{{#include ../../examples/abstractions/src/lending.rs}}

let mut bytes = [1, 2, 3];
let mut windows = ByteWindows::new(&mut bytes, NonZeroUsize::new(2).unwrap());
windows.next().unwrap()[1] = 7;
assert_eq!(&*windows.next().unwrap(), &[7, 3]);
assert!(windows.next().is_none());
```

The first borrow ends before requesting the second.
The later view observes the earlier write to the
overlapping byte. This is how the API allows temporal
reuse without simultaneously lending the same position.
An ordinary iterator's associated Item type cannot vary
with a fresh borrow of its `next` receiver in this way.
Disjoint mutable elements can use a different iterator
design because their regions do not intersect.

**Compile-fail Rust.** The fixture borrows a second
window while the first remains in use. Rust 1.75.0
reports E0499.

```rust,compile_fail,edition2021
{{#include ../../examples/abstractions/src/lending.rs}}

let mut bytes = [1, 2, 3];
let mut windows = ByteWindows::new(&mut bytes, NonZeroUsize::new(2).unwrap());
let first = windows.next().unwrap();
let second = windows.next().unwrap();
first[1] = 7;
second[0] = 8;
```

## Prove coverage and progress

Let the slice length be `n` and the positive width
be `w`. After `k` successful calls, `start = k`.
The next window is `[k, k+w)` and exists exactly
when its end is within the slice. The phase starts
at zero. Each successful call returns that region and
increments the start. The returned view is created
using bounds-checked slice access. Positive width and
a successful bounds check imply `start < n`, so
incrementing fits and advances one position.

When `w > n`, the first valid window cannot exist.
Otherwise starts run from 0 through `n−w`
inclusively, producing `n−w+1` windows. Once no
window fits, the state remains at that start and
further calls still return absence. The checked
end arithmetic rejects unrepresentable indices.
Each call makes a fixed number of arithmetic and
slice operations. Work on the bytes by a caller
is separate and can depend on width.

## Practice and assessment

{{#include ../../exercises/C035.md}}

## Summary and further reading

Markers can preserve type information without a runtime
payload. Higher-ranked bounds let a caller of a callback
choose fresh lifetimes. GATs let a returned item type
depend on a particular borrow, so an API can lend
overlapping mutable views in succession.
[Rust 1.75.0 Reference on generic associated types](https://doc.rust-lang.org/1.75.0/reference/items/associated-items.html#associated-types).

[Dispatch and trait objects](034-dispatch-and-trait-objects.md) · [Solutions](../abstraction-solutions.md#chapter-035) · [Pure functions and composition](../part-08/036-pure-functions-and-composition.md)
