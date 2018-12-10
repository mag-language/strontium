use self::Instruction::*;

#[derive(Debug, PartialEq)]
pub enum Instruction {
	/// Stop executing bytecode
	HALT,
	/// Set the bit(s) at the given address(es) to 1
	SET,
	/// Set the bit(s) at the given address(es) to 0
	UNSET,
	/// Grow the virtual memory by (n) bits
	ALLOCATE,
	/// Shrink the virtual memory by (n) bits
	DEALLOCATE,
	/// Perform a bitwise AND operation on two bits or two ranges
	AND,
	/// Perform a bitwise OR operation on two bits or two ranges
	OR,	
	/// Perform a bitwise exclusive OR operation on two bits or two ranges
	XOR,
	/// Perform a bitwise negation on two bits or two ranges
	NOT,
	/// Perform an arithmetic left shift on two given ranges
	LSHIFT,
	/// Perform an arithmetic right shift on two given ranges
	RSHIFT,
	/// Set the instruction pointer to the given value
	JUMP,
	/// Set the instruction pointer to a value if the bit at a given address is set
	JUMPIF,
	/// Invalid instruction
	ILLEGAL
}

impl From<u8> for Instruction {
	fn from(int: u8) -> Self {
		match int {
			0  => HALT,
			1  => SET,
			2  => UNSET,
			3  => ALLOCATE,
			4  => DEALLOCATE,
			5  => AND,
			6  => OR,
			7  => XOR,
			8  => NOT,
			9  => LSHIFT,
			10 => RSHIFT,
			11 => JUMP,
			12 => JUMPIF,
			_  => ILLEGAL,
		}
	}
}

impl Into<u8> for Instruction {
	fn into(self) -> u8 {
		match self {
			HALT  	   => 0,
			SET   	   => 1,
			UNSET  	   => 2,
			ALLOCATE   => 3,
			DEALLOCATE => 4,
			AND  	   => 5,
			OR  	   => 6,
			XOR  	   => 7,
			NOT 	   => 8,
			LSHIFT     => 9,
			RSHIFT 	   => 10,
			JUMP 	   => 11,
			JUMPIF 	   => 12,
			ILLEGAL    => 0,
		}
	}
}