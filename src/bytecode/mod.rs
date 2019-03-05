//! This module defines a bytecode format, which represents programs that can run on the virtual 
//! machine. Also included in this module is an image format, which enables a Poly environment
//! to be saved and restored at a later time.

mod instruction;

use crate::types::Numeric;
use crate::types::Numeric::*;
pub use self::instruction::Instruction;
pub use self::instruction::Instruction::*;
pub use self::instruction::{CalculationMethod, ComparisonMethod, BitwiseMethod, InterruptKind};

//use std::io::Cursor;
use byteorder::{LittleEndian, WriteBytesExt};



// The layout of the bytecode format looks like this:
//  ___________  ______________________
// |__header__| |____64-bit number_____|*
//  
//  ^~~ opcode      ^~~ float, int or uint
//      operands        zero or more of these



/// Convert an instruction to a byte vector
pub fn convert_instruction(instruction: Instruction) -> Vec<u8> {
	match instruction {
		Halt => {
			vec![1, 0, 0, 0]
		},

		Load { value, register } => {
			// Create the header
			let mut bytecode = vec![2, register as u8, 0, 0];

			// Append the value to the header
			match value {
				Int { value } => bytecode.write_i64::<LittleEndian>(value).unwrap(),
				UInt { value } => bytecode.write_u64::<LittleEndian>(value).unwrap(),
				Float { value } => bytecode.write_f64::<LittleEndian>(value).unwrap(),
			}

			bytecode
		},

		_ => vec![0, 0, 0, 0]
	}
}

#[cfg(test)]
mod tests {
	use super::*;

    #[test]
    fn convert_halt() {
        assert_eq!(
        	convert_instruction(Instruction::Halt), 
        	vec![1, 0, 0, 0]
        )
    }


    #[test]
    fn convert_load() {
        assert_eq!(
        	convert_instruction(Instruction::Load { value: Numeric::Int { value: 200 }, register: 7 }), 
        	vec![
        		// header
        		2, 7, 0, 0,

        		// i64
        		200, 0, 0, 0, 0, 0, 0, 0
        	]
        )
    }
}