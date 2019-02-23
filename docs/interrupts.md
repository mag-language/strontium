## Strontium Interrupts

Applications need a way to interact with the outside world, e.g. for networking or printing a string. For this purpose, this document declares a list of actions that may be dispatched in the virtual machine. 

### Interrupts

* ### `print`
  * Print the byte which starts at the given bit address in the virtual memory


* ### `read`
  * Read a byte from standard input and write it to memory

* ### `flush`
  * Write the `print` buffer to standard output, then empty it

