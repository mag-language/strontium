use super::instruction::MemoryMethod;
use super::instruction::MemoryMethod::*;
use super::StrontiumError;

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
			data: vec![0; 2048]
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

	/// Return a slice of the memory vector
	pub fn range(&self, range: std::ops::Range<usize>) -> Result<&[u8], StrontiumError> {
		// We don't check for indexes lower than zero here 
		// because the `usize` type can never be negative.
		if range.end < self.data.len() {
			Ok(&self.data[range])
		} else {
			Err(StrontiumError::OutOfBounds)
		}
	}

	/// Set a byte to a given value
	pub fn set(&mut self, address: usize, value: u8) {
		self.data[address] = value;
	}

	/// Set a range of bytes
	pub fn set_range(&mut self, mut address: usize, values: Vec<u8>) {
		for value in values {
			self.set(address, value);
			address += 1;
		}
	}		

	/// Grow the container by **n** bytes
	pub fn grow(&mut self, amount: usize) { 
		self.data.append(&mut vec![0; amount]); 
	}

	/// Shrink the container by **n** bytes
	pub fn shrink(&mut self, amount: usize) -> Result<(), StrontiumError> {
		let length = self.data.len() as isize;

		if length - amount as isize >= 0 {
			self.data.truncate(length as usize - amount);
			Ok(())
		} else {
			Err(StrontiumError::OutOfBounds)
		}
	}

	/// Apply bitwise operations on ranges of bytes, or grow/shrink the container
	pub fn compute(&mut self, method: MemoryMethod) -> Result<(), StrontiumError> {
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
			SET_RANGE { address, values } => self.set_range(address as usize, values),
			UNSET { address } => self.set(address as usize, 0),
		}

		Ok(())
	}
}

pub type   Address = usize;

#[cfg(test)]
mod tests {
	use super::*;

    #[test]
    fn grow() {
    	let mut memory = Memory::new();

    	memory.compute(GROW { amount: 4 }).unwrap();

    	assert_eq!(memory.data.len(), 4)
    }

    #[test]
    fn shrink() {
    	let mut memory = Memory::new();

    	memory.compute(GROW { amount: 4 }).unwrap();
    	memory.compute(SHRINK { amount: 3 }).unwrap();

    	assert_eq!(memory.data.len(), 1)
    }

    #[test]
    fn set() {
    	let mut memory = Memory::new();

    	memory.compute(GROW { amount: 4 }).unwrap();
    	memory.compute(SET { address: 2, value: 64 }).unwrap();

    	assert_eq!(memory.data[2], 64)
    }

    #[test]
    fn set_range() {
    	let mut memory = Memory::new();

    	memory.compute(GROW { amount: 4 }).unwrap();
    	memory.compute(SET_RANGE { address: 0, values: vec![24, 42, 127, 255] }).unwrap();

    	assert_eq!(&memory.data[..], &[24, 42, 127, 255])
    }

    #[test]
    fn unset() {
    	let mut memory = Memory::new();

    	memory.compute(GROW { amount: 4 }).unwrap();
    	memory.compute(SET { address: 2, value: 111 }).unwrap();
    	memory.compute(UNSET { address: 2 }).unwrap();

    	assert_eq!(memory.data[2], 0)
    }

    #[test]
    fn or() {
    	let mut memory = Memory::new();

    	memory.compute(GROW { amount: 8 }).unwrap();
    	memory.compute(SET { address: 2, value: 111 }).unwrap();
    	memory.compute(SET { address: 3, value: 78 }).unwrap();
    	memory.compute(OR { a: 2, b: 3, out: 4, len: 1 }).unwrap();

    	assert_eq!(memory.data[4], 111 | 78)
    }

    #[test]
    fn and() {
    	let mut memory = Memory::new();

    	memory.compute(GROW { amount: 8 }).unwrap();
    	memory.compute(SET { address: 2, value: 111 }).unwrap();
    	memory.compute(SET { address: 3, value: 78 }).unwrap();
    	memory.compute(AND { a: 2, b: 3, out: 4, len: 1 }).unwrap();

    	assert_eq!(memory.data[4], 111 & 78)
    }

    #[test]
    fn xor() {
    	let mut memory = Memory::new();

    	memory.compute(GROW { amount: 8 }).unwrap();
    	memory.compute(SET { address: 2, value: 65 }).unwrap();
    	memory.compute(SET { address: 3, value: 223 }).unwrap();
    	memory.compute(XOR { a: 2, b: 3, out: 4, len: 1 }).unwrap();

    	assert_eq!(memory.data[4], 65 ^ 223)
    }

    #[test]
    fn not() {
    	let mut memory = Memory::new();

    	memory.compute(GROW { amount: 8 }).unwrap();
    	memory.compute(SET { address: 2, value: 65 }).unwrap();
    	memory.compute(SET { address: 3, value: 128 }).unwrap();
    	memory.compute(NOT { a: 2,  out: 4, len: 2 }).unwrap();

    	assert_eq!(memory.data[4], !65);
    	assert_eq!(memory.data[5], !128);
    }

    #[test]
    fn shift() {
    	let mut memory = Memory::new();

    	memory.compute(GROW { amount: 8 }).unwrap();
    	memory.compute(SET { address: 2, value: 65 }).unwrap();
    	memory.compute(SET { address: 4, value: 128 }).unwrap();
    	memory.compute(LSH { a: 2, out: 3, amount: 2, len: 1 }).unwrap();
    	memory.compute(RSH { a: 4, out: 5, amount: 6, len: 1 }).unwrap();

    	assert_eq!(memory.data[3], 65 << 2);
    	assert_eq!(memory.data[5], 128 >> 6);
    }
}
