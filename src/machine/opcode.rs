use self::Opcode::*;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Opcode {
	/// Stop all execution instantly.
	HALT,
	// Load a 64-bit floating-point value into a register.
	LOAD,
	/// Move a value from a register to a memory location or vice versa. The first 
	/// argument is the source, the second is the destination. Swap the arguments 
	/// to change the direction. The source will be cleared after the operation.
	/// 
	/// This opcode is followed by two 64-bit unsigned integers.
	MOVE,
	/// Copy a value from a register to a memory location or vice versa. 
	/// The source will be left untouched.
	/// 
	/// This opcode is followed by two 64-bit unsigned integers.
	COPY,
	/// Perform a calculation (`ADD`, `SUBTRACT`, `MULTIPLY`, `DIVIDE`) on two registers 
	/// and write the result to a third.
	/// 
	/// This opcode is followed by three 16-bit unsigned integers.
	CALCULATE,
	/// Perform a comparison (`EQ`, `NEQ`, `LT`, `LTE`, `GT`, `GTE`) on two registers 
	/// and write the result to a third.
	/// 
	/// This opcode is followed by three 16-bit unsigned integers.
	COMPARE,
	/// Perform a bitwise operation (`AND`, `OR`, `XOR`, `NOT`, `LSH`, `RSH`) on two or 
	/// three addresses.
	BITWISE,
	/// Set the program counter to a value from a location, using one of the 
	/// methods (`absolute`, `forward`, `backward`)
	JUMP,
	/// Set the program counter to a value from a location if the byte at a given 
	/// address in memory has the value of 1
	JUMPC,
	/// Emit an event that needs immediate attention (`READ`, `PRINT`)
	INTERRUPT,
	CALL,
	RETURN,
	PUSH,
	APPEND,
	/// An invalid opcode.
	ILLEGAL,
}

impl From<u8> for Opcode {
	fn from(byte: u8) -> Opcode {
		match byte {
			0 => HALT,
			1 => LOAD,
			2 => MOVE,
			3 => COPY,
			4 => CALCULATE,
			5 => COMPARE,
			6 => BITWISE,
			7 => JUMP,
			8 => JUMPC,
			9 => INTERRUPT,
			10 => CALL,
			11 => RETURN,
			12 => PUSH,
			13 => APPEND,

			_ => ILLEGAL,
		}
	}
}

impl Into<u8> for Opcode {
	fn into(self) -> u8 {
		match self {
			HALT => 0,
			LOAD => 1,
			MOVE => 2,
			COPY => 3,
			CALCULATE => 4,
			COMPARE => 5,
			BITWISE => 6,
			JUMP => 7,
			JUMPC => 8,
			INTERRUPT => 9,
			CALL => 10,
			RETURN => 11,
			PUSH => 12,
			APPEND => 13,

			ILLEGAL => 255,
		}
	}
}