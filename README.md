
![Strontium](https://s3.fr-par.scw.cloud/strontium.dev/banner_green.svg)

# Introduction

Strontium is a register-based bytecode virtual machine with typed registers and multimethod dispatch, built to power the [Mag](https://github.com/mag-language) programming language.

Mag source code is parsed and compiled to Strontium bytecode by `magc`, then executed here.

## Registers

The following registers are pre-allocated when the machine starts:

| **Register** | **Type**       | **Content** | **Description**                                          |
| ------------ | -------------- | ----------- | -------------------------------------------------------- |
| `bc`         | `Array<UInt8>` | `[]`        | Bytecode of the running program.                         |
| `arg`        | any            |             | Argument register for multimethod dispatch.              |
| `ret`        | any            |             | Return value register for method calls.                  |
| `r1..r8`     | `Empty`        |             | General-purpose registers, allocated as needed.          |

## Instruction Set

| **Opcode** | **Name**      | **Description**                                                                                      |
| ---------- | ------------- | ---------------------------------------------------------------------------------------------------- |
| 0          | `HALT`        | Stop execution.                                                                                      |
| 1          | `LOAD`        | Load a literal value into a register.                                                                |
| 2          | `MOVE`        | Move a value from one register to another (source is cleared).                                       |
| 3          | `COPY`        | Copy a value from one register to another (source is unchanged).                                     |
| 4          | `CALCULATE`   | Arithmetic on two registers (`ADD`, `SUBTRACT`, `MULTIPLY`, `DIVIDE`, `MODULO`, `POWER`, `SQRT`).   |
| 5          | `COMPARE`     | Comparison on two registers (`EQ`, `NEQ`, `LT`, `LTE`, `GT`, `GTE`); result written to a third.     |
| 6          | `BITWISE`     | Bitwise operation (`AND`, `OR`, `XOR`, `NOT`, `LSH`, `RSH`).                                        |
| 7          | `JUMP`        | Set the program counter to an absolute byte address.                                                 |
| 8          | `JUMPC`       | Conditional jump: jumps when the register at `conditional_address` holds `Boolean(false)`.           |
| 9          | `INTERRUPT`   | Emit a VM interrupt (`PRINT`, `READ`).                                                               |
| 10         | `CALL`        | Call a method at an absolute byte address, pushing a new stack frame.                                |
| 11         | `RETURN`      | Return from a method call, restoring the previous stack frame.                                       |
| 14         | `STOREL`      | Store a register value into the current stack frame's local variables.                               |
| 15         | `LOADL`       | Load a value from the current stack frame's local variables into a register.                         |
| 16         | `DISPATCH`    | Multimethod dispatch: match the value in `arg` against registered patterns and jump to the match.    |

## Multimethod Dispatch

Methods are registered in a dispatch table keyed by name. Each entry carries a `DispatchPattern`:

- `Value(v)` — matches a specific value (e.g. `0` for `fib(0)`), precedence 3
- `Type(t)` — matches any value of a given type (e.g. `Int` for `def foo(n Int)`), precedence 2
- `Any` — matches anything (untyped variable pattern), precedence 1

The first pattern with the highest precedence that matches the argument wins.

# Project Status

**_Alpha — API and semantics will change. Not suitable for production use._**

# License

Licensed under the MIT license.
