![Strontium](https://i.imgur.com/cKsPKPh.png)

# What is Strontium?

Strontium is a lightweight virtual machine for dynamically and statically typed programming languages. It currently operates on a set of 64 registers, which each hold a 64-bit floating point value. A memory abstraction provides the storage space for anything else.

# Instruction Set Architecture

A lightweight RISC-like instruction set is used to represent programs. Strontium favors the composition of functionality using basic functions over the creation of new, custom instructions for specific tasks. The following is a list of all 10 available instructions:

| **Value**| **Instruction** | **Description**               
| -------- | --------------- | ----------------------------
|  `0x01`  | `HALT` 		 | Stop all execution instantly.
|  `0x02`  | `LOAD`      	 | Load a floating-point value into a register.
|  `0x03`  | `MOVE` 		 | Move a value from a register to a memory location or vice versa. The first argument is the source, the second is the destination. Swap the arguments to change the direction. The source will be cleared after the operation.
|  `0x04`  | `COPY` 		 | Copy a value from a register to a memory location or vice versa. The source will be left untouched.
|  `0x05`  | `CALCULATE` 	 | Perform a calculation (`ADD`, `SUBTRACT`, `MULTIPLY`, `DIVIDE`) on two registers and write the result to a third. 
|  `0x06`  | `COMPARE` 	     | Perform a comparison (`EQ`, `NEQ`, `LT`, `LTE`, `GT`, `GTE`) on two registers and write the result to a memory address or a register.
|  `0x07`  | `MEMORY` 	     | Perform a bitwise operation (`AND`, `OR`, `XOR`, `NOT`, `LSH`, `RSH`) on two or three addresses, or perform a memory operation (`GROW`, `SHRINK`, `SET`, `UNSET`)
|  `0x08`  | `JUMP` 	     | Set the program counter to a value from a location, using one of the methods (`absolute`, `forward`, `backward`)
|  `0x09`  | `JUMPC` 	     | Set the program counter to a value from a location if the byte at a given address in memory has the value of `1`
|  `0x0A`  | `INTERRUPT` 	 | Emit an event that needs immediate attention (`READ`, `PRINT`)

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

