
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
|`r1 .. r8`  |`Empty`| None| General purpose registers

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

Details about the binary encoding of particular instructions can be found in the wiki.

# Project Status

This project is still in an early stage of development and there may be breaking changes in 
the future. Most of the listed instructions have been implemented, but interrupts, copy and move are still missing.

Let's talk if you're interested on working on this project!

# Tests

*Coming soon*

# License

Licensed under the MIT license.
![Strontium](https://s3.fr-par.scw.cloud/strontium.dev/strontium_banner.svg)

# Introduction

Strontium is a lightweight register-based bytecode interpreter for statically and dynamically typed programming languages.

The machine operates on a set of named registers with type signatures and supports defining and calling multimethods.

It is built primarily to support Mag as the underlying, executive portion of the language engine. Any Mag source code first runs through the parser, is then compiled to Strontium bytecode and finally interpreted by this virtual machine.

## Instruction Set Architecture

A lightweight RISC-like instruction set architecture is used to keep the number of instructions small and easily maintainable, while more complex tasks are achieved using combinations of multiple bytecode instructions. The following sections tell more details about the registers and instruction set of this virtual machine.

## Registers

The following registers are pre-allocated when the machine starts:

|**Number**|**Register**|**Type**|**Description**
| -------- | ---------- | ------ | --------------
|  0       | `ip` 		 |`UInt`| Points to the position of the byte to be read next from the bytecode buffer in the `bc` register.
|  1       | `bc`      	 |`List<u8>`| Contains the bytecode of the running program.
|  2       | `arg`       |`Pattern`| Stores the parameter of the multimethod if one is currently being called.
|  3 .. 32 |`gp{3 .. 32}`|`*`| A set of 29 general-purpose registers, which are pre-allocated when the machine starts and filled with `Int`s containing zeroes. The program may then expand, shrink or mutate this register set freely.

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

Details about the binary encoding of particular instructions can be found in the wiki.

# Project Status

This project is still in an early stage of development and there may be breaking changes in 
the future. Most of the listed instructions have been implemented, but interrupts, copy and move are still missing.

Let's talk if you're interested on working on this project!

# Tests

The following tests have been implemented:

* `bytecode::tests::convert_halt`
* `bytecode::tests::convert_load`
* `machine::tests::execute_add`
* `machine::tests::execute_sub`
* `machine::tests::execute_mul`
* `machine::tests::execute_div`

# License

Licensed under the MIT license.


