<span style="width: 100%; text-align: center">
  <img src="https://i.postimg.cc/bvqBkJb3/strontium-logo.png" alt="Strontium" 
/>
</span>

## Design

The Strontium VM is a [portable code 
machine](https://en.wikipedia.org/wiki/P-code_machine), a simulated processor 
which executes a pre-defined set of arithmetic, comparison and control flow 
instructions, encoded as bytecode. Instead of registers or a stack, this 
machine uses an array of bits to store run-time data.

### Data management

Data is stored and retrieved as bits from memory, which is a dynamic array. 
The virtual machine provides instructions for bit-wise arithmetic, comparison 
and memory management (set, unset, allocate, deallocate) operations. Language 
implementations may store data in this memory, in a format specific to their 
needs.

### Instructions

We're working with a 32-bit instruction set, so we can store four bytes in a 
single instruction. After the opcode has been put into the buffer, there 
isn't enough room for a 32-bit memory address, so they use a separate 
instruction. Due to this, the virtual machine may jump ahead from 2 to 7 
steps when interpreting a single instruction.

## Status

This project is still in an early stage of development.
The current features work as expected, but there may be breaking changes in 
the future.
Let's talk if you're interested on working on this project!

### Tests

The following tests have been implemented:

* `vm::memory::tests::shrink`
* `vm::memory::tests::shrink_error_handling`
* `vm::memory::tests::grow`
* `vm::memory::tests::lshift`
* `vm::memory::tests::rshift`
* `vm::memory::tests::single_and`
* `vm::memory::tests::single_or`
* `vm::memory::tests::single_xor`
* `vm::memory::tests::single_not`
* `vm::memory::tests::range_and`
* `vm::memory::tests::range_or`
* `vm::memory::tests::range_xor`
* `vm::memory::tests::range_not`

## License

This project is licensed under [Creative Commons Attribution Share-Alike 4.0](https://creativecommons.org/licenses/by-sa/4.0/)

