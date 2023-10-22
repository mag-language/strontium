//! This module contains the virtual machine which executes Strontium bytecode. The VM uses a set of typed 
//! registers to do number arithmetic, a memory vector provides the storage space for anything else.

pub mod instruction;
pub mod opcode;
pub mod register;

use self::opcode::Opcode;
use self::register::{RegisterValue, Registers};
use self::instruction::*;

use crate::types::StrontiumError;

use std::convert::TryInto;
use std::collections::HashMap;
use std::rc::Rc;
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
	executors: HashMap<Opcode, Rc<dyn Executor>>,
}

impl Strontium {
	/// Create a new instance of the virtual machine
	pub fn new() -> Self {
		let mut executors: HashMap<Opcode, Rc<dyn Executor>> = HashMap::new();

        executors.insert(Opcode::HALT, Rc::new(HaltExecutor));
        executors.insert(Opcode::LOAD, Rc::new(LoadExecutor));
        executors.insert(Opcode::CALCULATE, Rc::new(CalculateExecutor));

		Self {
			registers:  Registers::new(),
			ip:      	0,
			call_stack: vec![],
			should_continue: true,
			executors,
		}
	}

	/// Append machine code to the array in the bytecode register.
    pub fn push_bytecode(&mut self, bytes: Vec<u8>) {
		let mut bytecode = self.bc();
        bytecode.extend(bytes);
        self.registers.set("bc", RegisterValue::Array(
			bytecode
				.iter()
				.map(|v| RegisterValue::UInt8(*v))
				.collect()
		));
    }

	/// Execute a single instruction.
    pub fn execute(&mut self) -> Result<bool, StrontiumError> {
		let opcode: Opcode = self.consume_u8()?.into();
		let executor = self.executors.get(&opcode).cloned();

		self.should_continue = match executor {
			Some(executor) => executor.execute(self)?,
			None => return Err(StrontiumError::IllegalOpcode(self.peek())),
		};

		Ok(self.should_continue)
	}

	/// Execute instructions until a `HALT` instruction is encountered.
	pub fn execute_until_halt(&mut self) -> Result<bool, StrontiumError> {
        self.should_continue = true;

        while self.should_continue && !self.eof() {
            self.execute()?;
        }

        Ok(true)
    }

	fn ip(&self) -> usize {
		match self.registers.get("ip").unwrap() {
			RegisterValue::UInt64(ip) => *ip as usize,
			_ => unreachable!(),
		}
	}

	fn bc(&self) -> Vec<u8> {
		let bc = match self.registers.get("bc").unwrap() {
			RegisterValue::Array(bytes) => bytes.clone(),
			_ => unreachable!(),
		};

		bc.iter().map(|v| match v {
            RegisterValue::UInt8(b) => *b,
            _ => unreachable!(),
        }).collect()
	}

	fn _set_register(&mut self, name: &str, value: RegisterValue) {
		self.registers.set(name, value);
	}

	fn _get_register(&self, name: &str) -> Option<&RegisterValue> {
		self.registers.get(name)
	}

	fn consume_bytes(&mut self, size: usize) -> Result<Vec<u8>, StrontiumError> {
		let ip = self.ip();
		let bytecode = self.bc();

		if ip + size > bytecode.len() {
			Err(StrontiumError::UnexpectedEof)
		} else {
			let bytes = bytecode[ip .. ip + size].to_vec();
			self.advance_by(size)?;
			Ok(bytes)
		}
	}

	/// Consume an unsigned 64-bit integer from the bytecode register.
	///
	/// This performs a lookahead on the bytecode register to read the next eight bytes
	/// and converts the bytes into a 64-bit integer value. The byte encoding within
	/// Strontium bytecode is always Little Endian.
    pub fn consume_u64(&mut self) -> Result<u64, StrontiumError> {
		let bytes = self.consume_bytes(8)?;
		Ok(u64::from_le_bytes(bytes.try_into().unwrap()))
	}

	pub fn consume_u32(&mut self) -> Result<u32, StrontiumError> {
		let bytes = self.consume_bytes(4)?;
		Ok(u32::from_le_bytes(bytes.try_into().unwrap()))
	}

	pub fn consume_u16(&mut self) -> Result<u16, StrontiumError> {
		let bytes = self.consume_bytes(2)?;
		Ok(u16::from_le_bytes(bytes.try_into().unwrap()))
	}

	pub fn consume_u8(&mut self) -> Result<u8, StrontiumError> {
		let bytes = self.consume_bytes(1)?;
		Ok(bytes[0])
	}

	pub fn consume_i64(&mut self) -> Result<i64, StrontiumError> {
		let bytes = self.consume_bytes(8)?;
		Ok(i64::from_le_bytes(bytes.try_into().unwrap()))
	}

	pub fn consume_i32(&mut self) -> Result<i32, StrontiumError> {
		let bytes = self.consume_bytes(4)?;
		Ok(i32::from_le_bytes(bytes.try_into().unwrap()))
	}

	pub fn consume_i16(&mut self) -> Result<i16, StrontiumError> {
		let bytes = self.consume_bytes(2)?;
		Ok(i16::from_le_bytes(bytes.try_into().unwrap()))
	}

	pub fn consume_i8(&mut self) -> Result<i8, StrontiumError> {
		let bytes = self.consume_bytes(1)?;
		Ok(bytes[0] as i8)
	}

	pub fn consume_f64(&mut self) -> Result<f64, StrontiumError> {
		let bytes = self.consume_bytes(8)?;
		Ok(f64::from_le_bytes(bytes.try_into().unwrap()))
	}

	pub fn consume_f32(&mut self) -> Result<f32, StrontiumError> {
		let bytes = self.consume_bytes(4)?;
		Ok(f32::from_le_bytes(bytes.try_into().unwrap()))
	}

	pub fn consume_bool(&mut self) -> Result<bool, StrontiumError> {
		let bytes = self.consume_bytes(1)?;
		Ok(bytes[0] == 1)
	}

	pub fn consume_byte(&mut self) -> Result<u8, StrontiumError> {
		let bytes = self.consume_bytes(1)?;
		Ok(bytes[0])
	}

	pub fn consume_string(&mut self) -> Result<String, StrontiumError> {
		// First, consume the length of the string (assuming it's stored as a 32-bit unsigned integer)
		let length = self.consume_u32()? as usize;

		// Now, consume the actual string bytes
		let bytes = self.consume_bytes(length)?;

		// Convert the bytes to a UTF-8 string
		match String::from_utf8(bytes) {
			Ok(string) => Ok(string),
			Err(_) => Err(StrontiumError::InvalidUtf8String),
		}
	}

	fn peek(&self) -> u8 {
		let bytecode = self.bc();
		bytecode[self.ip()]
	}

	fn advance_by(&mut self, n: usize) -> Result<(), StrontiumError> {
		let ip = self.ip().clone();
		
		if ip + n < self.bc().len() {
			self.registers.set(
				"ip",
				RegisterValue::UInt64((ip + n) as u64),
			);
			Ok(())
		} else {
			Err(StrontiumError::UnexpectedEof)
		}
	}

	/// Returns true when the instruction pointer is at the end of the memory array.
	fn eof(&mut self) -> bool {
		let ip = self.ip().clone();
		ip > self.bc().len()
	}
}

mod test {

}

