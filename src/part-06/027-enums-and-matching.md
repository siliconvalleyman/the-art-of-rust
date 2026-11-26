# 027. Enums, sums, destructuring, and exhaustive matches

A parcel can be collected at a counter, placed in a numbered
locker, or delivered to a zone. A locker needs a number.
Delivery needs a zone and a handling flag. The counter case
needs neither. This chapter derives a type in which the
chosen route determines which data exists, then uses pattern
matching to turn the route into an action.

**Prerequisite.** [Structs, products, methods, and associated functions](026-structs-and-products.md).
After this chapter, you should be able to count a finite
sum of products, read a pattern that takes fields apart,
and predict whether matching borrows or moves a payload.

## Encode alternatives with a tag

A product contains one value from each component. A **sum
type** contains a case tag and data for that particular case.
In mathematics this is a **disjoint union**. The tag keeps
cases separate even when they hold the same payload type.
Locker number 7 and delivery zone 7 remain distinct because
one case selects a locker and the other selects a delivery
zone.

Rust's `enum` declaration names those alternatives, called
**variants**. A unit variant has no fields, a tuple variant
has positional fields, and a struct variant has named fields.
All variants form one enum type. The route below admits all
byte values for locker numbers and delivery zones. This
represents identifiers, without claiming that every identifier
is installed in an actual warehouse.

| Case | Payload | Number of values |
| --- | --- | ---: |
| Collect | unit | 1 |
| Locker | a byte | 256 |
| Deliver | a byte and a Boolean | 512 |

The delivery case is a product of 256 zones and two handling
flags. The alternatives are disjoint, so there are 769 route
values. An alternative with independent optional fields for
locker and zone would allow simultaneously populated fields.
It would need an additional rule to reject that combination.
The enum encodes mutually exclusive cases directly.

## Destructure a value with match

A pattern asks whether the value has a particular shape and
can bind fields that have that shape. **Destructuring** is
using a pattern to access parts of a value. Each **match
arm** pairs a pattern with the expression to evaluate when
that pattern matches. Patterns are tried in order. The
first successful match whose guard, if present, succeeds
supplies the arm's result.

```rust,edition2021
{{#include ../../examples/data-modeling/src/route.rs}}

let parcel = Route::Deliver { zone: 7, fragile: true };
assert_eq!(parcel.desk(), Desk::Padded(7));
assert_eq!(Route::Locker(7).desk(), Desk::Locker(7));
assert_eq!(parcel.desk(), Desk::Padded(7));
```

The shared receiver keeps the route available after calling
the method. In the locker pattern, `number` borrows the
number because the match examines a shared reference.
`*number` copies the byte into the desk result. The named
pattern tests the handling flag and binds the zone. Each
delivery arm covers one of the Boolean values, and every
zone within that case has exactly one result.

The match implements the table directly. Counter and
locker cases preserve their routing identity. Both delivery
cases carry the original zone to the corresponding desk.
The implementation does a fixed number of tag and flag
decisions and contains no loop. That says how work scales
for this small model. Compiler optimization can choose a
different machine sequence with the same observable result.

## Prove coverage and test distinction

A match is **exhaustive** when every possible input value
can select an arm. Covering `Deliver` with two named
patterns works because its Boolean field has exactly two
values. Coverage is a property of the patterns and input
type. A compiler check for coverage does not prove that
the chosen result is correct.

**Compile-fail Rust.** Adding a status without updating a
match leaves an uncovered case. Rust 1.75.0 reports E0004
for the missing `Cancelled` arm below.

```rust,compile_fail,edition2021
{{#include ../../tests/fixtures/non_exhaustive_route.rs}}
```

A wildcard `_` matches anything still uncovered. Use it
when a deliberately uniform fallback applies. When each
variant carries a distinct business rule, listing those
variants makes a future addition prompt a review at the
decision point. A guarded arm such as `Value(x) if x > 0`
usually needs an unguarded fallback, since the pattern alone
cannot guarantee the guard succeeds.

For the route table, test the counter and the locker.
Keep the numeric input fixed while changing the handling
flag to verify that the selected desk changes. The
integration test covers all 769 inputs. This feasible
enumeration checks the implementation against the table.
The case argument explains why the table covers the entire
domain.

## Choose whether the payload moves

A pattern over an owned enum can move an owned field to
its new binding. Matching a reference can instead borrow
the payload. Compare the two examples here.

```rust,edition2021
enum Reading {
    Text(String),
    Missing,
}

let reading = Reading::Text(String::from("signal"));
let length = match &reading {
    Reading::Text(text) => text.len(),
    Reading::Missing => 0,
};
assert_eq!(length, 6);

let owned = match reading {
    Reading::Text(text) => text,
    Reading::Missing => String::new(),
};
assert_eq!(owned, "signal");
```

The first match borrows the enum. The second obtains the
String by value. In the text case that move transfers the
allocation instead of cloning it. A later use of the
whole moved enum is unavailable. In the missing case the
fallback constructs an empty string. If empty text and a
missing reading have different meanings, that fallback
also changes the output contract by merging the cases.
Chapter 28 models those alternatives explicitly.

A compiler must distinguish variants in memory, but the
default enum representation depends on the type and target.
The tag is useful as a logical model. It does not establish
a particular byte at a particular offset. Defining a file
format still requires a deliberate encoding.

## Practice and assessment

{{#include ../../exercises/C027.md}}

## Summary and further reading

Sum types distinguish alternatives and associate fields with
the applicable variant. A match covers the variants and
destructures their payloads. Borrowing and moving a payload
give callers different ownership outcomes.
[Rust 1.75.0 Reference on enumerations](https://doc.rust-lang.org/1.75.0/reference/items/enumerations.html) · [Match expressions](https://doc.rust-lang.org/1.75.0/reference/expressions/match-expr.html).

[Structs and products](026-structs-and-products.md) · [Solutions](../data-modeling-solutions.md#chapter-027) · [Option, Result, and domain states](028-option-and-result.md)
