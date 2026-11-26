# 006. Propositions, predicates, and proof

Firmware accepts two one-byte counts only when adding them
will fit in one byte. An engineer proposes checking whether the
first count is at most 255 minus the second. Is the check enough?
Can it reject an input that would fit? What does a test of the
pair `(254, 1)` tell us about the rest of the domain? This
chapter gives you a way to answer those questions before
relying on the implementation.

**Prerequisites.** [Boolean algebra and digital logic](../part-01/003-logic.md)
and [bounded arithmetic](../part-01/002-bits.md).

After this chapter, you should be able to state an implication
with a declared domain, negate quantified specifications, find
counterexamples, and prove a guard correct directly, by
contrapositive, and by contradiction.

## A precise claim has a truth value

A **proposition** is a statement with a definite truth value.
“255 fits in an unsigned byte” is true. “256 fits in an unsigned
byte” is false. “Read the next byte” is an instruction. A
**predicate** is a rule that produces a proposition when its
arguments are supplied. For example, `Fits(x)` could mean
`0 ≤ x ≤ 255` for an integer `x`. Writing the domain is
important. The same predicate would need a convention for a
measurement such as 1.5 if the domain included fractional
values.

The simplest logical **connectives** combine truth values. The
notation `¬P` means negation, `P ∧ Q` means conjunction
(both), and `P ∨ Q` means inclusive disjunction (at least
one). For an **implication** `P ⇒ Q`, `P` is the hypothesis
and `Q` its conclusion. It is false exactly when the
hypothesis holds and the conclusion fails.

| P | Q | P ⇒ Q | Q ⇒ P | P ∧ ¬Q |
| --- | --- | --- | --- | --- |
| false | false | true | true | false |
| false | true | true | false | false |
| true | false | false | true | true |
| true | true | true | true | false |

Why does the second row make `P ⇒ Q` true? Read `P` as
“this pair passed the guard” and `Q` as “this pair fits.”
The claim guarantees something about accepted pairs. A refused
pair, even if it fits, gives no counterexample to that
guarantee. This is **vacuous truth** for an implication with
a false hypothesis. To require acceptance of every fitting
pair, add the **converse** `Q ⇒ P`. The connector `⇔`
means both implications hold. An always-false guard satisfies
the first implication for all inputs but can still be useless.

A **precondition** is a condition the caller must satisfy for
a stated guarantee. A **postcondition** describes what follows
from a successful operation under the precondition. “The
input is sorted” and “a returned index contains the target”
could be statements in a search contract. To prove the
contract, you may use the precondition. A separate design
decision determines whether code checks it, uses a type
that establishes it, or requires its caller to prove it elsewhere.

## Quantifiers give the size of a claim

The universal quantifier `∀` means “for every.” The
existential quantifier `∃` means “there exists.” Both need
a domain. Let `B` be the set of integers 0 through 255.
The claim `∀ a ∈ B, ∀ b ∈ B, a + b ≤ 255` is false.
The pair `(255, 1)` is a **counterexample**, a choice
of arguments for which the proposed universal rule fails.
There is no need to list every other pair once a genuine
counterexample has been found.

To negate “every input satisfies the predicate,” look for one
input that fails it. To negate “at least one input satisfies
the predicate,” say every input fails it. Applying these rules
to a nested claim requires preserving the order of the
variables.

| Claim | Negation |
| --- | --- |
| `∀ x ∈ D, P(x)` | `∃ x ∈ D, ¬P(x)` |
| `∃ x ∈ D, P(x)` | `∀ x ∈ D, ¬P(x)` |
| `∀ x ∈ D, P(x) ⇒ Q(x)` | `∃ x ∈ D, P(x) ∧ ¬Q(x)` |

Different order can change truth. Suppose a test controller
can select inputs 0 and 1. Two monitors have different test
requirements. Monitor A passes only on 0 and monitor B passes
only on 1. For every monitor there exists a passing input.
There exists no one input that passes every monitor. Swapping
the quantifiers changes a separate choice for each monitor
into one shared choice. An **existence proof** gives a valid
example or otherwise shows one exists. A **witness** is such
an example.

| Input | Monitor A | Monitor B |
| --- | --- | --- |
| 0 | pass | fail |
| 1 | fail | pass |

*Figure 1. Each monitor column has a passing input. Every
input row has a failing monitor. The two quantifier orders
therefore give different answers.*

If the domain contains no values, the universal statement
is true and the existential statement is false. There is no
counterexample to the former and no witness for the latter.
An implementation that checks whether all measurements are
valid should therefore define whether an empty acquisition
is acceptable. If it must contain a reading, require at least
one reading and require each reading to be valid.
When testing such an interface, an empty case distinguishes those
requirements.

## Prove a byte bound three ways

Let `a` and `b` be integers in `B`. Put
`P(a,b) = (a ≤ 255 − b)` and
`Q(a,b) = (a + b ≤ 255)`, where addition here is ordinary
integer addition. The first specification, `P ⇒ Q`, says the
guard's accepted inputs fit. The second, `Q ⇒ P`, says no
fitting inputs are needlessly rejected.

**Direct proof.** Fix arbitrary `a` and `b` in the
domain and assume `P`. Adding the same integer `b` to
both sides preserves order. Thus `a + b ≤ (255 − b) + b`,
which simplifies to `a + b ≤ 255`. This is `Q`.
Since the choice of `a` and `b` was arbitrary, the
implication holds throughout the domain.

**Proof by contrapositive.** The contrapositive of
`P ⇒ Q` is `¬Q ⇒ ¬P`. Its one false row occurs under
the same conditions as in the original implication, so it
can serve as the proof target. Assume `¬Q`, hence
`a + b > 255`. Subtract `b` from both sides to obtain
`a > 255 − b`, which is `¬P`. This result rejects
an over-limit sum before performing it in the byte type.

**Proof by contradiction.** Assume a pair in `B` passes
the guard and has a sum greater than 255. Adding `b`
to the guard inequality implies the sum is at most 255.
The same quantity cannot be greater than and at most 255.
The assumption of an accepted, over-limit pair is impossible,
so `P ⇒ Q` holds. The contradictory statements must be
derived from the precise hypothetical failure. Pointing out
merely that overflow is undesirable cannot derive them.

**Converse.** Start with `Q` and subtract `b`. This
gives `P`, which proves completeness of the guard. The
full result is
`a ≤ 255 − b ⇔ a + b ≤ 255` for the stated domain.
For this claim the direct proof is short. The other two forms
show how the same facts can be organized when it is easier
to reason from an invalid result or an assumed counterexample.

## Connect the proof to finite-width code

Subtracting a byte `b` from 255 produces another value in
`B`. So the guard can compute `255 − b` before any risky
addition. On the accepted path, the proof ensures that adding
`a` and `b` fits. If you computed the wrapped sum first
and checked whether it was at most 255, the test would always
pass. Every wrapped byte already has that property, including a
wrapped 0 from `255 + 1`. The mathematical expression and
the finite-width operation must refer to the same quantity at
the point where you use a conclusion.

**Executable Rust preview.** The expressions in this example
come from the proof. Read the function as “when the guard
holds, return the sum, otherwise return no value” until the
first Rust chapters introduce the syntax.

```rust,edition2021
{{#include ../../mathematics/src/lib.rs:bounded_sum}}
assert_eq!(bounded_sum(254, 1), Some(255));
assert_eq!(bounded_sum(255, 1), None);
assert_eq!(bounded_sum(0, 0), Some(0));
```

The implementation uses a fixed number of operations per pair
and constant additional space. Widening the operands before
summing is another valid approach and gives a natural checking
implementation in the test suite. Its intermediate result can
represent every ordinary sum of two bytes. In production Rust,
a checked addition operation also expresses the intended
contract directly. The explicit guard makes the proof obligations
visible and can be adapted when the supported maximum is below the
type's maximum.

Review the boundary pair `(255, 0)`. The guard must accept
equality, so a comparison with `<` would be too strict.
For `(0, 0)`, the result is a successful zero. Representing
failure by the number 0 would make a success indistinguishable
from an error. The returned optional value preserves the
difference. Checking both maximum and zero is useful because
they exercise different representation assumptions.

## What verification evidence supports

A truth table enumerates all assignments for a known finite
set of Boolean variables. Exhausting the 65,536 byte pairs
against an independent widened oracle also covers every input
to the particular two-byte function. The quality of that
evidence depends on the finite domain, the correctness of
the oracle, and the conditions under which the test runs.
A proof can work for all nonnegative integers under its
assumptions, including a representation that has not been
implemented yet. The test can reveal a code transcription
error the mathematical proof did not address. Use both with
their actual scope in mind.

Treat surprising failures as a prompt to sharpen the
statement. Check the domain first. Check whether the desired
claim is an implication or an equivalence. If it says “every”
or “some,” look for a missing witness or an empty set of
inputs. When a compiler rejects an expression, record that
event and the rule causing it. A rejection is evidence about
the code's language constraints. The contract still needs its
own reasoning.

## Practice and assessment

{{#include ../../exercises/C006.md}}

## Summary and further reading

Write the domain and the meaning of each predicate. Check
both directions when a guard must accept exactly the valid
inputs. Negating a universal creates a counterexample search.
Direct proof, contrapositive, and contradiction can establish
the same implication through different starting assumptions.

For further worked proofs and logic exercises, see the authors'
course materials for [MIT Mathematics for Computer Science](https://ocw.mit.edu/courses/6-042j-mathematics-for-computer-science-fall-2010/).

[Operating systems, translation, and models of computation](../part-01/005-operating-systems.md) · [Solutions](../mathematics-solutions.md#chapter-006) · [Sets, relations, functions, and graphs](007-sets-and-graphs.md)
