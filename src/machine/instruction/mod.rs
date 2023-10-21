use crate::types::StrontiumError;
use super::RegisterValue;

mod executors;
pub use self::executors::*;

use super::Strontium;

pub trait Executor {
    fn execute(&self, machine: &mut Strontium) -> Result<bool, StrontiumError>;
}

#[derive(Debug, Clone, PartialEq)]
/// A signal indicating that an event needs immediate attention. This enumeration
/// contains the interrupt types supported by the virtual machine.
pub enum Interrupt {
	Print {
		address: String,
	},
	Read {
		address: String,
	},
}

/// Represents a callable machine instruction
#[derive(Debug, Clone, PartialEq)]
pub enum Instruction {
	/// Stop all further execution
	HALT,

	/// Load a numeric value into a register
	LOAD {
		value:    RegisterValue,
		register: String,
	},

	/// Move a value from one register address to another.
	MOVE {
		source: 	 String,
		destination: String,
	},

	/// Copy a value from a register to memory or vice versa
	COPY {
		source:      String,
		destination: String,
	},

	/// Perform a calculation on two registers and write the result to a third
	CALCULATE {
		// The type of calculation to perform
		method:   	 CalculationMethod,
		// Left side operand as a named register address
		operand1: 	 String,
		// Right side operand as a named register address
		operand2: 	 String,
		// Output register
		destination: String,
	},

	// Compare two registers and write the result (`0` or `1`) into a third
	COMPARE {
		method: 	 ComparisonMethod,
		operand1:    String,
		operand2:    String,
		destination: String,
	},

	// Perform a memory operation (`and`, `or`, `xor`, `not`, `lsh`, `rsh`, `grow`, `shrink`, `set`, or `unset`)
	BITWISE {
		method: BitwiseMethod
	},

	/// Set the program counter to the given value
	JUMP {
		destination: usize,
	},

	/// Set the program counter to a value if the given byte has the value of `1`
	JUMPC {
		destination: usize,
		conditional_address: String,
	},

	/// Set off an interrupt, for example to print a character to standard output
	INTERRUPT {
		interrupt: Interrupt,
	},

	CALL {},
	RETURN {},
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
pub enum BitwiseMethod {
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