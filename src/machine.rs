//! This module contains the virtual machine that executes Strontium bytecode. The VM uses a set of typed 
//! registers to do number arithmetic, and a memory vector provides the storage space for anything
//! else.

use crate::types::{Numeric};

use super::bytecode::{
	Instruction, 
	BitwiseMethod, 
	ComparisonMethod, 
	CalculationMethod, 
	InterruptKind,
};

use super::bytecode::Instruction::*;
use super::memory::Memory;

/// A set of signed and unsigned integers and floating point values
#[derive(Debug)]
pub struct Registers {
	pub int:   [i64; 32],
	pub uint:  [u64; 32],
	pub float: [f64; 32],
}

impl Registers {
	pub fn new() -> Self {
		Self {
			int:   [0; 32],
			uint:  [0; 32],
			float: [0.0; 32],
		}
	}
}

pub struct Strontium {
	/// Holds a set of typed numeric values
	pub registers: Registers,
	/// Models memory as a vector of bits. This structure holds program-related data,
	/// and will probably be replaced by a more complex, paged structure later on.
	pub memory:    Memory,
	/// Contains the parsed bytecode
	pub program:   Vec<Instruction>,
	/// Our current position in the program
	pub index:     usize,
}

impl Strontium {
	/// Create a new instance of the virtual machine
	pub fn new() -> Self {
		Self {
			registers: Registers::new(),
			memory:    Memory::new(),
			program:   vec![],
			index:     0,
		}
	}

	/// Append instructions to the program vector
	pub fn add_instruction(&mut self, instruction: Instruction) {
		self.program.push(instruction);
	}

	/// Get a full slice of the memory vector
	pub fn dump_memory(&self) -> &[u8] {
		&self.memory.data[..]
	}
 
	/// Execute a single instruction
	pub fn execute(&mut self) -> Result<bool, String> {
		let instruction = self.peek();

		match instruction {
			Halt => {
				Ok(self.halt())
			},

			Load { value, register } => {
				Ok(self.load(*value, *register))
			},

			Calculate { method, number_type, a, b, out } => {
				Ok(self.calculate(method.clone(), *number_type, *a, *b, *out)?)
			},
			
			Compare { method, number_type, a, b, out } => {
				Ok(self.compare(method.clone(), *number_type, *a, *b, *out)?)
			},

			Bitwise { method } => {
				Ok(self.bitwise(method.clone())?)
			},

			Jump { destination } => {
				Ok(self.jump(*destination))
			},

			JumpIf { destination, conditional_bit } => {
				Ok(self.jump_if(*destination, *conditional_bit))
			},

			Interrupt { kind } => {
				Ok(self.interrupt(kind.clone())?)
			}
		}
	}

	fn halt(&mut self) -> bool {
		false
	}

	fn load(&mut self, number: Numeric, register: usize) -> bool {
		// because of the static check done before execution it is safe not
		// to check for a out-of-range index

		match number {
			Numeric::Int { value }   => self.registers.int[register] = value,
			Numeric::UInt { value }  => self.registers.uint[register] = value,
			Numeric::Float { value } => self.registers.float[register] = value,
		}

		self.advance();

		true
	}

	fn jump(&mut self, destination: usize) -> bool {
		self.index = destination;

		true
	}

	fn jump_if(&mut self, destination: usize, pointer: usize) -> bool {
		if self.memory.data[pointer] == 1 {
			self.index = destination;
		}

		true
	}

	fn bitwise(&mut self, method: BitwiseMethod) -> Result<bool, String> {
		self.memory.compute(method)?;

		Ok(true)
	}

	fn calculate(&mut self, method: CalculationMethod, number_type: usize, a: usize, b: usize, out: usize) -> Result<bool, String> {
		match number_type {
			// int 
			0 => {
				match method {
					CalculationMethod::Add => self.registers.int[out] = self.registers.int[a] + self.registers.int[b],
					CalculationMethod::Sub => self.registers.int[out] = self.registers.int[a] - self.registers.int[b],
					CalculationMethod::Mul => self.registers.int[out] = self.registers.int[a] * self.registers.int[b],
					CalculationMethod::Div => self.registers.int[out] = self.registers.int[a] / self.registers.int[b],
				}
			},

			// uint
			1 => {
				match method {
					CalculationMethod::Add => self.registers.uint[out] = self.registers.uint[a] + self.registers.uint[b],
					CalculationMethod::Sub => self.registers.uint[out] = self.registers.uint[a] - self.registers.uint[b],
					CalculationMethod::Mul => self.registers.uint[out] = self.registers.uint[a] * self.registers.uint[b],
					CalculationMethod::Div => self.registers.uint[out] = self.registers.uint[a] / self.registers.uint[b],
				}
			},

			// float
			2 => {
				match method {
					CalculationMethod::Add => self.registers.float[out] = self.registers.float[a] + self.registers.float[b],
					CalculationMethod::Sub => self.registers.float[out] = self.registers.float[a] - self.registers.float[b],
					CalculationMethod::Mul => self.registers.float[out] = self.registers.float[a] * self.registers.float[b],
					CalculationMethod::Div => self.registers.float[out] = self.registers.float[a] / self.registers.float[b],
				}
			},

			_ => unimplemented!()
		}

		self.advance();

		Ok(true)
	}

	fn compare(&mut self, method: ComparisonMethod, number_type: usize, a: usize, b: usize, out: usize) -> Result<bool, String> {
		match number_type {
			// int 
			0 => {
				match method {
					ComparisonMethod::EQ  => self.memory.set(out, (self.registers.int[a] == self.registers.int[b]) as u8),
					ComparisonMethod::NEQ => self.memory.set(out, (self.registers.int[a] != self.registers.int[b]) as u8),
					ComparisonMethod::GT  => self.memory.set(out, (self.registers.int[a] >  self.registers.int[b]) as u8),
					ComparisonMethod::GTE => self.memory.set(out, (self.registers.int[a] >= self.registers.int[b]) as u8),
					ComparisonMethod::LT  => self.memory.set(out, (self.registers.int[a] <  self.registers.int[b]) as u8),
					ComparisonMethod::LTE => self.memory.set(out, (self.registers.int[a] <= self.registers.int[b]) as u8),
				}
			},

			// uint
			1 => {
				match method {
					ComparisonMethod::EQ  => self.memory.set(out, (self.registers.uint[a] == self.registers.uint[b]) as u8),
					ComparisonMethod::NEQ => self.memory.set(out, (self.registers.uint[a] != self.registers.uint[b]) as u8),
					ComparisonMethod::GT  => self.memory.set(out, (self.registers.uint[a] >  self.registers.uint[b]) as u8),
					ComparisonMethod::GTE => self.memory.set(out, (self.registers.uint[a] >= self.registers.uint[b]) as u8),
					ComparisonMethod::LT  => self.memory.set(out, (self.registers.uint[a] <  self.registers.uint[b]) as u8),
					ComparisonMethod::LTE => self.memory.set(out, (self.registers.uint[a] <= self.registers.uint[b]) as u8),
				}
			},

			// float
			2 => {
				match method {
					ComparisonMethod::EQ  => self.memory.set(out, (self.registers.float[a] == self.registers.float[b]) as u8),
					ComparisonMethod::NEQ => self.memory.set(out, (self.registers.float[a] != self.registers.float[b]) as u8),
					ComparisonMethod::GT  => self.memory.set(out, (self.registers.float[a] >  self.registers.float[b]) as u8),
					ComparisonMethod::GTE => self.memory.set(out, (self.registers.float[a] >= self.registers.float[b]) as u8),
					ComparisonMethod::LT  => self.memory.set(out, (self.registers.float[a] <  self.registers.float[b]) as u8),
					ComparisonMethod::LTE => self.memory.set(out, (self.registers.float[a] <= self.registers.float[b]) as u8),
				}
			},

			_ => unimplemented!()
		}

		self.advance();

		Ok(true)
	}

	fn interrupt(&mut self, kind: InterruptKind) -> Result<bool, String> {
		Ok(true)
	}

	fn peek(&self) -> &Instruction {
		&self.program[self.index]
	}

	fn advance(&mut self) -> bool {
		if self.index + 1 < self.program.len() {
			self.index += 1;
			true
		} else {
			false
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

    #[test]
    fn execute_halt() {
    	let mut machine = Strontium::new();
    	machine.add_instruction(Halt);

    	assert_eq!(machine.execute(), Ok(false))
    }

    
    #[test]
    fn execute_load() {
    	let mut machine = Strontium::new();
    	machine.add_instruction(Load { value: Numeric::UInt { value: 2000 }, register: 0 });
    	machine.execute().unwrap();

    	assert_eq!(machine.registers.uint[0], 2000)


    }

    #[test]
    fn execute_add() {
    	let mut machine = Strontium::new();
    	machine.add_instruction(Load { value: Numeric::Float { value: 20.2 }, register: 0 });
    	machine.add_instruction(Load { value: Numeric::Float { value: 12.7 }, register: 1 });
    	machine.add_instruction(Calculate { 
    		method: CalculationMethod::Add,
    		number_type: 2,
    		a: 0,
    		b: 1,
    		out: 2,
    	});

    	machine.execute().unwrap();
    	machine.execute().unwrap();
    	machine.execute().unwrap();

    	assert_eq!(machine.registers.float[2], 32.9);
    }

    #[test]
    fn execute_sub() {
    	let mut machine = Strontium::new();
    	machine.add_instruction(Load { value: Numeric::Int { value: 10 }, register: 0 });
    	machine.add_instruction(Load { value: Numeric::Int { value: 7 }, register: 1 });
    	machine.add_instruction(Calculate { 
    		method: CalculationMethod::Sub,
    		number_type: 0,
    		a: 0,
    		b: 1,
    		out: 2,
    	});

    	machine.execute().unwrap();
    	machine.execute().unwrap();
    	machine.execute().unwrap();

    	assert_eq!(machine.registers.int[2], 3);
    }


    #[test]
    fn execute_mul() {
    	let mut machine = Strontium::new();
    	machine.add_instruction(Load { value: Numeric::Float { value: 2.2 }, register: 0 });
    	machine.add_instruction(Load { value: Numeric::Float { value: 5.0 }, register: 1 });
    	machine.add_instruction(Calculate { 
    		method: CalculationMethod::Mul,
    		number_type: 2,
    		a: 0,
    		b: 1,
    		out: 2,
    	});

    	machine.execute().unwrap();
    	machine.execute().unwrap();
    	machine.execute().unwrap();

    	assert_eq!(machine.registers.float[2], 11.0);
    }

    #[test]
    fn execute_div() {
    	let mut machine = Strontium::new();
    	machine.add_instruction(Load { value: Numeric::UInt { value: 20 }, register: 0 });
    	machine.add_instruction(Load { value: Numeric::UInt { value: 3 }, register: 1 });
    	machine.add_instruction(Calculate { 
    		method: CalculationMethod::Div,
    		number_type: 1,
    		a: 0,
    		b: 1,
    		out: 2,
    	});

    	machine.execute().unwrap();
    	machine.execute().unwrap();
    	machine.execute().unwrap();

    	assert_eq!(machine.registers.uint[2], 6);
    }

    #[test]
    fn execute_jump() {
    	let mut machine = Strontium::new();
    	machine.add_instruction(Load { value: Numeric::Int { value: 10 }, register: 0 });
    	machine.add_instruction(Load { value: Numeric::Int { value: 7 }, register: 1 });
    	machine.add_instruction(Calculate { 
    		method: CalculationMethod::Sub,
    		number_type: 0,
    		a: 0,
    		b: 1,
    		out: 2,
    	});

    	machine.execute().unwrap();
    	machine.execute().unwrap();
    	machine.execute().unwrap();

    	assert_eq!(machine.registers.int[2], 3);
    }
}

