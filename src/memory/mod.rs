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
	pub fn set(&mut self, a: usize) { 
		self.data[a] = true; 
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

	/// Perform a bitwise AND operation on a single bit
	pub fn single_and(&mut self, a: Address, b: Address, out: Address) -> Result<(), &'static str> {
		if self.addr_in_bounds(a) && self.addr_in_bounds(b) && self.addr_in_bounds(out) {
			if self.data[a] == true && self.data[b] == true {
				self.data[out] = true;
				Ok(())
			} else {
				self.data[out] = false;
				Ok(())
			}
		} else {
			Err("Target out of bounds")
		}
	}

	/// Perform a bitwise OR operation on a single bit
	pub fn single_or(&mut self, a: Address, b: Address, out: Address) -> Result<(), &'static str> {
		if self.addr_in_bounds(a) && self.addr_in_bounds(b) && self.addr_in_bounds(out) {
			// The result in each position is 0 if both bits are 0, while otherwise the result is 1.
			if self.data[a] == false && self.data[b] == false {
				self.data[out] = false;
				Ok(())
			} else {
				self.data[out] = true;
				Ok(())
			}
		} else {
			Err("Target out of bounds")
		}
	}

	/// Perform a bitwise exclusive OR operation on a single bit
	pub fn single_xor(&mut self, a: Address, b: Address, out: Address) -> Result<(), &'static str> {
		if self.addr_in_bounds(a) && self.addr_in_bounds(b) && self.addr_in_bounds(out) {
			if (self.is_set(a) && self.is_unset(b)) || (self.is_unset(a) && self.is_set(b)) {
				self.data[out] = true;
				Ok(())
			} else {
				self.data[out] = false;
				Ok(())
			}
		} else {
			Err("Target out of bounds")
		}
	}

	/// Perform a bitwise negation on a single bit
	pub fn single_not(&mut self, a: Address, out: Address) -> Result<(), &'static str> {
		if self.addr_in_bounds(a) && self.addr_in_bounds(out) {
			self.data[out] = !self.data[a];
			Ok(())
		} else {
			Err("Target out of bounds")
		}
	}

	/// Perform a bitwise AND operation on a range of bits
	pub fn range_and(&mut self, a: Range, b: Range, out: Range) -> Result<(), &'static str> {
		if self.addr_in_bounds(a.1) && self.addr_in_bounds(b.1) && self.addr_in_bounds(out.1) {
			let iterations = a.1 - a.0;

			// Iterate over the bits in parallel
			for offset in 0..iterations {
				self.single_and(a.0 + offset, b.0 + offset, out.0 + offset)?;
			}

			Ok(())
		} else {
			Err("Target out of bounds")
		}
	}

	/// Perform a bitwise OR operation on a range of bits
	pub fn range_or(&mut self, a: Range, b: Range, out: Range) -> Result<(), &'static str> {
		if self.addr_in_bounds(a.1) && self.addr_in_bounds(b.1) && self.addr_in_bounds(out.1) {
			let iterations = (a.1 - a.0) + 1;

			// Iterate over the bits in parallel
			for offset in 0..iterations {
				self.single_or(a.0 + offset, b.0 + offset, out.0 + offset)?;
			}

			Ok(())
		} else {
			Err("Target out of bounds")
		}
	}

	/// Perform a bitwise exclusive OR operation on a range of bits
	pub fn range_xor(&mut self, a: Range, b: Range, out: Range) -> Result<(), &'static str> {
		if self.addr_in_bounds(a.1) && self.addr_in_bounds(b.1) && self.addr_in_bounds(out.1) {
			let iterations = (a.1 - a.0) + 1;

			// Iterate over the bits in parallel
			for offset in 0..iterations {
				self.single_xor(a.0 + offset, b.0 + offset, out.0 + offset)?;
			}

			Ok(())
		} else {
			Err("Target out of bounds")
		}
	}

	// Perform a bitwise negation on a range of bits
	pub fn range_not(&mut self, a: Range, out: Range) -> Result<(), &'static str> {
		if self.addr_in_bounds(a.1) && self.addr_in_bounds(out.1) {
			let iterations = a.1 - a.0;

			// Iterate over the bits in parallel
			for offset in 0..=iterations {
				self.single_not(a.0 + offset, out.0 + offset)?;
			}

			Ok(())
		} else {
			Err("Target out of bounds")
		}
	}

	// Perform a left shift on a range of bits and write the result to another range
    pub fn lshift(&mut self, by: usize, a: Range, out: Range) -> Result<(), &'static str>  {
    	if self.addr_in_bounds(a.1) && self.addr_in_bounds(out.1) {
    		// Create a temporary copy of the source range
	        let mut bits: Vec<bool> = self.data[a.0 .. a.1].to_vec();

	        // Remove the first N elements
	        for _ in 0 .. by {
	            bits.remove(0);
	        }

	        // Pad the right side with zeroes
	        for _ in 0 .. by {
	            bits.push(false);
	        }

	        // Write the result to the second range
	        for (i, bit) in bits.iter().enumerate() {
	            if (out.0 + i) <= out.1 {
	                self.data[out.0 as usize + (i as usize)] = *bit;
	            } else {
	                break;
	            }
	        }

	        Ok(())
	    } else {
			Err("Target out of bounds")
		}
    }

    /// Perform an arithmetic (sign-preserving) right-shift on a range of bits and write the result to another range
    pub fn rshift(&mut self, by: usize, a: Range, out: Range) {
        // Create a temporary copy of the source range
        let mut bits: Vec<bool> = self.data[a.0 .. a.1].to_vec();

        let sign_bit = bits[0];
        
        // Remove the last N elements
        for _ in 0 .. by {
            bits.pop();
        }

        // Pad with the sign bit
        for _ in 0 .. by {
            bits.insert(0, sign_bit);
        }

        // Write the result to the second range
        for (i, bit) in bits.iter().enumerate() {
            if (out.0 + i) <= out.1 {
                self.data[out.0 as usize + (i as usize)] = *bit;
            } else {
                break;
            }
        }
    }

	// Checks if a [usize] address lies within the range of the container
	fn addr_in_bounds(&self, address: Address) -> bool {
		address < self.data.len()
	}

	pub fn is_set(&self, address: Address) -> bool {
		self.data[address]
	}	

	pub fn is_unset(&self, address: Address) -> bool {
		!self.data[address]
	}
}

pub type   Address = usize;

#[derive(Debug, Clone, PartialEq)]
pub struct Range(pub Address, pub Address);

#[cfg(test)]
mod tests {
    use super::*;

    #[test] fn grow() {
    	let mut mem = Memory::new();

    	mem.grow(5);
    	assert_eq!(mem.data.len(), 5);
    }

    #[test] fn shrink() {
    	let mut mem = Memory::new();

    	mem.grow(5);
    	mem.shrink(3).unwrap();

    	assert_eq!(mem.data.len(), 2);
    }

    #[test] fn shrink_error_handling() {
    	let mut mem = Memory::new();

    	mem.grow(5);

    	assert_eq!(mem.shrink(6), Err("Cannot shrink to a buffer size lower than zero"))
    }


    #[test] fn single_and() {
    	let mut mem = Memory::new();

    	mem.grow(8);

    	mem.data[1] = true;
    	mem.data[2] = true;

    	let res = mem.single_and(1, 2, 5);

    	match res {
    		Ok(()) => {},
    		Err(string) => println!("ERROR: {}", string),
    	}

    	assert_eq!(mem.data[5], true)
    }

    #[test] fn range_and() {
    	let mut mem = Memory::new();

    	mem.append(&mut vec![true,  true, false, false]);
    	mem.append(&mut vec![false, true, false, false]);
    	mem.grow(4);

    	mem.range_and(Range(0, 3), Range(4, 7), Range(8, 11)).unwrap();

    	assert_eq!(mem.data[8 ..= 11], [false, true, false, false])
    }


    #[test] fn single_or() {
    	let mut mem = Memory::new();

    	mem.grow(8);

    	mem.data[1] = true;
    	mem.data[2] = false;

    	let res = mem.single_or(1, 2, 3);

    	match res {
    		Ok(()) => {},
    		Err(string) => println!("ERROR: {}", string),
    	}

    	assert_eq!(mem.data[3], true)
    }

    #[test] fn range_or() {
    	let mut mem = Memory::new();

    	mem.append(&mut vec![true,  true, false, false]);
    	mem.append(&mut vec![false, true, false, true]);
    	mem.grow(4);
    	mem.range_or(Range(0, 3), Range(4, 7), Range(8, 11)).unwrap();

    	assert_eq!(mem.data[8 ..= 11], [true, true, false, true])
    }

    #[test] fn single_xor() {
    	let mut mem = Memory::new();
    	mem.grow(7);

    	mem.data[1] = true;
    	mem.data[2] = true;

    	mem.data[4] = true;
    	mem.data[5] = false;

    	mem.single_xor(1, 2, 3).unwrap();
    	mem.single_xor(4, 5, 6).unwrap();

    	assert_eq!(mem.data[3], false);
    	assert_eq!(mem.data[6], true);
    }

    #[test] fn range_xor() {
    	let mut mem = Memory::new();

    	mem.append(&mut vec![true,  true, false, false]);
    	mem.append(&mut vec![false, true, false, true]);
    	mem.grow(4);

    	mem.range_xor(Range(0, 3), Range(4, 7), Range(8, 11)).unwrap();

    	assert_eq!(mem.data[8 ..= 11], [true, false, false, true])
    }

    #[test] fn single_not() {
    	let mut mem = Memory::new();

    	mem.grow(8);
    	mem.data[3] = true;
    	mem.single_not(3, 3).unwrap();

    	assert_eq!(mem.data[3], false)
    }

    #[test] fn range_not() {
    	let mut mem = Memory::new();

    	mem.append(&mut vec![true, false, true, true, false, false]);
    	mem.grow(6);
    	mem.range_not(Range(0, 5), Range(6, 11)).unwrap();

    	assert_eq!(mem.data[6 ..= 11], [false, true, false, false, true, true])
    }
}