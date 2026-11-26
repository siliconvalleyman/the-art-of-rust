# 026. Structs, products, methods, and associated functions

A warehouse prints rectangular labels. Its printer needs the
width and height separately because rotating a label can make
it fit the printer. It also needs the area for material
accounting. The two dimensions belong together, but they can
vary independently. A useful Rust model expresses that
independence and gives each operation the access it needs.

**Prerequisites.** [Sets, relations, and functions](../part-02/007-sets-and-graphs.md)
and [ownership, moves, Copy, and Clone](../part-05/021-ownership-and-moves.md).
[Compound types](../part-03/014-types-and-text.md) covers tuple
and array syntax. After this chapter, you should be able to model
a Cartesian product with a Rust struct, select an appropriate
receiver, and distinguish an API contract from a memory layout.

## Derive the product from independent choices

Let `W` be the set of permitted widths and `H` the set of
permitted heights. A dimension is a member of `W × H`.
A particular member contains one choice from each set.
If there are four supported widths and three supported heights,
there are twelve dimension pairs, since each width can accompany
every height. That is the product rule from
[Chapter 9](../part-02/009-counting-and-probability.md).

This counting argument uses independence. If the largest
width may be combined only with the smallest height, the twelve
pairs no longer all represent admissible objects. The product
is still a useful representation of candidate inputs. An API
can validate a candidate before treating it as a usable label.
This chapter admits all `u16` pairs, including zero. The zero
choices describe degenerate rectangles with area zero. A domain
that prohibits them needs an additional contract.

A named-field **struct** gives the product a distinct type
and a name for each component. Field names help distinguish
two values of the same type. Selecting `width_mm` communicates
more than selecting `.0` at each use. Once declared, the type
can be used as a field or a function parameter. Since named
structs have distinct identities, another type with the same
field names does not become interchangeable by construction.

## Choose named, tuple, and unit forms

```rust,edition2021
struct PrinterLimits {
    width_mm: u16,
    height_mm: u16,
}
struct PrinterId(u32);
struct LocalPickup;

let limits = PrinterLimits { width_mm: 40, height_mm: 70 };
let printer = PrinterId(17);
let _mode = LocalPickup;
assert_eq!((limits.width_mm, limits.height_mm), (40, 70));
assert_eq!(printer.0, 17);
```

Named-field construction binds each field by name. A **tuple
struct** uses positions and retains a distinct type name.
`PrinterId` wraps one number, but a function requesting a
`PrinterId` cannot accidentally accept an arbitrary `u32`
without constructing the wrapper. Chapter 29 develops the
invariants this technique can protect. A **unit struct**
has no fields. It can serve as a marker that carries an
identity through a type. It has one value. Two different
unit struct types still represent different choices at a call
boundary.

The product rule extends to these forms. A one-field tuple
struct has as many possible field values as its field type.
The empty product has one value, corresponding to selecting
the unique empty list of field values. An array of dimensions
is a different model: it repeats the same element type
and gives each element a numerical position.

## Give operations appropriate receivers

An `impl Dimensions` block associates definitions with the
type. A function with no receiver is an **associated
function** that you call using the type name. A function
with a `self` receiver is a **method**. Inside the block,
`Self` stands for the implementing type. Rust has no
special constructor keyword here. The name `new` is a
convention, and `Self { width_mm, height_mm }` is an
ordinary struct expression using field-init shorthand.

```rust,edition2021
{{#include ../../examples/data-modeling/src/dimensions.rs}}

let mut dimensions = Dimensions::new(30, 40);
assert_eq!(dimensions.area_mm2(), 1200);
dimensions.rotate();
assert_eq!(dimensions.into_pair(), (40, 30));
```

| Signature | Access promised by the receiver | Situation |
| --- | --- | --- |
| `fn new(width_mm, height_mm) -> Self` | No existing instance | Construct a value |
| `fn area_mm2(&self) -> u32` | Temporary shared access | Read the dimensions and compute their area |
| `fn rotate(&mut self)` | Temporary exclusive access | Exchange the fields of the same value |
| `fn into_pair(self) -> (u16, u16)` | Ownership of the instance | Consume the value and give back its components |

The area operation widens both dimensions before
multiplication. The maximum mathematical product is
`65,535 × 65,535 = 4,294,836,225`, which fits in a
`u32`. Widening after multiplication would leave the product
operation in `u16`, where many valid inputs exceed its
range. The verified bound applies to all pairs, including
zero and the two maximum values. Rotation exchanges the
components with `std::mem::swap`. Since multiplication
commutes, the area is unchanged. Rotating twice restores the
original ordered pair because it applies the exchange twice.

The call syntax can borrow a receiver to match a method
signature. Here `dimensions.area_mm2()` behaves as a shared
borrow for the call. The mutable call needs a binding usable
through `&mut`. The final call moves the instance.
`Dimensions` in this example is not Copy, even though its
fields are Copy: the type does not implement that trait.
If a caller needs two independent uses, it can borrow for an
observation or design a type with an explicit duplication
contract. Compare these behaviors in
[Chapter 21](../part-05/021-ownership-and-moves.md).

## Keep representation and layout separate

The struct contains two stored integers. The methods are
separate functions associated with the type. Each instance
does not store a copy of the method bodies. Reading two
fields and multiplying fixed-width integers requires a fixed
number of operations in this model. Actual calling
sequences and optimization depend on the target and
compiler. An API name such as `area_mm2` can stay stable even
if an implementation changes how it derives the value.

The default Rust layout does not promise declaration order
for ordinary struct fields. Padding and alignment depend on
representation and target. The logical product says which
values a record contains, not an on-disk byte order.
Serialize the dimension fields explicitly if a file or
protocol must remain portable.
[Chapter 18](../part-04/018-alignment-and-layout.md) derives
offsets and byte order for a separate wire contract.

Constructors are a convenient location for validation.
Required invariants must also hold on every other way to
obtain or modify the type. A private field constrains
construction outside the defining module. A public
setter must still preserve the promised property.
The example permits every pair, so no validation is
needed here. When the domain requires positive dimensions,
Chapter 29 shows how to make the type enforce that rule.

## Check the model against boundaries

Record the independent choices first. Use unequal width
and height when testing a rotation so that a missing
exchange cannot pass. Test zero and the maximum width for
arithmetic. Compare both area and field order: two different
dimension pairs can have the same area. A test concerned only
with the area would miss a failure to change the orientation.

## Practice and assessment

{{#include ../../exercises/C026.md}}

## Summary and further reading

Named, tuple, and unit structs represent products with
different access conventions. Receivers express shared access,
exclusive access, or ownership. Check arithmetic in the
types where computations occur, and keep a portable format
separate from an in-memory representation.
[Rust 1.75.0 Reference on structs](https://doc.rust-lang.org/1.75.0/reference/items/structs.html) · [Associated items](https://doc.rust-lang.org/1.75.0/reference/items/associated-items.html).

[RAII, Drop, and resource invariants](../part-05/025-raii-and-drop.md) · [Solutions](../data-modeling-solutions.md#chapter-026) · [Enums and matching](027-enums-and-matching.md)
