//! This module contains the virtual machine which executes Strontium bytecode. The VM uses a set of typed 
//! registers to do number arithmetic, and a memory vector provides the storage space for anything else.

use crate::types::{MemoryAddress, Location};

pub mod memory;
pub mod instruction;

use self::memory::Memory;
use self::instruction::{
	Instruction,
	Instruction::*,
	MemoryMethod, 
	ComparisonMethod, 
	CalculationMethod, 
	Interrupt,
};

/*/// A set of signed and unsigned integers and floating point values
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
}*/

#[derive(Debug, PartialEq)]
pub enum StrontiumError {
	Overflow,
	DivisionByZero,
	// Raised when the memory size is shrinked to a number lower than zero
	InvalidMemorySize,
	Other(String),
}

const NUM_REGISTERS: usize = 64;


pub struct Strontium {
	/// Holds 64 64-bit floating point values
	pub registers: [f64; NUM_REGISTERS],
	/// Models memory as a vector of bits. This structure holds program-related data,
	/// and will probably be replaced by a more complex, paged structure later on
	pub memory:    Memory,
	/// Contains the parsed bytecode
	pub program:   Vec<Instruction>,
	/// Our current position in the program
	pub index:     usize,
	/// Contains references for function arguments and return values
	call_stack: Vec<MemoryAddress>,
	/// References the bytecode position the VM should go to after returning from a
	/// function
	last_known_point: usize,
}

impl Strontium {
	/// Create a new instance of the virtual machine
	pub fn new() -> Self {
		Self {
			registers:  [0.0; NUM_REGISTERS],
			memory:     Memory::new(),
			program:    vec![],
			index:      0,
			call_stack: vec![],
			last_known_point: 0,
		}
	}

	/// Append an instruction to the program vector
	pub fn add_instruction(&mut self, instruction: Instruction) {
		self.program.push(instruction);
	}

	/// Get a full slice of the memory vector
	pub fn dump_memory(&self) -> &[u8] {
		&self.memory.data[..]
	}
 
	/// Execute a single instruction
	pub fn execute(&mut self) -> Result<bool, StrontiumError> {
		let instruction = self.peek();

		match instruction {
			HALT => {
				Ok(
					self.halt()
				)
			},

			LOAD { value, register } => {
				Ok(
					self.load(value, register as usize)?
				)
			},

			MOVE { source, destination } => {
				Ok(
					self.move_value(source, destination)
				)
			},

			COPY { source, destination } => {
				Ok(
					self.copy_value(source, destination)
				)
			},

			CALCULATE { method, operand1, operand2, destination } => {
				Ok(
					self.calculate(method.clone(), operand1 as usize, operand2 as usize, destination as usize)?
				)
			},
			
			COMPARE { method, operand1, operand2, destination } => {
				Ok(
					self.compare(method.clone(), operand1 as usize, operand2 as usize, destination as usize)?
				)
			},

			MEMORY { method } => {
				Ok(
					self.bitwise(method.clone())?
				)
			},

			JUMP { destination } => {
				Ok(
					self.jump(destination)
				)
			},

			JUMPC { destination, conditional_address } => {
				Ok(
					self.jumpc(destination, conditional_address)
				)
			},

			INTERRUPT { interrupt } => {
				Ok(
					self.interrupt(interrupt.clone())?
				)
			},

			PUSH { value } => {
				self.call_stack.push(value);

				Ok(true)
			},

			POP => {
				self.call_stack.pop();

				Ok(true)
			},

			CALL { function_pointer, arguments } => {
				// Save the position we're gonna return to later
				self.last_known_point = self.index;

				// Push function arguments onto the stack
				for arg in arguments {
					self.call_stack.push(arg);
				}

				// Set the program counter to the start of the called function
				self.index = function_pointer as usize;

				Ok(true)
			},
				

			_ => unimplemented!(),
		}
	}

	fn halt(&mut self) -> bool {
		false
	}

	fn load(&mut self, value: f64, register: usize) -> Result<bool, StrontiumError> {
		if register <= NUM_REGISTERS {
			self.registers[register] = value;
		} else {
			return Err(StrontiumError::Overflow);
		}
		
		self.advance();

		Ok(true)
	}

	fn move_value(&mut self, source: Location, destination: Location) -> bool {
		println!("The MOVE instruction has not yet been implemented");
		true
	}

	fn copy_value(&mut self, _source: Location, _destination: Location) -> bool {
		println!("The COPY instruction has not yet been implemented");
		true
	}

	fn jump(&mut self, destination: usize) -> bool {
		self.index = destination;

		true
	}

	fn jumpc(&mut self, destination: usize, pointer: MemoryAddress) -> bool {
		if self.memory.data[pointer as usize] == 1 {
			self.index = destination;
		}

		true
	}

	fn bitwise(&mut self, method: MemoryMethod) -> Result<bool, StrontiumError> {
		self.memory.compute(method)?;

		Ok(true)
	}

	fn calculate(&mut self, method: CalculationMethod, a: usize, b: usize, destination: usize) -> Result<bool, StrontiumError> {
		match method {
			CalculationMethod::ADD 		=> self.registers[destination] = self.registers[a] + self.registers[b],
			CalculationMethod::SUBTRACT => self.registers[destination] = self.registers[a] - self.registers[b],
			CalculationMethod::MULTIPLY => self.registers[destination] = self.registers[a] * self.registers[b],
			CalculationMethod::DIVIDE 	=> self.registers[destination] = self.registers[a] / self.registers[b],
		}

		self.advance();

		Ok(true)
	}

	fn compare(&mut self, method: ComparisonMethod, a: usize, b: usize, destination: usize) -> Result<bool, StrontiumError> {
		match method {
			ComparisonMethod::EQ  => self.memory.set(destination, (self.registers[a] == self.registers[b]) as u8),
			ComparisonMethod::NEQ => self.memory.set(destination, (self.registers[a] != self.registers[b]) as u8),
			ComparisonMethod::GT  => self.memory.set(destination, (self.registers[a] >  self.registers[b]) as u8),
			ComparisonMethod::GTE => self.memory.set(destination, (self.registers[a] >= self.registers[b]) as u8),
			ComparisonMethod::LT  => self.memory.set(destination, (self.registers[a] <  self.registers[b]) as u8),
			ComparisonMethod::LTE => self.memory.set(destination, (self.registers[a] <= self.registers[b]) as u8),
		}

		self.advance();

		Ok(true)
	}

	fn interrupt(&mut self, kind: Interrupt) -> Result<bool, StrontiumError> {
		Ok(true)
	}

	fn peek(&self) -> Instruction {
		self.program[self.index].clone()
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
    fn halt_instruction() {
 		let mut machine = Strontium::new();

 		machine.add_instruction(Instruction::HALT);

 		assert_eq!(machine.execute(), Ok(false));
    }

    #[test]
    fn load_instruction() {
 		let mut machine = Strontium::new();

 		machine.add_instruction(Instruction::LOAD { value: 1332.5, register: 5 });
 		machine.execute();

 		assert_eq!(machine.registers[5], 1332.5);
    }

    #[test]
    fn add_instruction() {
 		let mut machine = Strontium::new();

 		machine.add_instruction(Instruction::LOAD { value: 44.7, register: 1 });
 		machine.add_instruction(Instruction::LOAD { value: 36.8, register: 2 });

 		machine.add_instruction(Instruction::CALCULATE { 
 			method: CalculationMethod::ADD,
 			operand1: 1,
 			operand2: 2,
 			destination: 3,
 		});

 		machine.execute();
 		machine.execute();
 		machine.execute();

 		assert_eq!(machine.registers[3], 44.7 + 36.8);
    }

    #[test]
    fn subtract_instruction() {
 		let mut machine = Strontium::new();

 		machine.add_instruction(Instruction::LOAD { value: 3452.37, register: 1 });
 		machine.add_instruction(Instruction::LOAD { value: 3685.8148, register: 2 });

 		machine.add_instruction(Instruction::CALCULATE { 
 			method: CalculationMethod::SUBTRACT,
 			operand1: 1,
 			operand2: 2,
 			destination: 3,
 		});
 		
 		machine.execute();
 		machine.execute();
 		machine.execute();

 		assert_eq!(machine.registers[3], 3452.37 - 3685.8148);
    }

    #[test]
    fn multiply_instruction() {
 		let mut machine = Strontium::new();

 		machine.add_instruction(Instruction::LOAD { value: 3.642, register: 1 });
 		machine.add_instruction(Instruction::LOAD { value: 2.46682, register: 2 });

 		machine.add_instruction(Instruction::CALCULATE { 
 			method: CalculationMethod::MULTIPLY,
 			operand1: 1,
 			operand2: 2,
 			destination: 3,
 		});
 		
 		machine.execute();
 		machine.execute();
 		machine.execute();

 		assert_eq!(machine.registers[3], 3.642 * 2.46682);
    }

    #[test]
    fn divide_instruction() {
 		let mut machine = Strontium::new();

 		machine.add_instruction(Instruction::LOAD { value: 12.534, register: 1 });
 		machine.add_instruction(Instruction::LOAD { value: 8.388475294, register: 2 });

 		machine.add_instruction(Instruction::CALCULATE { 
 			method: CalculationMethod::DIVIDE,
 			operand1: 1,
 			operand2: 2,
 			destination: 3,
 		});
 		
 		machine.execute();
 		machine.execute();
 		machine.execute();

 		assert_eq!(machine.registers[3], 12.534 / 8.388475294);
    }

    #[test]
    fn and_instruction() {
 		let mut machine = Strontium::new();

 		machine.add_instruction(Instruction::MEMORY {
 			method: MemoryMethod::GROW { amount: 3 }
 		});

 		machine.add_instruction(Instruction::MEMORY {
 			method: MemoryMethod::SET { value: 3, address: 0 }
 		});

 		machine.add_instruction(Instruction::MEMORY {
 			method: MemoryMethod::SET { value: 7, address: 1 }
 		});

 		machine.add_instruction(Instruction::MEMORY { 
 			method: MemoryMethod::AND {
 				a: 0,
 				b: 1,
 				out: 2,
 				len: 1,
 			}
 		});
 		
 		machine.execute();
 		machine.execute();
 		machine.execute();
 		machine.execute();

 		assert_eq!(machine.memory.data[2], 3 & 7);
    }
}

