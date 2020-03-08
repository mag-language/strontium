use super::Instruction::{self, *};
use super::CalculationMethod::*;
use super::ComparisonMethod::*;
use super::MemoryMethod::*;
use crate::types::Location;

use byteorder::{LittleEndian, WriteBytesExt};

pub fn encode(header: Header, instructions: Vec<Instruction>) -> Vec<u8> {
	let mut accumulator = vec![];

	accumulator.append(&mut header.into());

	for instruction in instructions {
		accumulator.append(&mut instruction.into());
	}

	accumulator
}

pub struct Header {
	/// The version(s) of Strontium compatible with this program.
	pub version: String,
	/// Because the virtual machine doesn't have a type system,
	/// a constant is simply an array of bytes.
	pub constants: Vec<Vec<u8>>, 
}

impl Into<Vec<u8>> for Header {
	fn into(self) -> Vec<u8> {
		let mut accumulator = vec![];

		// Append the magic "strontium" string to the start of the executable.
		accumulator.append(&mut String::from("strontium").into_bytes());

		// Push the total number of constants.
		accumulator.push(self.constants.len() as u8);

		// Now, for each constant, write its length and the value into the vector.
		for mut constant in self.constants {
			accumulator.push(constant.len() as u8);
			accumulator.append(&mut constant);
		}

		// Return the encoded header.
		accumulator
	}
}

impl Into<Vec<u8>> for Instruction {
	fn into(self) -> Vec<u8> {
		match self {
			HALT => return vec![01, 00, 00, 00],

			LOAD { value, register } => {
				let mut accumulator = vec![];

				accumulator.push(0x02);

				accumulator
        			.write_f64::<LittleEndian>(value)
        			.expect("Unable to write value into accumulator");
        		
        		accumulator.push(register as u8);

				return accumulator
			},

			MOVE { source, destination } => {
				let mut accumulator = vec![];

				accumulator.push(0x03);

				match source {
					Location::Memory(address) => {
						accumulator
        					.write_u64::<LittleEndian>(address)
        					.expect("Unable to write source address into accumulator");
					},

					Location::Register(address) => {
						accumulator.push(address);
						accumulator.append(&mut vec![0u8; 7]);
					}
				}

				match destination {
					Location::Memory(address) => {
						accumulator
        					.write_u64::<LittleEndian>(address)
        					.expect("Unable to write destination address into accumulator");
					},

					Location::Register(address) => {
						accumulator.push(address);
						accumulator.append(&mut vec![0u8; 7]);
					}
				}

				return accumulator
			},

			COPY { source, destination } => {
				let mut accumulator = vec![];

				accumulator.push(0x04);

				match source {
					Location::Memory(address) => {
						accumulator
        					.write_u64::<LittleEndian>(address)
        					.expect("Unable to write source address into accumulator");
					},

					Location::Register(address) => {
						accumulator.push(address);
						accumulator.append(&mut vec![0u8; 7]);
					}
				}

				match destination {
					Location::Memory(address) => {
						accumulator
        					.write_u64::<LittleEndian>(address)
        					.expect("Unable to write destination address into accumulator");
					},

					Location::Register(address) => {
						accumulator.push(address);
						accumulator.append(&mut vec![0u8; 7]);
					}
				}

				return accumulator
			},

			CALCULATE { method, operand1, operand2, destination } => {
				let mut accumulator = vec![];

				accumulator.push(0x05);

				match method {
					ADD 	 => accumulator.push(0x01),
					SUBTRACT => accumulator.push(0x02),
					MULTIPLY => accumulator.push(0x03),
					DIVIDE   => accumulator.push(0x04),
					POWER    => accumulator.push(0x05),
					MODULO   => accumulator.push(0x06),
				}

				accumulator.append(&mut vec![operand1, operand2, destination]);

				return accumulator
			},

			COMPARE { method, operand1, operand2, destination } => {
				let mut accumulator = vec![];

				accumulator.push(0x06);

				match method {
					EQ 	=> accumulator.push(0x01),
					NEQ => accumulator.push(0x02),
					GT  => accumulator.push(0x03),
					GTE => accumulator.push(0x04),
					LT  => accumulator.push(0x05),
					LTE => accumulator.push(0x06),
				}

				accumulator.append(&mut vec![operand1, operand2]);

				accumulator
					.write_u64::<LittleEndian>(destination)
					.expect("Unable to write destination address into accumulator");

				return accumulator
			},

			MEMORY { method } => {
				let mut accumulator = vec![];

				accumulator.push(0x07);

				match method {
					AND { .. } 	  => accumulator.push(0x01),
					OR  { .. } 	  => accumulator.push(0x02),
					XOR { .. } 	  => accumulator.push(0x03),
					NOT { .. } 	  => accumulator.push(0x04),
					LSH { .. } 	  => accumulator.push(0x05),
					RSH { .. } 	  => accumulator.push(0x06),
					GROW { .. }   	 => accumulator.push(0x07),
					SHRINK { .. } 	 => accumulator.push(0x08),
					SET { .. } 	  	 => accumulator.push(0x09),
					SET_RANGE { .. } => accumulator.push(0x0A),
					UNSET { .. }     => accumulator.push(0x0B),
				}

				match method {
					AND { a, b, out, len } | 
					OR  { a, b, out, len } |
					XOR { a, b, out, len }  => {
						accumulator
        					.write_u64::<LittleEndian>(a)
        					.expect("Unable to write memory address into accumulator");

        				accumulator
        					.write_u64::<LittleEndian>(b)
        					.expect("Unable to write memory address into accumulator");

        				accumulator
        					.write_u64::<LittleEndian>(out)
        					.expect("Unable to write memory address into accumulator");

        				accumulator
        					.write_u64::<LittleEndian>(len as u64)
        					.expect("Unable to write range length into accumulator");
					},

					NOT { a, out, len } => {
						accumulator
        					.write_u64::<LittleEndian>(a)
        					.expect("Unable to write memory address into accumulator");

        				accumulator
        					.write_u64::<LittleEndian>(out)
        					.expect("Unable to write memory address into accumulator");

        				accumulator
        					.write_u64::<LittleEndian>(len as u64)
        					.expect("Unable to write range length into accumulator");
					},

					LSH { a, out, amount, len } | RSH { a, out, amount, len } => {
						accumulator
        					.write_u64::<LittleEndian>(a)
        					.expect("Unable to write memory address into accumulator");

        				accumulator
        					.write_u64::<LittleEndian>(out)
        					.expect("Unable to write memory address into accumulator");

        				accumulator
        					.write_u64::<LittleEndian>(amount as u64)
        					.expect("Unable to write memory address into accumulator");

        				accumulator
        					.write_u64::<LittleEndian>(len as u64)
        					.expect("Unable to write range length into accumulator");
					},

					GROW { amount } => {
						accumulator
        					.write_u64::<LittleEndian>(amount)
        					.expect("Unable to write grow amount into accumulator");
					},

					SHRINK { amount } => {
						accumulator
        					.write_u64::<LittleEndian>(amount)
        					.expect("Unable to write shrink amount into accumulator");
					},

					SET { address, value } => {
						accumulator
        					.write_u64::<LittleEndian>(address)
        					.expect("Unable to write memory address into accumulator");

        				accumulator.push(value)
					},

					SET_RANGE { address, values } => {
						accumulator
        					.write_u64::<LittleEndian>(address)
        					.expect("Unable to write memory address into accumulator");
        				
						accumulator
        					.write_u32::<LittleEndian>(values.len() as u32)
        					.expect("Unable to write range length into accumulator");

        				for value in values {
        					accumulator.push(value);
        				}
					},

					UNSET { address } => {
						accumulator
        					.write_u64::<LittleEndian>(address)
        					.expect("Unable to write memory address into accumulator");
					},
				}

				return accumulator
			},

			_ => unimplemented!(),
		}
	}
}