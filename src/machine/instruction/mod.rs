use crate::types::{
	MemoryAddress,
	RegisterAddress,
	Location,
};

pub mod encode;

/// Represents a callable machine instruction
#[derive(Debug, Clone, PartialEq)]
pub enum Instruction {
	/// Stop all further execution
	HALT,

	/// Load a numeric value into a register
	LOAD {
		value:    f64,
		register: RegisterAddress,
	},

	/// Move a value from a register to memory or vice versa
	MOVE {
		source: 	 Location,
		destination: Location,
	},

	/// Copy a value from a register to memory or vice versa
	COPY {
		source:      Location,
		destination: Location,
	},

	/// Perform a calculation on two registers and write the result to a third
	CALCULATE {
		method: 	 CalculationMethod,
		operand1:    RegisterAddress,
		operand2:    RegisterAddress,
		destination: RegisterAddress,
	},

	// Compare two registers and write the result (`0` or `1`) into a given memory address
	COMPARE {
		method: 	 ComparisonMethod,
		operand1:    RegisterAddress,
		operand2:    RegisterAddress,
		destination: MemoryAddress,
	},

	// Perform a memory operation (`and`, `or`, `xor`, `not`, `lsh`, `rsh`, `grow`, `shrink`, `set`, or `unset`)
	MEMORY {
		method: MemoryMethod
	},

	/// Set the program counter to the given value
	JUMP {
		destination: usize,
	},

	/// Set the program counter to a value if the given byte has the value of `1`
	JUMPC {
		destination: usize,
		conditional_address: u64,
	},

	/// Set off a hardware interrupt, for example to print a character to standard output
	INTERRUPT {
		interrupt: Interrupt,
	},

	/// Push a pointer value onto the call stack
	PUSH {
		value: MemoryAddress,
	},

	/// Remove the most recent call stack value
	POP,

	/// Set the virtual machine's last known point to the current
	/// program counter, push the address and arguments of 
	/// the function to be called onto the stack, and set the
	/// program counter to the function pointer.
	CALL {
		function_pointer: MemoryAddress,
		arguments: Vec<MemoryAddress>,
	},

	/// Restore the program counter from the last known point, then 
	/// pop the function pointer and arguments off the stack
	RETURN {
		/// Specifies how many arguments should be popped off the stack
		argument_count: u32,
	}
}

#[derive(Debug, Clone, PartialEq)]
/// A signal to the machine, indicating that an event needs immediate attention. This enumeration
/// contains the interrupt types which are supported by the virtual machine.
pub enum Interrupt {
	/// Print the ASCII character from the given address in memory
	PRINT {
		address: u64
	},

	/// Read an ASCII character from standard input and write it to the given memory address
	READ {
		address: u64
	}
}

#[derive(Debug, Clone, PartialEq)]
/// Basic mathematic methods which operate on two registers and write the result to a third.
pub enum CalculationMethod {
	ADD,
	SUBTRACT,
	MULTIPLY,
	DIVIDE,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ComparisonMethod {
	EQ,
	NEQ,
	GT,
	GTE,
	LT,
	LTE,
}

#[derive(Debug, Clone, PartialEq)]
pub enum MemoryMethod {
	AND {
		a: u64,
		b: u64,
		out: u64,
		len: usize,
	},

	OR {
		a: u64,
		b: u64,
		out: u64,
		len: usize,
	},

	XOR {
		a: u64,
		b: u64,
		out: u64,
		len: usize,
	},

	NOT {
		a: u64,
		out: u64,
		len: usize,
	},

	LSH {
		a: u64,
		out: u64,
		amount: usize,
		len: usize,
	},
	
	RSH {
		a: u64,
		out: u64,
		amount: usize,
		len: usize,
	},

	GROW {
		amount: u64,
	},

	SHRINK {
		amount: u64,
	},

	SET {
		address: u64,
		value: u8,
	},

	UNSET {
		address: u64
	}
}