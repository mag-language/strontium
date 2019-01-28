use super::memory::{Memory, Range};

use super::instruction::Instruction;
use super::instruction::Instruction::*;

use bytes::{Buf};
use std::io::Cursor;

pub struct Machine {
	/// The machine's current position in the bytecode
	pub pointer: usize,
	/// Contains the bytecode of the program
	pub program: Vec<u8>,
	/// Models physical memory as an array of bits
	pub memory:  Memory,
	/// Indicates if there has been an error
	error:   bool,
}

impl Machine {
	pub fn new() -> Self {
		Self {
			pointer: 0,
			program: vec![],
			memory:  Memory::new(),
			error:   false
		}
	}

	pub fn append(&mut self, mut vec: Vec<u8>) {
		self.program.append(&mut vec);
	}

	pub fn dump_memory(&self) -> Vec<bool> {
		self.memory.data.clone()
	}

	fn is_at_end(&mut self) -> bool {
		self.pointer + 4 > self.program.len() 
	}

	// Execute a single instruction
	pub fn execute(&mut self) -> Result<bool, &'static str> {
		if self.program.len() > 0 {
			let instruction = Instruction::from(self.program[self.pointer]);

			match instruction {
				// Stop all execution immediately
				HALT => {
					println!("Halt signal encountered, shutting down.");
					Ok(false)
				},

				SET => {
					let address_amount = self.program[self.pointer + 3];
					self.advance()?;

					// get_u32 advances implicitly
					for _ in 0 .. address_amount {
						let address = self.get_u32()?;
						self.memory.set(address);
					}

					Ok(true)
				},

				UNSET => {
					let address_amount = self.program[self.pointer + 3];
					self.advance()?;

					for _ in 0 .. address_amount {
						let address = self.get_u32()?;
						self.memory.unset(address);
					}

					Ok(true)
				},

				ALLOCATE => {
					self.advance()?;
					let amount = self.get_u32()?;

					self.memory.grow(amount);

					Ok(true)
				},

				DEALLOCATE => {
					self.advance()?;
					let amount = self.get_u32()?;
					self.memory.shrink(amount)?;

					Ok(true)
				},

				AND => {
					// Check if we are in single-bit or range mode
					let is_range = self.program[self.pointer + 3];
					self.advance()?;

					if is_range == 1 {
						let (a1, a2) 	 = (self.get_u32()?, self.get_u32()?);
						let (b1, b2) 	 = (self.get_u32()?, self.get_u32()?);
						let (out1, out2) = (self.get_u32()?, self.get_u32()?);

						self.memory.range_and(Range(a1, a2), Range(b1, b2), Range(out1, out2))?;

						Ok(true)
					} else {
						let (a, b, out)  = (self.get_u32()?, self.get_u32()?, self.get_u32()?);
						self.memory.single_and(a, b, out)?;

						Ok(true)
					}
				},

				OR => {
					// Check if we are in single-bit or range mode
					let is_range = self.program[self.pointer + 3];
					self.advance()?;

					if is_range == 1 {
						let (a1, a2) 	 = (self.get_u32()?, self.get_u32()?);
						let (b1, b2) 	 = (self.get_u32()?, self.get_u32()?);
						let (out1, out2) = (self.get_u32()?, self.get_u32()?);

						self.memory.range_or(Range(a1, a2), Range(b1, b2), Range(out1, out2))?;

						Ok(true)
					} else {
						let (a, b, out)  = (self.get_u32()?, self.get_u32()?, self.get_u32()?);
						self.memory.single_or(a, b, out)?;

						Ok(true)
					}
				},

				XOR => {
					// Check if we are in single-bit or range mode
					let is_range = self.program[self.pointer + 3];

					if is_range == 1 {
						let (a1, a2) 	 = (self.get_u32()?, self.get_u32()?);
						let (b1, b2) 	 = (self.get_u32()?, self.get_u32()?);
						let (out1, out2) = (self.get_u32()?, self.get_u32()?);

						self.memory.range_xor(Range(a1, a2), Range(b1, b2), Range(out1, out2))?;
						Ok(true)
					} else {
						let (a, b, out)  = (self.get_u32()?, self.get_u32()?, self.get_u32()?);
						self.memory.single_xor(a, b, out)?;
						Ok(true)
					}
				},

				NOT => {
					// Check if we are in single-bit or range mode
					let is_range = self.program[self.pointer + 3];

					if is_range == 1 {
						let (a1, a2) 	 = (self.get_u32()?, self.get_u32()?);
						let (out1, out2) = (self.get_u32()?, self.get_u32()?);

						self.memory.range_not(Range(a1, a2), Range(out1, out2))?;

						self.advance()?;
						Ok(true)
					} else {
						let (a, out)  = (self.get_u32()?, self.get_u32()?);
						self.memory.single_not(a, out)?;

						self.advance()?;
						Ok(true)
					}
				},

				LSHIFT => {
					let range_len = self.program[self.pointer + 3];

					let (a1, a2) 	 = (self.get_u32()?, self.get_u32()?);
					let (out1, out2) = (self.get_u32()?, self.get_u32()?);

					self.memory.lshift(range_len as usize, Range(a1, a2), Range(out1, out2))?;

					self.advance()?;
					Ok(true)
				},

				RSHIFT => {
					let range_len = self.program[self.pointer + 3];

					let (a1, a2) 	 = (self.get_u32()?, self.get_u32()?);
					let (out1, out2) = (self.get_u32()?, self.get_u32()?);

					self.memory.rshift(range_len as usize, Range(a1, a2), Range(out1, out2));

					self.advance()?;
					Ok(true)
				},

				JUMP => {
					self.pointer = self.get_u32()? as usize;

					self.advance()?;
					Ok(true)
				},

				JUMPIF => {
					let position  = self.get_u32()? as usize;
					let check_bit = self.get_u32()? as usize;

					if self.memory.is_set(check_bit) {
						self.pointer = position;
					}

					self.advance()?;
					Ok(true)
				},

				_ => {
					println!("Exception: Illegal opcode");
					Ok(false)
				}
			}
		} else {
			Err("The program vector is empty")
		}
	}

	/// Parse an u32 from the program buffer
    pub fn get_u32(&mut self) -> Result<usize, &'static str> {
        // Let's create a buffer slice that contains the needed bytes
        let mut bytes = Cursor::new(&self.program[self.pointer .. self.pointer + 4]);
        let num = bytes.get_u32_be() as usize;

        if self.program.len() > self.pointer + 4 {
        	self.advance()?;
        } 
        Ok(num)
    }

	fn advance(&mut self) -> Result<(), &'static str> {
		if self.program.len() > self.pointer + 4 {
			self.pointer += 4;
			Ok(())
		} else {
			Err("Cannot advance the pointer any further")
		}
	}

	pub fn execute_loop(&mut self) {
		while !self.is_at_end() {
			match self.execute() {
				Ok(should_continue) => {
					if !should_continue { break; }
				},
				Err(e) => {
					println!("{}", e);
					self.error = true;
				}
			}
		}
	}
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test] fn get_u32() {
    	let mut machine = Machine::new();

    	machine.append(vec![3, 0, 0, 0, // Allocate 8 bits
    						0, 0, 0, 8,
    					    1, 0, 0, 1, // Set the bit at position 4
    					    0, 0, 0, 4,
    					    0, 0, 0, 0]);

    	machine.execute_loop();

    	assert_eq!(machine.memory.data[4], true);
    }
}