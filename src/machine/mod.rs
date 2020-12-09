//! This module contains the virtual machine which executes Strontium bytecode. The VM uses a set of typed 
//! registers to do number arithmetic, a memory vector provides the storage space for anything else.

use crate::types::{MemoryAddress, Location};

pub mod memory;
pub mod instruction;
pub mod opcode;

use self::memory::Memory;

use self::opcode::Opcode;
use self::opcode::Opcode::*;

use self::instruction::{
	Instruction,
	MemoryMethod, 
	ComparisonMethod, 
	CalculationMethod, 
	Interrupt,
};

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};

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
	/// A division by zero has occured.
	DivisionByZero,
	/// An invalid memory or register address has been accessed.
	OutOfBounds,
	/// The machine encountered an invalid operation code.
	IllegalOpcode(u8),
	UnexpectedEof,
	Other(String),
}

const NUM_REGISTERS: usize = 32;


pub struct Strontium {
	/// Holds 64 64-bit floating point values
	pub registers: [u64; NUM_REGISTERS],
	/// Models memory as a vector of bytes. This structure holds the 
	/// program and related data.
	pub memory:    Memory,
	/// An instruction pointer that indicates the current position in the program memory.
	pub ip:        usize,
	/// Contains references for function arguments and return values
	pub call_stack:    Vec<MemoryAddress>,
	should_continue: bool,
}

impl Strontium {
	/// Create a new instance of the virtual machine
	pub fn new() -> Self {
		Self {
			registers:  [0; NUM_REGISTERS],
			memory:     Memory::new(),
			ip:      	0,
			call_stack: vec![],
			should_continue: true,
		}
	}

	/// Get a full slice of the memory vector
	pub fn dump_memory(&self) -> &[u8] {
		&self.memory.data[..]
	}

	pub fn push_bytecode(&mut self, bytes: &[u8]) {
		if (self.ip + bytes.len() <= self.memory.data.len()) {
			// We have enough space in memory to add the new code.
			self.memory.set_range(self.ip, bytes.to_vec());
		} else {
			// We calculate the needed space, grow memory and then add the new code.
			self.memory.grow(self.ip + bytes.len() - self.memory.data.len());
			self.memory.set_range(self.ip, bytes.to_vec());
		}
	}

	pub fn execute_until_halt(&mut self) -> Result<bool, StrontiumError> {
		self.should_continue = true;

		while self.should_continue && !self.eof() {
			self.execute()?;
		}

		Ok(true)
	}

	pub fn consume_f64(&mut self) -> Result<f64, StrontiumError> {
		if (self.ip + 7 <= self.memory.data.len()) {
			println!("ip: {}    [before f16]", self.ip);
			let mut bytes = self.memory.range(self.ip .. self.ip + 8)?;
			println!("f64: bytes:  {:?} ", &bytes);

			let float = bytes
        			.read_f64::<LittleEndian>()
        			.expect("Unable to read f64 value");

        	println!("ip: {}    [after f16]", self.ip);
			Ok(float)
		} else {
			Err(StrontiumError::OutOfBounds)
		}
	}

	pub fn consume_u64(&mut self) -> Result<u64, StrontiumError> {
		if (self.ip + 7 <= self.memory.data.len()) {
			// We have enough space in memory to add the new code.
			let mut bytes = self.memory.range(self.ip .. self.ip + 8).unwrap();
			println!("{:?}", bytes);

			let int = bytes
        			.read_u64::<LittleEndian>()
        			.expect("Unable to read u64 value");

        	self.advance_by(8)?;

			Ok(int)
		} else {
			Err(StrontiumError::OutOfBounds)
		}
	}

	pub fn consume_u16(&mut self) -> Result<u16, StrontiumError> {
		if (self.ip + 7 <= self.memory.data.len()) {
			// We have enough space in memory to add the new code.
			let mut bytes = self.memory.range(self.ip .. self.ip + 8).unwrap();

			let int = bytes
        			.read_u16::<LittleEndian>()
        			.expect("Unable to read u16 value");

        	self.advance_by(2)?;

        	println!("ip: {}    [after u16]", self.ip);

			Ok(int)
		} else {
			Err(StrontiumError::OutOfBounds)
		}
	}

	/// Execute a single instruction
	pub fn execute(&mut self) -> Result<bool, StrontiumError> {
		let byte = self.peek();

		let opcode: Opcode = byte.into();

		println!("ip: {}", self.ip);

		self.should_continue = match opcode {
			HALT => {
				false
			},
			LOAD => {
				self.advance();
				let register = self.consume_u16()?;
				let value = self.consume_u64()?;

				println!("");

				self.registers[register as usize] = value;
				true
			},

			/*
			MOVE => self.move_value(source, destination)?,
			COPY => self.copy_value(source, destination),
			CALCULATE => self.calculate(method.clone(), operand1 as usize, operand2 as usize, destination as usize)?),
			
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
			},*/

			_ => {
				return Err(StrontiumError::IllegalOpcode(self.peek()))
			}
		};

		Ok(self.should_continue)
	}

	/*fn load(&mut self, value: f64, register: usize) -> Result<bool, StrontiumError> {
		if register <= NUM_REGISTERS {
			self.registers[register] = value;
		} else {
			return Err(StrontiumError::OutOfBounds);
		}
		
		self.advance();

		Ok(true)
	}

	fn move_value(&mut self, source: Location, destination: Location) -> Result<bool, StrontiumError> {
		match source {
			Location::Memory(src_address) => {
				match destination {
					Location::Memory(dest_address)   => {
						self.memory.compute(MemoryMethod::SET_RANGE {
							address: dest_address,
							values:  self.memory.range(src_address as usize .. src_address as usize + 8)?.to_vec(),
						})?;
					},

					Location::Register(dest_address)   => {
						if dest_address as usize <= NUM_REGISTERS {
							let mut range = self.memory.range(src_address as usize .. src_address as usize + 8)?;

							self.registers[dest_address as usize] = range.read_f64::<LittleEndian>()
        						.expect("Cannot read f64 value from memory");
						} else {
							return Err(StrontiumError::OutOfBounds)
						}
					},
				}
			},

			Location::Register(src_address) => {
				match destination {
					Location::Memory(dest_address)   => {
						if src_address  as usize <= NUM_REGISTERS {
							let mut  values = vec![];

							values
								.write_f64::<LittleEndian>(self.registers[src_address as usize])
								.expect("Cannot write f64 value to temporary buffer");

							self.memory.compute(MemoryMethod::SET_RANGE {
								address: dest_address,
								values,
							})?;
						} else {
							return Err(StrontiumError::OutOfBounds)
						}
					},
					Location::Register(dest_address) => {
						if    src_address  as usize <= NUM_REGISTERS 
						   && dest_address as usize <= NUM_REGISTERS {
						   	self.registers[src_address as usize] = self.registers[dest_address as usize];
						} else {
							return Err(StrontiumError::OutOfBounds)
						}
					},
				}
			}
		}
		
		Ok(true)
	}

	fn copy_value(&mut self, _source: Location, _destination: Location) -> bool {
		println!("The COPY instruction has not yet been implemented");
		true
	}

	fn jump(&mut self, destination: usize) -> bool {
		self.ip = destination;

		true
	}

	fn jumpc(&mut self, destination: usize, pointer: MemoryAddress) -> bool {
		if self.memory.data[pointer as usize] == 1 {
			self.ip = destination;
		}

		true
	}

	fn bitwise(&mut self, method: MemoryMethod) -> Result<bool, StrontiumError> {
		self.memory.compute(method)?;

		self.advance();

		Ok(true)
	}

	fn calculate(&mut self, method: CalculationMethod, a: usize, b: usize, destination: usize) -> Result<bool, StrontiumError> {
		match method {
			CalculationMethod::ADD 		=> self.registers[destination] = self.registers[a] + self.registers[b],
			CalculationMethod::SUBTRACT => self.registers[destination] = self.registers[a] - self.registers[b],
			CalculationMethod::MULTIPLY => self.registers[destination] = self.registers[a] * self.registers[b],
			CalculationMethod::DIVIDE 	=> self.registers[destination] = self.registers[a] / self.registers[b],
			CalculationMethod::POWER 	=> self.registers[destination] = self.registers[a].powf(self.registers[b]),
			CalculationMethod::MODULO 	=> self.registers[destination] = self.registers[a] % self.registers[b],
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

	fn interrupt(&mut self, _kind: Interrupt) -> Result<bool, StrontiumError> {
		Ok(true)
	}*/

	fn peek(&self) -> u8 {
		self.memory.data[self.ip]
	}

	fn advance(&mut self) -> bool {
		if self.ip + 1 < self.memory.data.len() {
			self.ip += 1;
			true
		} else {
			false
		}
	}

	fn advance_by(&mut self, n: usize) -> Result<(), StrontiumError> {
		if self.ip + n < self.memory.data.len() {
			self.ip += n;
			Ok(())
		} else {
			Err(StrontiumError::UnexpectedEof)
		}
	}

	/// Returns true when the instruction pointer is at the end of the memory array.
	fn eof(&self) -> bool {
		self.ip > self.memory.data.len()
	}
}

#[cfg(test)]
mod tests {
	use super::*;

    #[test]
    fn halt() {
 		let mut machine = Strontium::new();

 		machine.add_instruction(Instruction::HALT);

 		assert_eq!(machine.execute(), Ok(false));
    }

    #[test]
    fn load() {
 		let mut machine = Strontium::new();

 		machine.add_instruction(Instruction::LOAD { value: 1332.5, register: 5 });

 		machine.execute().unwrap();

 		assert_eq!(machine.registers[5], 1332.5);
    }

    #[test]
    fn add() {
 		let mut machine = Strontium::new();

 		machine.add_instruction(Instruction::LOAD { value: 44.7, register: 1 });
 		machine.add_instruction(Instruction::LOAD { value: 36.8, register: 2 });

 		machine.add_instruction(Instruction::CALCULATE { 
 			method: CalculationMethod::ADD,
 			operand1: 1,
 			operand2: 2,
 			destination: 3,
 		});

 		machine.execute().unwrap();
 		machine.execute().unwrap();
 		machine.execute().unwrap();

 		assert_eq!(machine.registers[3], 44.7 + 36.8);
    }

    #[test]
    fn subtract() {
 		let mut machine = Strontium::new();

 		machine.add_instruction(Instruction::LOAD { value: 3452.37, register: 1 });
 		machine.add_instruction(Instruction::LOAD { value: 3685.8148, register: 2 });

 		machine.add_instruction(Instruction::CALCULATE { 
 			method: CalculationMethod::SUBTRACT,
 			operand1: 1,
 			operand2: 2,
 			destination: 3,
 		});
 		
 		machine.execute().unwrap();
 		machine.execute().unwrap();
 		machine.execute().unwrap();

 		assert_eq!(machine.registers[3], 3452.37 - 3685.8148);
    }

    #[test]
    fn multiply() {
 		let mut machine = Strontium::new();

 		machine.add_instruction(Instruction::LOAD { value: 3.642, register: 1 });
 		machine.add_instruction(Instruction::LOAD { value: 2.46682, register: 2 });

 		machine.add_instruction(Instruction::CALCULATE { 
 			method: CalculationMethod::MULTIPLY,
 			operand1: 1,
 			operand2: 2,
 			destination: 3,
 		});
 		
 		machine.execute().unwrap();
 		machine.execute().unwrap();
 		machine.execute().unwrap();

 		assert_eq!(machine.registers[3], 3.642 * 2.46682);
    }

    #[test]
    fn divide() {
 		let mut machine = Strontium::new();

 		machine.add_instruction(Instruction::LOAD { value: 12.534, register: 1 });
 		machine.add_instruction(Instruction::LOAD { value: 8.388475294, register: 2 });

 		machine.add_instruction(Instruction::CALCULATE { 
 			method: CalculationMethod::DIVIDE,
 			operand1: 1,
 			operand2: 2,
 			destination: 3,
 		});
 		
 		machine.execute().unwrap();
 		machine.execute().unwrap();
 		machine.execute().unwrap();

 		assert_eq!(machine.registers[3], 12.534 / 8.388475294);
    }

    #[test]
    fn power() {
 		let mut machine = Strontium::new();

 		machine.add_instruction(Instruction::LOAD { value: 3.141592, register: 1 });
 		machine.add_instruction(Instruction::LOAD { value: 4.0, register: 2 });

 		machine.add_instruction(Instruction::CALCULATE { 
 			method: CalculationMethod::POWER,
 			operand1: 1,
 			operand2: 2,
 			destination: 3,
 		});
 		
 		machine.execute().unwrap();
 		machine.execute().unwrap();
 		machine.execute().unwrap();

 		assert_eq!(machine.registers[3], (3.141592 as f64).powf(4.0));
    }

    #[test]
    fn modulo() {
 		let mut machine = Strontium::new();

 		machine.add_instruction(Instruction::LOAD { value: 3.141592, register: 1 });
 		machine.add_instruction(Instruction::LOAD { value: 4.0, register: 2 });

 		machine.add_instruction(Instruction::CALCULATE { 
 			method: CalculationMethod::MODULO,
 			operand1: 1,
 			operand2: 2,
 			destination: 3,
 		});
 		
 		machine.execute().unwrap();
 		machine.execute().unwrap();
 		machine.execute().unwrap();

 		assert_eq!(machine.registers[3], 3.141592 % 4.0);
    }

    #[test]
    fn and() {
 		let mut machine = Strontium::new();

 		machine.add_instruction(Instruction::MEMORY {
 			method: MemoryMethod::GROW { amount: 8 }
 		});

 		machine.add_instruction(Instruction::MEMORY {
 			method: MemoryMethod::SET { value: 3, address: 2 }
 		});

 		machine.add_instruction(Instruction::MEMORY {
 			method: MemoryMethod::SET { value: 7, address: 3 }
 		});

 		machine.add_instruction(Instruction::MEMORY { 
 			method: MemoryMethod::AND {
 				a: 2,
 				b: 3,
 				out: 4,
 				len: 1,
 			}
 		});
 		
 		machine.execute().unwrap();
 		machine.execute().unwrap();
 		machine.execute().unwrap();
 		machine.execute().unwrap();

 		println!("{:?}", machine.memory.data);

 		assert_eq!(machine.memory.data[4], 3 & 7);
    }
}

