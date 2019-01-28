use std::io::Cursor;
use bytes::{Bytes, Buf};

pub struct Assembler {
	characters: Vec<char>,
	input:		String,
	index:		usize
}

impl Assembler {
	pub fn new(input: String) -> Self {
		Self {
			characters: input.chars().collect(),
			input:		input,
			index:		0,
		}
	}

	pub fn parse(&mut self) -> Result<Vec<Token>, String> {
		let mut tokens = Vec::new();

		while !self.eof() {
			let character = self.peek();

			match character {
				'a' ... 'z' | 'A' ... 'Z' => {
					tokens.push(self.parse_text());
				},

				// Seems like this might be an address
				'0' => {
					if self.starts_with("0x") {
						self.next();
						self.next();

						tokens.push(Token::Address(self.parse_address()?));
					}
				},

				// Ignore spaces, tabs and newlines
				' ' | '\t' | '\n' | '\r' => {
					self.next();
				},

				// We've found a range
				'(' => {
					self.next();
					tokens.push(self.parse_range()?);
				},

				_ => { return Err(format!("Unexpected character: {}", character)) }
			}
		}

		Ok(tokens)
	}

	fn parse_text(&mut self) -> Token {
		let mut result = String::new();

		while !self.eof() {
			let character = self.peek();

			match character {
				'a' ... 'z' | 'A' ... 'Z' => {
					result.push(character);
					self.next();
				},
				_ => {
					break
				}
			}
		}

		Token::Text(result)
	}

	fn parse_address(&mut self) -> Result<u32, String> {
		let mut address = String::new();

		while !self.eof() {
			let character = self.peek();

			// Allow only hexadecimal characters
			match character {
				'a' ... 'f' | 'A' ... 'F' | '0' ... '9' => {
					address.push(character);
					self.next();
				},
				_ => {
					break
				}
			}
		}

		if address.len() == 8 {
			Ok(hex_to_u32(address))
		} else {
			Err(format!("Expected address length to be 8 characters instead of {}\nAddress: {}", address.len(), address))
		}
		
	}

	fn parse_range(&mut self) -> Result<Token, String> {
		let mut addresses = Vec::new();

		while !self.eof() {
			let character = self.peek();

			// Allow only hexadecimal characters
			match character {
				'0' => {
					if self.starts_with("0x") {
						self.next();
						self.next();

						addresses.push(self.parse_address()?);
					}
				},

				' ' | '\t' | '\r' => { self.next(); },

				')' => { self.next(); break },

				_ => { self.next(); }
			}
		}

		if addresses.len() == 2 {
			Ok(Token::Range(addresses[0], addresses[1]))
		} else {
			Err("A range must contain exactly two addresses".to_string())
		}
	}

	fn eof(&self) -> bool {
		self.index == self.characters.len()
	}

	fn peek(&self) -> char {
		self.characters[self.index]
	}

	fn next(&mut self) -> Option<char> {
		if !self.eof() {
			self.index += 1;
			Some(self.characters[self.index - 1])
		} else {
			None
		}
	}

	// Do the next characters start with the given string?
    fn starts_with(&self, s: &str) -> bool {
        self.input[self.index .. ].starts_with(s)
    }
}

#[derive(Debug, PartialEq)]
pub enum Token {
	Text(String),
	Address(u32),
	Range(u32, u32),
}

fn hex_to_u32(input: String) -> u32 {
	// Create a temporary container and decode the HEX string into bytes
	let mut cursor = Cursor::new(hex::decode(input).unwrap());
	cursor.get_u32_be()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test] fn parse_text() {
    	let mut asm = Assembler::new("ALLOCATE 		SET XOR AND NOT".to_string());

    	assert_eq!(asm.parse(), Ok(vec![
    		Token::Text("ALLOCATE".to_string()),
    		Token::Text("SET".to_string()),
    		Token::Text("XOR".to_string()),
    		Token::Text("AND".to_string()),
    		Token::Text("NOT".to_string()),
    	]))
    }

    #[test] fn parse_address() {
    	let mut asm = Assembler::new("0x499602D2 0xFFFFFFFF 0x00000000".to_string());

    	assert_eq!(asm.parse(), Ok(vec![
    		Token::Address(1234567890),
    		Token::Address(4294967295),
    		Token::Address(0)
    	]))
    }

    #[test] fn parse_range() {
    	let mut asm = Assembler::new("(0x499602D2 0xFFFFFFFF) (0x00000001 0x00000003)".to_string());

    	assert_eq!(asm.parse(), Ok(vec![
    		Token::Range(1234567890, 4294967295),
    		Token::Range(1, 3)
    	]))
    }
}