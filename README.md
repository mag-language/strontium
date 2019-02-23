
<br />

<center><img src="https://i.postimg.cc/bvqBkJb3/strontium-logo.png" alt="Strontium" /></center>
<br />
<center> <i>Please note that this project is still in an early stage, expect breaking changes.</i> </center>

## What is Strontium?

Strontium is an embeddable process virtual machine for dynamically and statically typed programming languages. This crate provides the VM along with a bytecode format and parser. Strontium aims to be lightweight and to provide as much freedom to the language implementor as possible.



### Features
A _checked_ box indicates an implemented feature.

* &nbsp;[ ✓ ] &nbsp;&nbsp;Number arithmetic on `Int`, `UInt` and `Float` registers
* &nbsp;[ ✓ ] &nbsp;&nbsp;Bitwise arithmetic (`AND`, `OR`, `XOR`, `NOT`, `LSH`,  ...) on a sequence of bits
* &nbsp;[ &nbsp;&nbsp;&nbsp;   ] &nbsp;&nbsp; JIT-compile bytecode to machine code using Cranelift
* &nbsp;[ &nbsp;&nbsp;&nbsp;   ] &nbsp;&nbsp; Hardware Interrupts

### Status

This project is still in an early stage of development.
The current features work as expected, but there may be breaking changes in 
the future.
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

