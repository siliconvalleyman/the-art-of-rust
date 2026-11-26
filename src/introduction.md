# Introduction

A program gives a computer a repeatable way to transform information.
The result can be a printed value, a message to another computer, or a
change to a file. To reason about that transformation, identify the
initial information, the rules that change it, and the result you
expect. You can do all three with a pencil before you install a tool.

## Work through a chapter

1. Read the prerequisites. If a term is unfamiliar, follow the
   prerequisite and trace the example there before coming back.
2. Predict the result of each example before reading its trace. Write
   down the initial values and the rule applied at each step.
3. Find one input for which your explanation could fail. An empty
   input, the largest representable value, or an interrupted operation
   can expose an assumption hidden by a typical example.
4. Attempt the exercises before opening the solutions. Where there is
   a proof, label the assumptions and show exactly where each one is
   needed. Where there is code, record how you ran it.
5. Use the knowledge check to explain a result in your own words.
   Being able to reproduce an output and being able to account for it
   require different evidence.

## Reading paths

New programmers can begin with [How computers execute instructions](part-01/001-machine.md).
The first part uses bounded paper machines, tables, and pseudocode.
At this stage, a trace is a table of state before and after each step.
Rust implementations in the solutions are optional previews until
the first Rust programming chapter.

Experienced readers can try the Part I problems before revisiting
explanations. If you can track the state but cannot explain why it must
have the stated value, continue through the proof exercises. If you can
explain the proof, change one assumption and see where the proof needs
repair.

Use the book's table of contents to find available material.
