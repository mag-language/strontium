use crate::bytecode::BitwiseMethod;
use crate::bytecode::BitwiseMethod::*;

/// A generic, very low-level structure for data and code storage. It has no concept of numbers or strings. Instead, it manipulates an array
/// of bits. The basic operations can be used to store numbers or strings, compose methods like addition, multiplication, etc.
/// This makes the module really versatile.
pub struct Memory {
	pub data: Vec<bool>
}

impl Memory {
	/// Create a new memory instance
	pub fn new() -> Self {
		Self {
			data: vec![]
		}
	}

	/// Load a memory snapshot and drop the content from before
	pub fn load(&mut self, vec: &mut Vec<bool>) {
		self.data = vec.to_owned();
	}

	/// Add a vector of bits to the container
	pub fn append(&mut self, vec: &mut Vec<bool>) {
		self.data.append(vec);
	}

	/// Return the contents of the container
	pub fn dump(&mut self) -> Vec<bool> {
		self.data.clone()
	}

	/// Set the given bit to 1
	pub fn set(&mut self, a: usize, condition: bool) {
		if condition {
			self.data[a] = true;
		}
	}

	/// Set the given bit to 0
	pub fn unset(&mut self, a: usize) { 
		self.data[a] = false; 
	}	

	/// Grow the container by (n) bits
	pub fn grow(&mut self, amount: usize) { 
		self.data.append(&mut vec![false; amount]); 
	}

	/// Shrink the container by (n) bits
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
				// Create a temporary copy of the source range
		        let mut bits: Vec<bool> = self.data[a as usize .. a as usize + len].to_vec();

		        // Remove the first N elements
		        for _ in 0 .. amount {
		            bits.remove(0);
		        }

		        // Pad the right side with zeroes
		        for _ in 0 .. amount {
		            bits.push(false);
		        }

		        // Write the result to the second range
		        for (i, bit) in bits.iter().enumerate() {
		            if ((out as usize + i) as usize) < len {
		                self.data[out as usize + i] = *bit;
		            } else {
		                break;
		            }
		        }
			},

			RSH { a, out, amount, len } => {
				// Create a temporary copy of the source range
		        let mut bits: Vec<bool> = self.data[a as usize .. a as usize + len].to_vec();

		        let sign_bit = bits[0];
		        
		        // Remove the last N elements
		        for _ in 0 .. amount {
		            bits.pop();
		        }

		        // Pad with the sign bit
		        for _ in 0 .. amount {
		            bits.insert(0, sign_bit);
		        }

		        // Write the result to the second range
		        for (i, bit) in bits.iter().enumerate() {
		            if ((out as usize + i) as usize) < len {
		                self.data[out as usize + i] = *bit;
		            } else {
		                break;
		            }
		        }
			},

			GROW { amount }   => self.grow(amount as usize),
			SHRINK { amount } => self.shrink(amount as usize)?,
			SET { address } => self.set(address as usize, true),
			UNSET { address } => self.unset(address as usize),
		}

		Ok(())
	}

	pub fn is_set(&self, address: Address) -> bool {
		self.data[address]
	}	

	pub fn is_unset(&self, address: Address) -> bool {
		!self.data[address]
	}
}

pub type   Address = usize;
