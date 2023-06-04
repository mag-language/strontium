//! This module contains the virtual machine which executes Strontium bytecode. The VM uses a set of typed 
//! registers to do number arithmetic, a memory vector provides the storage space for anything else.

//pub mod instruction;
pub mod opcode;
pub mod register;

use self::opcode::Opcode;
use self::opcode::Opcode::*;
use self::register::{RegisterValue, Registers, Reserved};

use crate::types::StrontiumError;

// use byteorder::{LittleEndian, ReadBytesExt};

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
	Bytes(Vec<u8>),
	Int(i64),
	UInt(u64),
	Float(f64),
	String(String),
}


pub struct StackFrame {
    /// The instruction pointer to return to after the function returns
    pub return_address: usize,
    /// The index of the first argument in the `registers` array
    pub arg_start: usize,
    /// The number of arguments passed to the function
    pub arg_count: usize,
}

pub struct Strontium {
	/// Holds general-purpose registers for storing different types of values
	pub registers: Registers,
	/// Points to the next index in the buffer of the bytecode register.
	pub ip:        usize,
	/// Contains references for function arguments and return values
	pub call_stack: Vec<u8>,
	should_continue: bool,
}

impl Strontium {
	/// Create a new instance of the virtual machine
	pub fn new() -> Self {
		Self {
			registers:  Registers::new(),
			ip:      	0,
			call_stack: vec![],
			should_continue: true,
		}
	}

	pub fn push_bytecode(&mut self, bytes: Vec<u8>) {
		let ip = self.ip();
		let mut bytecode = self.get_bytecode().clone();
		bytecode.append(
			&mut bytes
				.iter()
				.map(|byte| RegisterValue::UInt8(*byte))
				.collect()
		);

		self.registers.set_reserved(
			Reserved::Ip,
			RegisterValue::UInt((ip + 1) as u64),
		);
	}

	fn get_reserved_register(&mut self, r: Reserved) -> &RegisterValue {
		self.registers.get_reserved(r)
	}

	fn get_bytecode(&mut self) -> &Vec<RegisterValue> {
		match self.get_reserved_register(Reserved::Bc) {
			RegisterValue::Array(vec) => &vec,
			_ => unreachable!(),
		}
	}

	fn ip(&mut self) -> usize {
		self.registers.ip()
	}

	fn bc(&mut self) -> Vec<u8> {
		self.registers.bc()
	}

	pub fn execute_until_halt(&mut self) -> Result<bool, StrontiumError> {
		self.should_continue = true;

		while self.should_continue && !self.eof() {
			self.execute()?;
		}

		Ok(true)
	}

	/*pub fn consume_f64(&mut self) -> Result<f64, StrontiumError> {
		if self.get_ip() + 7 <= self.get_bytecode().len() {
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
		if self.get_ip() + 7 <= self.get_bytecode().len() {
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
		if self.get_ip() + 7 <= self.get_bytecode().len().try_into().unwrap() {
			// We have enough space in memory to add the new code.
			let mut bytes = self.memory.range(self.get_ip() .. self.get_ip() + 8).unwrap();

			let int = bytes
        			.read_u16::<LittleEndian>()
        			.expect("Unable to read u16 value");

        	self.advance_by(2)?;

        	println!("ip: {}    [after u16]", self.ip);

			Ok(int)
		} else {
			Err(StrontiumError::OutOfBounds)
		}
	}*/

	/// Execute a single instruction
	pub fn execute(&mut self) -> Result<bool, StrontiumError> {
		let byte = self.peek();

		let opcode: Opcode = byte.into();

		println!("ip: {}", self.ip());

		self.should_continue = match opcode {
			HALT => {
				false
			},
			LOAD => {
				self.advance();
/*
				let register = self.consume_u16()?;
				let value = self.consume_u64()?;

				println!("");

				self.registers[register as usize] = Value::UInt(value);
*/
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

	fn peek(&self) -> u8 {
		let bytecode = self.registers.bc();
		bytecode[self.registers.ip()]
	}

	fn advance(&mut self) -> bool {
		let ip = self.ip();

		if ip + 1 < self.bc().len() {
			self.registers.set_reserved(
				Reserved::Ip,
				RegisterValue::UInt((self.ip + 1) as u64),
			);
			true
		} else {
			false
		}
	}
/*
	fn advance_by(&mut self, n: usize) -> Result<(), StrontiumError> {
		let ip = self.ip();
		
		if ip + n < self.bc().len() {
			self.registers.set_reserved(
				Reserved::Ip,
				RegisterValue::UInt((ip + n) as u64),
			);
			Ok(())
		} else {
			Err(StrontiumError::UnexpectedEof)
		}
	}
*/
	/// Returns true when the instruction pointer is at the end of the memory array.
	fn eof(&mut self) -> bool {
		self.ip > self.bc().len()
	}
}

