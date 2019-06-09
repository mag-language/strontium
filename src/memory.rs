use crate::bytecode::BitwiseMethod;
use crate::bytecode::BitwiseMethod::*;

/// A generic, low-level structure for data and code storage. It has no concept of numbers or strings. Instead, it manipulates an array
/// of bytes. The basic operations can be used to store numbers or strings, compose methods like addition, multiplication, etc.
/// This makes the module really versatile.
pub struct Memory {
	pub data: Vec<u8>
}

impl Memory {
	/// Create a new memory instance
	pub fn new() -> Self {
		Self {
			data: vec![]
		}
	}

	/// Load a memory snapshot and drop the content from before
	pub fn load(&mut self, vec: &mut Vec<u8>) {
		self.data = vec.to_owned();
	}

	/// Add a vector of bytes to the container
	pub fn append(&mut self, vec: &mut Vec<u8>) {
		self.data.append(vec);
	}

	/// Return the contents of the container
	pub fn dump(&mut self) -> Vec<u8> {
		self.data.clone()
	}

	/// Set a byte to a given value
	pub fn set(&mut self, address: usize, value: u8) {
		self.data[address] = value;
	}	

	/// Grow the container by **n** bytes
	pub fn grow(&mut self, amount: usize) { 
		self.data.append(&mut vec![0; amount]); 
	}

	/// Shrink the container by **n** bytes
	pub fn shrink(&mut self, amount: usize) -> Result<(), &'static str> {
		let length = self.data.len() as isize;

		if length - amount as isize >= 0 {
			self.data.truncate(length as usize - amount);
			Ok(())
		} else {
			Err("Cannot shrink to a buffer size lower than zero")
		}
	}

	pub fn compute(&mut self, method: BitwiseMethod) -> Result<(), String> {
		match method {
			AND { a, b, out, len } => {
				let out = out as usize;
				let a   = a as usize;
				let b   = b as usize;

				for offset in 0 .. len {
					self.data[out + offset] = self.data[a + offset] & self.data[b + offset];
				}
			},

			OR { a, b, out, len } => {
				let out = out as usize;
				let a   = a as usize;
				let b   = b as usize;

				for offset in 0 .. len {
					self.data[out + offset] = self.data[a + offset] | self.data[b + offset];
				}
			},

			XOR { a, b, out, len } => {
				let out = out as usize;
				let a   = a as usize;
				let b   = b as usize;

				for offset in 0 .. len {
					self.data[out + offset] = self.data[a + offset] ^ self.data[b + offset];
				}
			},

			NOT { a, out, len } => {
				let out = out as usize;
				let a   = a as usize;

				for offset in 0 .. len {
					self.data[out + offset] = !self.data[a + offset];
				}
			},

			LSH { a, out, amount, len } => {
		        let out = out as usize;
				let a   = a as usize;

				for offset in 0 .. len {
					self.data[out + offset] = self.data[a + offset] << amount;
				}
			},

			RSH { a, out, amount, len } => {
				let out = out as usize;
				let a   = a as usize;

				for offset in 0 .. len {
					self.data[out + offset] = self.data[a + offset] >> amount;
				}
			},

			GROW { amount }   => self.grow(amount as usize),
			SHRINK { amount } => self.shrink(amount as usize)?,
			SET { address, value } => self.set(address as usize, value),
		}

		Ok(())
	}
}

pub type   Address = usize;
