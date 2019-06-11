![Strontium](https://strontium.lyn.cloud/img/banner.png)

# What is Strontium?

Strontium is a lightweight virtual machine for dynamically and statically typed programming languages. It operates on a set of typed registers, which each hold 32 numeric (`Int`, `UInt`, and `Float`) 64-bit values. A memory vector provides byte-wise storage for anything else.

## Instruction Set Architecture

A lightweight RISC-like instruction set is used to represent programs, favoring the **composition of functionality using basic functions** over new, custom instructions for specific tasks. The following is a list of all available instructions:

| **Value**| **Instruction** | **Description**               
| -------- | --------------- | ----------------------------
|  `0x01`  | `HALT` 		 | Stop all execution instantly.
|  `0x02`  | `LOAD`      	 | Load a `Int`, `UInt` or `Float` value into a register.
|  `0x03`  | `MOVE` 		 | Move a value from a register to a memory location or vice versa. The first argument is the source, the second is the destination. Swap the arguments to change the direction. The source will be cleared after the operation.
|  `0x04`  | `COPY` 		 | Copy a value from a register to a memory location or vice versa. The source will be left untouched.
|  `0x05`  | `CALCULATE` 	 | Perform a calculation (`add`, `subtract`, `divide`, `multiply`) on two registers and write the result to a third. 
|  `0x06`  | `COMPARE` 	     | Perform a comparison (`EQ`, `NEQ`, `LT`, `LTE`, `GT`, `GTE`) on two registers and write the result to a third.
|  `0x07`  | `MEMORY` 	     | Perform a bitwise operation (`and`, `or`, `xor`, `not`, `lsh`, `rsh`) on two or three addresses, or perform a memory operation (`grow`, `shrink`, `set`, `unset`)
|  `0x08`  | `JUMP` 	     | Set the program counter to a value from a location, using one of the methods (`absolute`, `forward`, `backward`)
|  `0x09`  | `INTERRUPT` 	 | Emit a virtual hardware interrupt (`read`, `print`)

Details about the binary encoding of particular instructions can be found in the wiki.

### Status

This project is still in an early stage of development and there may be breaking changes in 
the future. Most of the listed instructions have been implemented, but interrupts, copy and move are still missing.

Let's talk if you're interested on working on this project!

### Tests

The following tests have been implemented:

* `bytecode::tests::convert_halt`
* `bytecode::tests::convert_load`
* `machine::tests::execute_add`
* `machine::tests::execute_sub`
* `machine::tests::execute_mul`
* `machine::tests::execute_div`
* `machine::tests::execute_halt`
* `machine::tests::execute_load`
* `machine::tests::execute_jump`


## License

Licensed under the MIT license.

