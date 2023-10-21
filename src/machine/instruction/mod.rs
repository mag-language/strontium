use num_derive::{FromPrimitive, ToPrimitive};

use crate::types::StrontiumError;
use super::RegisterValue;

mod executors;
pub use self::executors::*;

use super::Strontium;

pub trait Executor {
    fn execute(&self, machine: &mut Strontium) -> Result<bool, StrontiumError>;
}


/// A signal indicating that an event needs immediate attention. This enumeration
/// contains the interrupt types supported by the virtual machine.
#[derive(Debug, Clone, PartialEq)]
pub struct Interrupt {
	/// The address to print to or read from
	pub address: String,
	pub kind: InterruptKind,
}

#[derive(Debug, Clone, PartialEq, ToPrimitive, FromPrimitive)]
pub enum InterruptKind {
	Print = 200,
	Read = 201,
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

impl Into<Vec<u8>> for Instruction {
	fn into(self) -> Vec<u8> {
		let mut bytecode = vec![];

		match self {
			Instruction::HALT => {
				bytecode.push(0);
			},

			Instruction::LOAD { value, register } => {
				bytecode.push(1);
				bytecode.append(&mut value.into());
				bytecode.append(&mut register.into_bytes());
			},

			Instruction::MOVE { source, destination } => {
				bytecode.push(2);
				bytecode.append(&mut source.into_bytes());
				bytecode.append(&mut destination.into_bytes());
			},

			Instruction::COPY { source, destination } => {
				bytecode.push(3);
				bytecode.append(&mut source.into_bytes());
				bytecode.append(&mut destination.into_bytes());
			},

			Instruction::PUSH { value, destination } => {
				bytecode.push(12);
				bytecode.append(&mut value.into());
				bytecode.append(&mut destination.into_bytes());
			},

			Instruction::APPEND { value, destination } => {
				bytecode.push(13);
				bytecode.push(value.len() as u8);
				for val in value {
					bytecode.append(&mut val.into());
				}
				bytecode.append(&mut destination.into_bytes());
			},

			Instruction::CALCULATE { method, operand1, operand2, destination } => {
				bytecode.push(4);
				bytecode.push(method.into());
				bytecode.append(&mut operand1.into_bytes());
				bytecode.append(&mut operand2.into_bytes());
				bytecode.append(&mut destination.into_bytes());
			},

			Instruction::COMPARE { method, operand1, operand2, destination } => {
				bytecode.push(5);
				bytecode.push(method.into());
				bytecode.append(&mut operand1.into_bytes());
				bytecode.append(&mut operand2.into_bytes());
				bytecode.append(&mut destination.into_bytes());
			},

			Instruction::BITWISE { method } => {
				bytecode.push(6);
				match method {
					BitwiseMethod::AND { a, b, out, len } => {
						bytecode.push(0);
						bytecode.append(&mut a.into_bytes());
						bytecode.append(&mut b.into_bytes());
						bytecode.append(&mut out.into_bytes());
						bytecode.push(len as u8);
					},

					BitwiseMethod::OR { a, b, out, len } => {
						bytecode.push(1);
						bytecode.append(&mut a.into_bytes());
						bytecode.append(&mut b.into_bytes());
						bytecode.append(&mut out.into_bytes());
						bytecode.push(len as u8);
					},

					BitwiseMethod::XOR { a, b, out, len } => {
						bytecode.push(2);
						bytecode.append(&mut a.into_bytes());
						bytecode.append(&mut b.into_bytes());
						bytecode.append(&mut out.into_bytes());
						bytecode.push(len as u8);
					},

					BitwiseMethod::NOT { a, out, len } => {
						bytecode.push(3);
						bytecode.append(&mut a.into_bytes());
						bytecode.append(&mut out.into_bytes());
						bytecode.push(len as u8);
					},

					BitwiseMethod::LSH { a, out, amount, len } => {
						bytecode.push(4);
						bytecode.append(&mut a.into_bytes());
						bytecode.append(&mut out.into_bytes());
						bytecode.push(amount as u8);
						bytecode.push(len as u8);
					},

					BitwiseMethod::RSH { a, out, amount, len } => {
						bytecode.push(5);
						bytecode.append(&mut a.into_bytes());
						bytecode.append(&mut out.into_bytes());
						bytecode.push(amount as u8);
						bytecode.push(len as u8);
					},
				}
			},

			_ => unimplemented!("Opcode not implemented"),
		}

		bytecode
	}
}

/// Basic arithmetic operations which act on two registers and write the result to a third.
#[derive(Debug, Clone, PartialEq, ToPrimitive, FromPrimitive)]
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

#[derive(Debug, Clone, PartialEq, ToPrimitive, FromPrimitive)]
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

#[derive(Debug, Clone, PartialEq)]
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