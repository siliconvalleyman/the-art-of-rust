## Fundamentals midterm answers

### 1. Machine state

| Step | Next PC | A | Cell 0 | Consumed | Output |
| --- | ---: | ---: | ---: | ---: | --- |
| READ | 1 | 250 | 0 | 1 | `[]` |
| STORE | 2 | 250 | 250 | 1 | `[]` |
| READ | 3 | 8 | 250 | 2 | `[]` |
| ADD | 4 | 2 | 250 | 2 | `[]` |
| WRITE | 5 | 2 | 250 | 2 | `[2]` |
| HALT | 5 | 2 | 250 | 2 | `[2]` |

The ordinary sum is 258 and the wrapped sum is 2. The
third byte is unread. Give four points for trace state,
one for the input position and trailing byte, and one
for explaining the two sums. Reporting 258 in the byte
register fails the representation constraint.

### 2. Bounds

Let both inputs range over 0 through 65,535. Compute
`a <= u16::MAX - b`. The difference is always
nonnegative under the domain. Adding `b` to the
inequality establishes the sufficient condition, and
subtracting it from the desired sum bound establishes
necessity. Valid pairs `(0,0)` and `(65535,0)` yield
0 and 65,535. Pair `(65535,1)` is rejected and would
also wrap to the word zero. Give one point each for
the domain, guard, safe intermediate, and each proof
direction, plus one for discriminating boundary tests.
Checking a wrapped result afterward loses the necessary
information.

### 3. Cost and probability

On successful requests the positions take 1 through 9
comparisons with equal probability, giving conditional
expectation 5. Misses cost 9. The expectation is
`(2/3) × 5 + (1/3) × 9 = 19/3`. The best and worst
are 1 and 9. If all hits cost one, the expectation
becomes `(2/3) × 1 + (1/3) × 9 = 11/3`.
Give two points for conditioning and the success
average, two for the weighted result, one for both
bounds, and one for the changed distribution. Averaging
only hit positions omits the miss contribution.

### 4. Compiler diagnostics

The trailing semicolon discards the sum as an expression
statement, so the body evaluates to unit. The return
type requires `u16`. The fixture below reproduces
diagnostic E0308 on Rust 1.75.0.

```rust,compile_fail,edition2021
{{#include ../tests/fixtures/wrong_tail_type.rs}}
```

Remove that semicolon.

```rust,edition2021
fn total() -> u16 {
    let a = 7_u16;
    a + 6
}

assert_eq!(total(), 13);
```

Give two points for the original failure and its type
explanation, two for the repair and expected result,
and two for observed build evidence and a test. Changing
the return type to unit would leave the caller without
the requested number.

### 5. Text validation

```rust,edition2021
fn valid_prefix(bytes: &[u8], budget: usize) -> Option<&str> {
    let text = std::str::from_utf8(bytes).ok()?;
    let mut end = budget.min(text.len());
    while !text.is_char_boundary(end) {
        end -= 1;
    }
    Some(&text[..end])
}

assert_eq!(valid_prefix(b"rust", 2), Some("ru"));
assert_eq!(valid_prefix("Aé".as_bytes(), 2), Some("A"));
assert_eq!(valid_prefix(&[b'A', 0xff], 1), None);
assert_eq!(valid_prefix(b"rust", 0), Some(""));
```

The `?` here returns absence immediately when decoding
fails. Validation considers the full sequence. Checking
only the requested prefix could accept corrupt input
outside the budget. When validation succeeds, every
skipped end position lies inside a scalar. The loop
cannot decrement at zero, and the first valid position
it reaches is the largest eligible one. UTF-8 validation
can traverse all input bytes. The loop moves at most
three positions and returns a view over the bytes.
Give two points for validation and correct failure
precedence, two for the loop and its proof, one for
boundary tests, and one for the cost model.
