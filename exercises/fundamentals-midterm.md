Complete this assessment after Chapters 1–15. It measures
whether you can trace state, justify a finite-width
algorithm, distinguish an observed failure from its cause,
and check a text interface on hostile or incomplete input.
Each section is worth six points. For Problems 4 and 5,
write down your prediction before running commands and
record which test distinguishes a plausible wrong answer.

1. **Machine state, beginner.** Start the Chapter 1
   machine with input `[250, 8, 7]` and zeroed state.
   Run the instruction sequence
   `READ, STORE 0, READ, ADD 0, WRITE, HALT`.
   Record the program counter, register, memory cell 0,
   number of bytes consumed, and output after each step.
   Classify the result under ordinary and wrapped addition.
2. **Bounds, intermediate.** Derive a Boolean
   guard that is true exactly when adding two `u16`
   values fits `u16`. The guard must never overflow.
   State the integer domain, prove both directions, and
   give three boundary cases, including a valid result
   and a rejected input with the same wrapped 16-bit
   representation.
3. **Cost and probability, intermediate.** A scan of
   nine values has a unique hit uniformly distributed
   over its positions when successful, and one in three
   requests misses. Derive the expected number of
   comparisons and give the best and worst counts.
   Describe how making successful requests always hit
   the first value would change the expectation.
4. **Compiler diagnostics, intermediate.** The body of
   a function declared `fn total() -> u16` contains
   `let a = 7_u16;` followed by `a + 6;`. Predict
   why Rust rejects the body. Try the code on 1.75.0,
   change the minimum necessary punctuation, and
   verify that calling the repaired function returns 13.
5. **Text validation, challenge.** Implement a function
   taking bytes and a byte budget. It returns absence
   for any invalid UTF-8 in the input and otherwise the
   longest valid UTF-8 prefix that fits. Give a
   correctness argument and test ASCII, a multibyte
   boundary, invalid bytes after a valid prefix, and a
   zero budget. State which operations traverse input.
