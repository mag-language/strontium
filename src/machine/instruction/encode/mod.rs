use super::Instruction::{self, *};
use super::CalculationMethod::*;
use super::ComparisonMethod::*;
use super::MemoryMethod::*;
use crate::types::Location;

use byteorder::{LittleEndian, WriteBytesExt};
use std::mem;

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
					AND { .. } => accumulator.push(0x01),
					OR  { .. } => accumulator.push(0x02),
					XOR { .. } => accumulator.push(0x03),
					NOT { .. } => accumulator.push(0x04),
					LSH { .. } => accumulator.push(0x05),
					RSH { .. } => accumulator.push(0x06),
					GROW { .. } => accumulator.push(0x07),
					SHRINK { .. } => accumulator.push(0x08),
					SET { .. } => accumulator.push(0x09),
					UNSET { .. } => accumulator.push(0x0A),
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