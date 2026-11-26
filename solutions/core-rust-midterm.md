## Core Rust midterm answers

### 1. Representation and cost

The identifier is `0x1234` and the count is `0x12345678`.
On a target where `u32` requires four-byte alignment, a C-layout
struct can insert padding between the two fields to put the count at
offset four rather than at offset two. Check the wire length first,
then pass bytes at offsets 0 and 1 to `u16::from_le_bytes` and those
at offsets 2 through 5 to `u32::from_le_bytes`. This reads six bytes
at defined wire positions and uses no typed reference to wire memory.

**Rubric.** Two points for correct values, two for a target-specific
alignment explanation, and two for a checked, endian-aware decoder.

### 2. Borrowing and diagnostics

Two separate indexing borrows can overlap from the compiler's
perspective. After sorting the indices, split at the larger one. The
smaller index is in the left half while the larger index becomes the
first index in the right half. For reversed arguments, swap the order
of references on return.

```rust,edition2021
use std::cmp::Ordering;

fn pair_mut<T>(items: &mut [T], i: usize, j: usize) -> Option<(&mut T, &mut T)> {
    if i >= items.len() || j >= items.len() {
        return None;
    }
    match i.cmp(&j) {
        Ordering::Less => {
            let (left, right) = items.split_at_mut(j);
            Some((&mut left[i], &mut right[0]))
        }
        Ordering::Greater => {
            let (left, right) = items.split_at_mut(i);
            Some((&mut right[0], &mut left[j]))
        }
        Ordering::Equal => None,
    }
}

let mut values = [0, 1, 2];
let (first, second) = pair_mut(&mut values, 2, 0).unwrap();
*first = 9;
*second = 8;
assert_eq!(values, [8, 1, 9]);
assert!(pair_mut(&mut values, 1, 1).is_none());
assert!(pair_mut(&mut values, 1, 3).is_none());
```

**Rubric.** Two points for the aliasing explanation, two for correct
partitioning and order, and two for the reverse and invalid cases.

### 3. Abstraction

The returned slice borrows from that particular call's mutable borrow
of the reader. A second call that needs a mutable borrow must wait
until the earlier reference is no longer used. A generic call can use
the statically known implementation, while an object-safe trait method
called through `dyn` usually dispatches through a vtable. Rust 1.75.0
cannot use a GAT that is available on the object as part of an
object-safe trait. A borrowing interface designed for trait objects
can instead use object-safe methods with lifetime parameters if the
rest of its requirements allow it.

**Rubric.** Two points for the borrow dependency, two for comparing
dispatch, and two for the Rust 1.75.0 object-safety limitation.

### 4. Demand and observation

The output is `[6, 7]`. The predicate runs four times, including
the calls for 2 and 1. A positive demand stops at the position of
the last required match if there are enough matches. Otherwise the
consumer scans the full input. For example, threshold 9 and a budget
of two inspect all five inputs and yield only 9. A zero budget
invokes no predicate.

**Rubric.** Two points for correct values and call count, two for
the stopping rule, and two for a full-scan workload and zero demand.

### 5. Bounded updates

After staging the first two rows, the draft has four units available.
The request for five on line 3 fails with `Stock`, with `available`
equal to four. This error takes precedence because the fifth line has
not been reached. The original still has eight units. The draft
starts at eight and each passing step subtracts precisely its
quantity, so a partial change stays local until successful publication.

```rust,edition2021
mod row_errors {
    {{#include ../examples/reliability/src/row_errors.rs}}
}
mod stock {
    {{#include ../examples/reliability/src/stock.rs}}
}
{{#include ../examples/reliability/src/batches.rs}}

# fn main() {
    let mut stock = Stock::new(8);
    let result = apply_reservations(&mut stock, "3\n1\n5\n0\n0");
    assert!(matches!(result, Err(BatchError::Stock {
        line: 3,
        reason: InsufficientStock { requested: 5, available: 4 },
    })));
    assert_eq!(stock, Stock::new(8));
    assert!(apply_reservations(&mut stock, "3\n1").is_ok());
    assert_eq!((stock.available(), stock.held()), (4, 4));
# }
```

**Rubric.** Two points for error precedence and the error's values,
two for the draft invariant, and two for tests of failure and success.
