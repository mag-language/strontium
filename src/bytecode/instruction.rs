use crate::types::Numeric;

/// Represents a callable machine instruction
#[derive(Debug, Clone, PartialEq)]
pub enum Instruction {
	/// Stop all further execution
	Halt,

	/// Load a numeric value into a register
	Load {
		value:    Numeric,
		register: usize,
	},

	/// Perform a calculation on two registers and write the result to a third
	Calculate {
		method: CalculationMethod,
		number_type: usize,
		a:   usize,
		b: 	 usize,
		out: usize,
	},

	// Compare two registers and write the result to a third
	Compare {
		method: ComparisonMethod,
		number_type: usize,
		a:   usize,
		b: 	 usize,
		out: usize, 
	},

	// Perform a bitwise operation on at least one range of bits
	Bitwise {
		method: BitwiseMethod
	},

	/// Set the program counter to the given value
	Jump {
		destination: usize,
	},

	/// Set the program counter to a value if the given bit is set
	JumpIf {
		destination: usize,
		conditional_bit: usize,
	},

	Interrupt {
		kind: InterruptKind,
	}
}

#[derive(Debug, Clone, PartialEq)]
pub enum InterruptKind {
	/// Print the byte which starts at the given address in memory
	Print {
		address: u32
	},

	/// Read a byte and write it to memory
	Read {
		address: u32
	}
}

#[derive(Debug, Clone, PartialEq)]
pub enum CalculationMethod {
	Add,
	Sub,
	Mul,
	Div,
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
	},

	UNSET {
		address: u64,
	},
}