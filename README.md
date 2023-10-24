
![Strontium](https://s3.fr-par.scw.cloud/strontium.dev/banner_green.svg)

# Introduction

Strontium is a bytecode machine with typed registers and multimethods for statically and dynamically typed programming languages.

It is built primarily to support [Mag](https://github.com/mag-language) as the underlying, executive portion of the language engine. Any Mag source code first runs through the parser, is then compiled to Strontium bytecode and finally interpreted by this virtual machine.

## Instruction Set Architecture

A lightweight RISC-like instruction set architecture is used to keep the number of instructions small and easily maintainable, while more complex tasks are achieved using combinations of multiple bytecode instructions.

## Registers

The following registers are pre-allocated when the machine starts:

|**Register**|**Type**|**Content**|**Description**
| -------- | ---------- | ------ | --------------
| `ip` 		 |`UInt64`| `0` | The index of the byte to be read next from the bytecode buffer in the `bc` register.
| `bc`       |`Array<UInt8>`| `[]`|The bytecode of the running program.
|`r1..r8`  |`Empty`| | General purpose registers

Use the `LOAD` instruction to load values from bytecode into general purpose registers at runtime.

## Instruction Set

The following instructions may then used in program bytecode to operate on the registers:

|**Opcode**| **Name**     | **Description**
| -------- | --------------- | ----------------------------
|  0  | `HALT` 		 | Stop all execution instantly.
|  1  | `LOAD`      	 | Load a `Value` from program bytecode into a register.
|  2  | `MOVE` 		 | Move a value from one register to another. The source register will be cleared after the operation.
|  3  | `COPY` 		 | Copy a value from one register to another. The source will be left untouched.
|  4  | `CALCULATE` 	 | Perform a calculation (`ADD`, `SUBTRACT`, `MULTIPLY`, `DIVIDE`) on two registers and write the result to a third. 
|  5  | `COMPARE` 	     | Perform a comparison (`EQ`, `NEQ`, `LT`, `LTE`, `GT`, `GTE`) on two registers and write the result to a third.
|  6  | `BITWISE` 	     | Perform a bitwise operation on one (`NOT`) or two (`AND`, `OR`, `XOR`, `LSH`, `RSH`) registers and write the result into another.
|  7  | `JUMP` 	     | Set the program counter to a value from a location, using one of the methods (`absolute`, `forward`, `backward`)
|  8  | `JUMPC` 	     | Same as the previous instruction, but with an additional register address argument. Will only perform the jump if the given register contains a `Value::Bool(true)`.
|  9  | `INTERRUPT` 	 | Emit an event that needs immediate attention (`READ`, `PRINT`)
|  10 | `CALL` 	 | *Unimplemented method call instruction*
|  11 | `RETURN` | *Unimplemented method return instruction*

This README will soon be updated with further documentation.

# Project Status

**_This is still very much in the alpha phase, so the semantics and API surface of this library will change dynamically over time. Do not rely on this in any production-critical way and expect earth-shattering change._**

Let's talk if you're interested on working on this project!

# Tests

*Coming soon*

# License

Licensed under the MIT license.
