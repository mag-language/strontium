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

    fn consume_u64(&mut self) -> Result<u64, StrontiumError> {
		let ip = self.ip();

        let bytes: Vec<u8> = self.bc()[ip .. ip + 8].to_vec();

        let int = u64::from_le_bytes(bytes.try_into().unwrap());
        self.advance_by(8)?;
        Ok(int)
    }

    fn consume_u16(&mut self) -> Result<u16, StrontiumError> {
		let ip = self.ip();
        let bytes: Vec<u8> = self.bc()[ip .. ip + 2].to_vec();
        let int = u16::from_le_bytes(bytes.try_into().unwrap());

        self.advance_by(2)?;
        Ok(int)
    }

	pub fn execute_until_halt(&mut self) -> Result<bool, StrontiumError> {
        self.should_continue = true;
        while self.should_continue && !self.eof() {
            self.execute()?;
        }
        Ok(true)
    }

    pub fn execute(&mut self) -> Result<bool, StrontiumError> {
		let byte = self.peek();
		let opcode: Opcode = byte.into();
	
		let executor = self.executors.get(&opcode).cloned();
	
		self.should_continue = match executor {
			Some(executor) => executor.execute(self)?,
			None => return Err(StrontiumError::IllegalOpcode(self.peek())),
		};
	
		Ok(self.should_continue)
	}

/*
	/// Execute a single instruction
	pub fn execute(&mut self) -> Result<bool, StrontiumError> {
		let byte = self.peek();
		let opcode: Opcode = byte.into();

		println!("ip: {}", self.ip());

		self.should_continue = match opcode {
			// Stop all execution instantly.
			HALT => {
				false
			},

			// Load a value into a register.
			LOAD => {
                self.advance();
				// Parse the register index and value.
                let register_index = self.consume_u16()? as usize;
                let value = self.consume_u64()?;
				// Load the value into register.
                self.registers.set(register_index, RegisterValue::UInt(value));
                true
            },

			// Move a register value to another register
			MOVE => {
				self.advance();
				let source = self.consume_u16()? as usize;
				let destination = self.consume_u16()? as usize;
				let value = self.registers.get(source).unwrap().clone();
				self.registers.set(source, RegisterValue::Empty);
				self.registers.set(destination, value);
				true
			},

			// Implement the `COPY` instruction
			COPY => {
				self.advance();
				let source = self.consume_u16()? as usize;
				let destination = self.consume_u16()? as usize;
				let value = self.registers.get(source).unwrap().clone();
				self.registers.set(destination, value);
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
*/

	fn peek(&self) -> u8 {
		let bytecode = self.bc();
		bytecode[self.ip()]
	}

	fn advance(&mut self) -> bool {
		let ip = self.ip();

		if ip + 1 < self.bc().len() {
			self.registers.set(
				"ip",
				RegisterValue::UInt64((self.ip + 1) as u64),
			);
			true
		} else {
			false
		}
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

