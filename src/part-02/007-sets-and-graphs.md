# 007. Sets, relations, functions, and graphs

A build planner has three jobs: parse, check, and package.
Package requires check, and check requires parse. The job list
alone cannot express those constraints. We need a language for
which jobs are available, which pairs are connected, and which
paths can be inferred from the connections. The same
distinctions arise when modeling state transitions and routing.

**Prerequisite.** [Propositions, predicates, and proof](006-propositions.md).

After this chapter, you should be able to model a domain with
sets, distinguish relations from functions, partition objects
by a stated equivalence, and derive a graph query from a
relation rather than relying on a particular drawing.

## Sets model membership

A **set** collects distinct elements. For a set `S`, the
expression `x ∈ S` means that `x` is a member of `S`.
An enumeration such as `{parse, check, package}` lists its
members. A property can specify it instead, for example all
jobs whose input is available. For a finite set, `|S|`
denotes its number of members.

The ordering and repetition of terms in a set enumeration
carry no information. A retry queue may contain the same job
twice and may care which attempt comes first. Use a sequence
or track attempt identifiers to represent that queue. A set
is suitable when the question is simply whether a job
belongs to the collection.

Write `S ⊆ T` when every member of `S` is also a member
of `T`. If both `S ⊆ T` and `T ⊆ S`, they are the same
set. The **empty set** is `∅`, a subset of every set by
the same vacuous-truth rule we saw in Chapter 6.

Let `R = {parse, check}` be the jobs requested in one batch.
Let `C = {parse, package}` denote jobs with cached results.

| Operation | Membership rule | Example |
| --- | --- | --- |
| Union `R ∪ C` | In at least one set | `{parse, check, package}` |
| Intersection `R ∩ C` | In both sets | `{parse}` |
| Difference `R ∖ C` | In `R` and absent from `C` | `{check}` |

The difference gives a list of jobs that still need results under
the stated cache rule. If cached results can be stale, change the
definition of `C` to require a valid result for the requested
input. Correct set algebra cannot recover an omitted condition
in the model. In a program, represent the set with a bit per job,
a sorted array of identifiers, or a hash-based collection.
The exact choice changes space and operation costs, while the
formal membership rule remains the same.

## Relations connect pairs

An **ordered pair** `(x, y)` records an order. For example,
`(parse, check)` and `(check, parse)` generally express
different facts. The **Cartesian product** `A × B` is the
set of pairs whose first and second elements come from `A`
and `B`, respectively. A **relation** from `A` to `B`
is a subset of that product. Write `x R y` to mean
`(x, y) ∈ R`. In the job example, let `x R y` mean
“x is a direct prerequisite of y.”

Relations on the same set can have structural properties.

| Property | Condition on a relation R |
| --- | --- |
| Reflexive | Every `x` is related to itself |
| Symmetric | If `x R y` then `y R x` |
| Transitive | If `x R y` and `y R z` then `x R z` |
| Antisymmetric | If both `x R y` and `y R x`, then `x = y` |

Direct dependency edges are not transitive in the example.
Parse is directly required by check, and check by package.
The implication would also require a direct parse-to-package
edge. The induced “can reach along zero or more edges”
relation is transitive, since concatenating two paths gives a
sequence of valid edges. Zero-edge paths also make it reflexive.

A relation that is reflexive, symmetric, and transitive is an
**equivalence relation**. Grouping jobs by artifact kind
gives an example: two jobs are equivalent when their chosen
artifact kind is equal. Equality is reflexive, symmetric and
transitive, so the grouping relation has those properties.
An **equivalence class** consists of all objects equivalent
to a given object. Classes form a **partition**, a collection
of pairwise disjoint nonempty sets whose union is the whole
domain. Every object belongs to its own class by reflexivity.
If two classes overlap, a common member connects their
representatives by symmetry and transitivity. Any member of
either class is then related to both representatives, so the
classes are equal.

Compare grouping by equal input identifier with grouping by
“has a common input identifier.” The second relation can
relate job A using only input `x` to job B using `x` and
`y`, and B to job C using only `y`. It need not relate A
to C. Treating it as equality can merge objects that fail
the engineering condition the grouping was meant to enforce.

## Functions add existence and uniqueness

A **function** from domain `A` to codomain `B` assigns
exactly one element of `B` to every member of `A`. We
write `f: A → B` and `f(x)` for the assigned value.
A relation with zero outputs for one input, or several
outputs for one input, violates that definition. Both
conditions can be modeled by changing the codomain. For
example, a lookup that may miss can return either an object
or an explicit absent value, and a search can return one
collection of all matches.

A function is **injective** if equal outputs imply equal
inputs, and **surjective** when every member of its codomain
occurs as an output. Both properties together make it
**bijective** and give a well-defined inverse: existence of
the inverse output is guaranteed by surjectivity, and its
uniqueness by injectivity. An allocation from jobs to distinct
slot IDs can be injective even when it leaves some slots
unused. A hash code mapping arbitrarily many possible strings
to a finite number of outputs cannot be injective over that
whole domain.

When the output of function `f` fits the input domain of
`g`, their **composition** `g ∘ f` maps an input `x`
to `g(f(x))`. For a byte count, let `f` widen the count
to a whole number, and let `g` format that number as a
decimal string. Composing them first widens, then formats.
Changing the written order of the functions may also change
which intermediate type is required. Relation composition
generalizes the idea: there is a connection from `x` to
`z` if there is some `y` such that `x R y` and
`y S z`.

## Graphs give a structure to a relation

A **directed graph** has a vertex set `V` and a set of
directed edges `E ⊆ V × V`. A **vertex** is an object in
the graph. An **edge** identifies an allowed one-step
connection. In an **undirected graph**, an edge connects
its endpoints in both directions. Unless specified otherwise
here, edges have no weights and each ordered pair occurs at
most once. A **walk** is a sequence of vertices in which
each consecutive pair is an edge. A **path** in the stricter
sense also forbids repeated vertices. A **cycle** is a
nonempty closed walk with no vertex repetitions other than
its starting and ending vertex. A self-edge is a one-edge
cycle in this directed graph model.

```text
parse ----> check ----> package
  |                       ^
  +-------> sign ---------+
```

*Figure 1. An edge is directed from prerequisite to dependent.
Two distinct length-two walks connect parse to package.
Reachability alone does not say how many walks exist.*

The **out-degree** of a vertex counts its outgoing edges.
Its **in-degree** counts its incoming edges. In Figure 1,
package has in-degree two and out-degree zero. Reachability
from parse includes parse itself when we allow a walk of
length zero. Reachability can equivalently be expressed by
a path as defined above: if a walk repeats a vertex, remove
the segment between two occurrences of that vertex. The
remaining sequence still connects the same endpoints, and
each removal strictly shortens the walk. A finite walk thus
gives a path.

One memory representation is an **adjacency matrix**:
number the vertices and let `edge[i][j]` be true
exactly when the graph contains edge `(i, j)`. A single
edge check needs two indices. An `n`-vertex matrix stores
`n²` Boolean entries. An **adjacency list** stores just
the outgoing neighbors for each vertex. It uses storage
proportional to `n + m` for `n` vertices and `m`
edges, ignoring representation overhead beyond a constant
amount per list and per entry. A dense matrix is convenient
for constant-time pair queries in this model. A list lets
a traversal inspect only vertices and neighbors it actually
visits. The physical representation can also affect cache
accesses, especially when lists are scattered or a matrix is
packed.

For a two-edge walk from `i` to `j`, the required
intermediate vertex satisfies both `edge[i][k]` and
`edge[k][j]`. That suggests visiting candidate vertices
`k` in order and returning true on the first match.

**Executable Rust preview.** This square matrix uses indices
0 for parse, 1 for check, 2 for sign, and 3 for package.
The function distinguishes out-of-domain vertex indices
from a valid query with no two-edge walk.

```rust,edition2021
{{#include ../../mathematics/src/relations.rs:two_steps}}
let edges = [
    [false, true,  true,  false],
    [false, false, false, true ],
    [false, false, false, true ],
    [false, false, false, false],
];
assert_eq!(two_steps(&edges, 0, 3), Some(true));
assert_eq!(two_steps(&edges, 0, 1), Some(false));
assert_eq!(two_steps(&edges, 4, 3), None);
```

After testing the first `k` vertices, a continued search
has ruled out all of them as intermediates. Initially the
tested prefix is empty. At each step the conjunction detects
exactly the two edges needed. An unsuccessful check adds
one vertex to the ruled-out prefix. On exhaustion every
intermediate was ruled out, so no two-edge walk exists.
The number of untested vertices decreases. For a valid query
on an `n`-vertex matrix, the first candidate can succeed
in one check, and a miss or final candidate uses `n`
checks. The extra space is constant beyond the supplied
matrix. This query considers a fixed walk length. Longer
walks and full reachability need a different algorithm.

## Practice and assessment

{{#include ../../exercises/C007.md}}

## Summary and further reading

Sets capture membership. Relations capture pairs. Functions
constrain the outputs per input. For a graph relation, the
edge direction, treatment of zero-length walks, and chosen
memory representation affect what queries mean and how
efficiently they can run.

[Propositions, predicates, and proof](006-propositions.md) · [Solutions](../mathematics-solutions.md#chapter-007) · [Induction, strong induction, and recurrence relations](008-induction.md)
