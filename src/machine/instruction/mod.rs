use crate::types::{
	MemoryAddress,
	RegisterAddress,
	StrontiumError,
	Location,
};
use super::RegisterValue;

mod executors;
pub use self::executors::*;

use super::Strontium;

pub trait Executor {
    fn execute(&self, machine: &mut Strontium) -> Result<bool, StrontiumError>;
}

pub enum Instruction {
	Halt,
	Load(LoadInstruction),
}

pub struct LoadInstruction {
	/// The name of the register to load the value into.
	name:  String,
	/// The value to load into the register.
	value: RegisterValue,
}

/// Represents a callable machine instruction
/*
#[derive(Debug, Clone, PartialEq)]
pub enum Instruction {
	/// Stop all further execution
	HALT,

	/// Load a numeric value into a register
	LOAD {
		value:    f64,
		register: RegisterAddress,
	},

	/// Move a value from one register address to another.
	MOVE {
		source: 	 RegisterAddress,
		destination: RegisterAddress,
	},

	/// Copy a value from a register to memory or vice versa
	COPY {
		source:      RegisterAddress,
		destination: RegisterAddress,
	},

	/// Perform a calculation on two registers and write the result to a third
	CALCULATE {
		method: 	 CalculationMethod,
		operand1:    RegisterAddress,
		operand2:    RegisterAddress,
		destination: RegisterAddress,
	},

	// Compare two registers and write the result (`0` or `1`) into a third
	COMPARE {
		method: 	 ComparisonMethod,
		operand1:    RegisterAddress,
		operand2:    RegisterAddress,
		destination: RegisterAddress,
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

	/// Set off an interrupt, for example to print a character to standard output
	INTERRUPT {
		interrupt: Interrupt,
	},

	CALL {},
	RETURN {},
}
*/

#[derive(Debug, Clone, PartialEq)]
/// A signal indicating that an event needs immediate attention. This enumeration
/// contains the interrupt types supported by the virtual machine.
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
/// Basic arithmetic operations which act on two registers and write the result to a third.
pub enum CalculationMethod {
	ADD,
	SUBTRACT,
	MULTIPLY,
	DIVIDE,
	POWER,
	MODULO,
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
#[allow(non_camel_case_types)]
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

	SET_RANGE {
		address: u64,
		values: Vec<u8>,
	},

	UNSET {
		address: u64
	}
}