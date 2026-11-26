## Binary and hexadecimal

| Decimal digit value | Binary nibble | Hex digit | Decimal digit value | Binary nibble | Hex digit |
| ---: | --- | --- | ---: | --- | --- |
| 0 | `0000` | `0` | 8 | `1000` | `8` |
| 1 | `0001` | `1` | 9 | `1001` | `9` |
| 2 | `0010` | `2` | 10 | `1010` | `A` |
| 3 | `0011` | `3` | 11 | `1011` | `B` |
| 4 | `0100` | `4` | 12 | `1100` | `C` |
| 5 | `0101` | `5` | 13 | `1101` | `D` |
| 6 | `0110` | `6` | 14 | `1110` | `E` |
| 7 | `0111` | `7` | 15 | `1111` | `F` |

Group binary positions in fours from the right. For a two-byte
unsigned field, the high byte has weight 256 and the low byte
has weight 1. Consult the field specification for which byte is
first.

## Trace vocabulary

| Term | Use in a trace |
| --- | --- |
| Address | Identifies a byte or a cell in the relevant address space |
| Carry | Adds a unit to the next more significant column |
| Instruction set architecture | Specifies instruction encoding and the state changes software can observe |
| Invariant | Property preserved at a specified point in repeated execution |
| Process | Running program instance with resources and an address space |
| Program counter | Identifies the next instruction in the architectural model |
| State | Data needed to determine subsequent behavior under the model |
| Thread | Execution context that can be scheduled within a process |
| Translation | Uses the current mapping and access mode to resolve an address |

## Common failures and checks

| Observed failure | First check |
| --- | --- |
| Sum wraps unexpectedly | Maximum mathematical sum and available bit width |
| Binary and hex numbers disagree | Group boundaries and weight of the high group |
| Cross-platform bytes disagree | Known multibyte fixture with distinct bytes and an explicit order |
| Branch skips an expected write | Condition operands and next program counter |
| Model cache reports a hit on its first request | Distinct representation of an empty slot |
| A write faults at an address that reads successfully | Read and write permissions in the caller's address space |
| Short CPU time but long elapsed time | Waiting intervals and scheduling delay |
