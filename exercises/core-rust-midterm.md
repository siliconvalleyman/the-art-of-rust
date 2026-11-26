Complete this assessment after Chapters 16–45. Each problem is
worth six points. For **diagnostics**, record your prediction before
compiling. For **bounded updates**, explain the invariant in addition
to providing tests.

1. **Representation and cost.** A hand-written wire format contains
   a `u16` identifier followed immediately by a `u32` count, both
   little endian. Decode `34 12 78 56 34 12` in hexadecimal. Explain
   why a six-byte wire record can be smaller than a `#[repr(C)]`
   Rust struct containing the same fields. Specify a parsing strategy
   that avoids depending on the target's layout or alignment.
2. **Borrowing and diagnostics.** Write `pair_mut` that takes a mutable
   slice and two different indices and returns references in the
   caller's requested order. Return `None` for equal or out-of-range
   indices. First explain why two independent indexing expressions
   do not prove disjointness. Then use a standard slice operation to
   demonstrate the proof in the repair. Test the reverse-index case.
3. **Abstraction.** A reader repeatedly lends a slice from an internal
   mutable buffer. Describe the lifetime of each returned slice.
   Explain the difference between a generic reader using a known
   implementation and an ordinary method call through a trait
   object. State why erasing a trait containing an unrestricted GAT
   itself fails the object-safety requirements on Rust 1.75.0.
4. **Demand and observation.** Values arrive as `[2, 6, 1, 7, 9]`.
   Determine the output of filtering at an inclusive threshold of 6
   and taking two results. Record how many predicate calls occur
   when consuming the result. Derive the general stopping rule and
   give one workload on which this approach still scans every input.
5. **Bounded updates.** The reservation API starts with eight units
   and receives the batch `"3\n1\n5\n0\n0"`. Predict the returned
   error and the state left visible. Give a correctness argument for
   the draft and write tests that distinguish this precedence from
   pre-counting the lines or mutating the original as each row passes.
