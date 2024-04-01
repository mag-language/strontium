//! A virtual machine which executes Strontium instructions.

pub mod bytecode;
pub mod instruction;
pub mod opcode;
pub mod register;

use self::opcode::Opcode;
use self::register::{RegisterValue, Registers};
use self::instruction::*;

use crate::types::{StrontiumError, ValueType};
use self::bytecode::decode::BytecodeParser;

use std::convert::TryInto;
use std::collections::{BTreeMap, HashMap};
use std::hash::Hash;
use std::rc::Rc;

pub struct DispatchMethod {
    pub method_name: String,
    pub argument_types: Vec<ValueType>,
    pub return_type: ValueType,
    pub bytecode_start: usize,
}

// Define the dispatch table
type DispatchKey = (String, Vec<ValueType>);
type DispatchTable = BTreeMap<DispatchKey, DispatchMethod>;

pub struct StackFrame {
    pub return_address: usize,
    pub local_variables: HashMap<String, RegisterValue>,
}

pub struct Strontium {
	/// Holds general-purpose registers for storing different types of values
	pub registers: 	 Registers,
	executors: 		 HashMap<Opcode, Rc<dyn Executor>>,
	/// Points to the next index in the buffer of the bytecode register.
	pub ip:        	 usize,
	bytecode_parser: BytecodeParser,
	should_continue: bool,
	dispatch_table:  DispatchTable,
	call_stack: 	 Vec<StackFrame>,
}

impl Strontium {
	/// Create a new instance of the virtual machine
	pub fn new() -> Self {
		let mut executors: HashMap<Opcode, Rc<dyn Executor>> = HashMap::new();

        executors.insert(Opcode::HALT, Rc::new(HaltExecutor));
        executors.insert(Opcode::LOAD, Rc::new(LoadExecutor));
        executors.insert(Opcode::CALCULATE, Rc::new(CalculateExecutor));
        executors.insert(Opcode::CALL, Rc::new(CallExecutor));
        executors.insert(Opcode::INTERRUPT, Rc::new(InterruptExecutor));
        executors.insert(Opcode::RETURN, Rc::new(ReturnExecutor));

		let registers = Registers::new();

		Self {
			registers,
			executors,
			ip:      		 0,
			bytecode_parser: BytecodeParser::new(vec![]),
			should_continue: true,
			dispatch_table:  BTreeMap::new(),
			call_stack: 	 vec![],
		}
	}

	// Method to add a method to the dispatch table
    pub fn add_method(&mut self, method: DispatchMethod) {
        let key = (method.method_name.clone(), method.argument_types.clone());
        self.dispatch_table.insert(key, method);
    }

    // Method to resolve a method call
    pub fn resolve_method(&self, name: &str, argument_types: Vec<ValueType>) -> Option<&DispatchMethod> {
        self.dispatch_table.get(&(name.to_string(), argument_types))
    }

	/// Append machine code to the array in the bytecode register.
    pub fn push_bytecode(&mut self, bytes: Vec<u8>) {
		let mut bytecode = self.bc().to_vec();
        bytecode.extend(bytes.iter().map(|b| RegisterValue::UInt8(*b)).collect::<Vec<RegisterValue>>());
        self.registers.set("bc", RegisterValue::Array(bytecode));
    }

	pub fn push_instruction(&mut self, instruction: Instruction) {
		//println!("pushing instruction: {:?}", instruction);
		let mut bytecode = self.bc().to_vec();
		let decoded: Vec<u8> = instruction.into();

		bytecode.append(&mut
			decoded
				.iter()
				.map(|b| RegisterValue::UInt8(*b))
				.collect::<Vec<RegisterValue>>()
		);
		self.registers.set("bc", RegisterValue::Array(bytecode.clone()));
		self.bytecode_parser.set_bytecode(bytecode.iter().map(|reg_value| { match reg_value {
			RegisterValue::UInt8(b) => *b,
			_ => unreachable!(),
		}}).collect());
	}

	pub fn parse_instruction(&mut self) -> Result<Instruction, StrontiumError> {
		Ok(self.bytecode_parser.parse_instruction()?)
	}

	/// Execute a single instruction.
    pub fn execute(&mut self) -> Result<bool, StrontiumError> {
		let opcode: Opcode = self.consume_u8()?.into();
		let executor = self.executors.get(&opcode).cloned();

		println!("Launching instruction executor: {:?}", Opcode::from(opcode.clone()));

		self.should_continue = match executor {
			Some(executor) => executor.execute(self)?,
			None => return Err(StrontiumError::IllegalOpcode(opcode as u8)),
		};

		Ok(self.should_continue)
	}

	pub fn execute_until_eof(&mut self) -> Result<bool, StrontiumError> {
		self.should_continue = true;

		while self.should_continue && !self.eof() {
			self.execute()?;
		}

		Ok(true)
	}

	/// Execute instructions until a `HALT` instruction is encountered.
/*
	pub fn execute_until_halt(&mut self) -> Result<bool, StrontiumError> {
        self.should_continue = true;

        while self.should_continue && !self.eof() {
            self.execute()?;
        }

        Ok(true)
    }
*/

	fn ip(&self) -> usize {
		self.bytecode_parser.index
	}

	fn bc(&self) -> Vec<RegisterValue> {
		let bc = match self.registers.get("bc").unwrap() {
			RegisterValue::Array(bytes) => bytes.clone(),
			_ => unreachable!(),
		};

		bc
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

		println!("IP: {}", ip);

		if ip + size > bytecode.len() {
			Err(StrontiumError::UnexpectedEof)
		} else {
			let bytes = bytecode[ip .. ip + size].to_vec();
			println!("Advancing by: {}", size);
			self.advance_by(size)?;

			Ok(bytes.iter().map(|b| match b {
				RegisterValue::UInt8(b) => *b,
				_ => unreachable!(),
			}).collect::<Vec<u8>>())
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
		println!("Consume String");
		// First, consume the length of the string (assuming it's stored as a 32-bit unsigned integer)
		let length = self.consume_u32()? as usize;
		println!("Length: {}", length);

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
		self.bytecode_parser.bytecode[self.ip()]
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

