use serde::{Serialize, Deserialize};

use crate::types::StrontiumError;
use crate::machine::opcode::Opcode;
use super::{Strontium, RegisterValue};

mod executors;
pub use self::executors::*;

pub trait Executor {
    fn execute(
		&self,
		machine: &mut Strontium,
		instruction: Instruction,
	) -> Result<bool, StrontiumError>;
}


/// A signal indicating that an event needs immediate attention. This enumeration
/// contains the interrupt types supported by the virtual machine.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Interrupt {
	/// The address to print to or read from
	pub address: String,
	pub kind: InterruptKind,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum InterruptKind {
	Print,
	Read,
}

impl Into<u8> for InterruptKind {
	fn into(self) -> u8 {
		match self {
			InterruptKind::Print => 0,
			InterruptKind::Read => 1,
		}
	}
}

/// Represents a callable machine instruction
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Instruction {
	/// Stop all further execution
	HALT,

	/// Load a numeric value into a register
	LOAD {
		value:    RegisterValue,
		register: String,
	},

	/// Move a value from one register address to another
	MOVE {
		source: 	 String,
		destination: String,
	},

	/// Copy a value from a register to memory or vice versa
	COPY {
		source:      String,
		destination: String,
	},

	/// Add a value to an array in a register
	PUSH {
		/// The value to be pushed
		value: RegisterValue,
		/// The name of the array register to append to
		destination: String,
	},

	/// Add a list of values to an array in a register
	APPEND {
		/// The values to append
		value: Vec<RegisterValue>,
		/// The name of the array register to append to
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

impl Into<Opcode> for Instruction {
	fn into(self) -> Opcode {
		match self {
			Instruction::HALT => Opcode::HALT,
			Instruction::LOAD { .. } => Opcode::LOAD,
			Instruction::MOVE { .. } => Opcode::MOVE,
			Instruction::COPY { .. } => Opcode::COPY,
			Instruction::CALCULATE { .. } => Opcode::CALCULATE,
			Instruction::COMPARE { .. } => Opcode::COMPARE,
			Instruction::BITWISE { .. } => Opcode::BITWISE,
			Instruction::JUMP { .. } => Opcode::JUMP,
			Instruction::JUMPC { .. } => Opcode::JUMPC,
			Instruction::INTERRUPT { .. } => Opcode::INTERRUPT,
			Instruction::CALL { .. } => Opcode::CALL,
			Instruction::RETURN { .. } => Opcode::RETURN,
			Instruction::PUSH { .. } => Opcode::PUSH,
			Instruction::APPEND { .. } => Opcode::APPEND,
		}
	}
}

/// Basic arithmetic operations which act on two registers and write the result to a third.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CalculationMethod {
	ADD,
	SUBTRACT,
	MULTIPLY,
	DIVIDE,
	POWER,
	MODULO,
}

impl Into<u8> for CalculationMethod {
	fn into(self) -> u8 {
		match self {
			CalculationMethod::ADD => 0,
			CalculationMethod::SUBTRACT => 1,
			CalculationMethod::MULTIPLY => 2,
			CalculationMethod::DIVIDE => 3,
			CalculationMethod::POWER => 4,
			CalculationMethod::MODULO => 5,
		}
	}
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ComparisonMethod {
	EQ,
	NEQ,
	GT,
	GTE,
	LT,
	LTE,
}

impl Into<u8> for ComparisonMethod {
	fn into(self) -> u8 {
		match self {
			ComparisonMethod::EQ => 0,
			ComparisonMethod::NEQ => 1,
			ComparisonMethod::GT => 2,
			ComparisonMethod::GTE => 3,
			ComparisonMethod::LT => 4,
			ComparisonMethod::LTE => 5,
		}
	}
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub enum BitwiseMethod {
	AND {
		a: String,
		b: String,
		out: String,
		len: usize,
	},

	OR {
		a: String,
		b: String,
		out: String,
		len: usize,
	},

	XOR {
		a: String,
		b: String,
		out: String,
		len: usize,
	},

	NOT {
		a: String,
		out: String,
		len: usize,
	},

	LSH {
		a: String,
		out: String,
		amount: usize,
		len: usize,
	},
	
	RSH {
		a: String,
		out: String,
		amount: usize,
		len: usize,
	},
}