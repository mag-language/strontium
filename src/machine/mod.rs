//! A virtual machine which executes Strontium instructions.

pub mod bytecode;
pub mod instruction;
pub mod opcode;
pub mod register;

use self::instruction::*;
use self::opcode::Opcode;
use self::register::{RegisterValue, Registers};

use self::bytecode::decode::BytecodeParser;
use crate::types::StrontiumError;

use std::collections::{BTreeMap, HashMap};
use std::convert::TryInto;
use std::rc::Rc;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

use self::instruction::DispatchPattern;

/// An entry in the multimethod dispatch table.
#[derive(Debug, Clone)]
pub struct DispatchEntry {
    /// The pattern to match against the argument.
    pub pattern: DispatchPattern,
    /// The bytecode address to jump to if this pattern matches.
    pub address: usize,
}

/// Maps method names to their dispatch entries (sorted by precedence, highest first).
pub type MultimethodTable = BTreeMap<String, Vec<DispatchEntry>>;

/// Shared cancellation state used by embedders to interrupt VM execution.
#[derive(Debug, Clone, Default)]
pub struct CancellationToken {
    cancelled: Arc<AtomicBool>,
}

impl CancellationToken {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn cancel(&self) {
        self.cancelled.store(true, Ordering::SeqCst);
    }

    pub fn reset(&self) {
        self.cancelled.store(false, Ordering::SeqCst);
    }

    pub fn is_cancelled(&self) -> bool {
        self.cancelled.load(Ordering::SeqCst)
    }

    pub fn flag(&self) -> Arc<AtomicBool> {
        Arc::clone(&self.cancelled)
    }
}

pub struct StackFrame {
    pub return_address: usize,
    pub local_variables: HashMap<String, RegisterValue>,
    /// Caller-saved registers, restored when returning from this frame.
    pub saved_registers: HashMap<String, RegisterValue>,
}

pub struct Strontium {
    /// Holds general-purpose registers for storing different types of values
    pub registers: Registers,
    executors: HashMap<Opcode, Rc<dyn Executor>>,
    /// Points to the next index in the buffer of the bytecode register.
    pub ip: usize,
    pub bytecode_parser: BytecodeParser,
    should_continue: bool,
    /// Maps method names to dispatch entries for runtime multimethod dispatch.
    pub multimethod_table: MultimethodTable,
    pub call_stack: Vec<StackFrame>,
    /// Whether to print debug output during execution.
    pub debug: bool,
}

impl Strontium {
    /// Create a new instance of the virtual machine
    pub fn new(debug: bool) -> Self {
        let mut executors: HashMap<Opcode, Rc<dyn Executor>> = HashMap::new();

        executors.insert(Opcode::Halt, Rc::new(HaltExecutor));
        executors.insert(Opcode::Load, Rc::new(LoadExecutor));
        executors.insert(Opcode::Calculate, Rc::new(CalculateExecutor));
        executors.insert(Opcode::Compare, Rc::new(CompareExecutor));
        executors.insert(Opcode::Call, Rc::new(CallExecutor));
        executors.insert(Opcode::Interrupt, Rc::new(InterruptExecutor));
        executors.insert(Opcode::Return, Rc::new(ReturnExecutor));
        executors.insert(Opcode::StoreLocal, Rc::new(StoreLocalExecutor));
        executors.insert(Opcode::LoadLocal, Rc::new(LoadLocalExecutor));
        executors.insert(Opcode::Jump, Rc::new(JumpExecutor));
        executors.insert(Opcode::Copy, Rc::new(CopyExecutor));
        executors.insert(Opcode::Dispatch, Rc::new(DispatchExecutor));

        let registers = Registers::new();

        Self {
            registers,
            executors,
            ip: 0,
            bytecode_parser: BytecodeParser::new(vec![], debug),
            should_continue: true,
            multimethod_table: BTreeMap::new(),
            call_stack: vec![],
            debug,
        }
    }

    /// Register a method variant in the dispatch table.
    /// Entries are kept sorted by precedence (highest first).
    pub fn register_method(&mut self, name: String, pattern: DispatchPattern, address: usize) {
        let entry = DispatchEntry { pattern, address };
        let entries = self.multimethod_table.entry(name).or_insert_with(Vec::new);

        // Insert in precedence order (highest first)
        let precedence = entry.pattern.precedence();
        let pos = entries
            .iter()
            .position(|e| e.pattern.precedence() < precedence)
            .unwrap_or(entries.len());
        entries.insert(pos, entry);
    }

    /// Dispatch a method call based on the argument value.
    /// Returns the bytecode address of the matching method, or None if no match.
    pub fn dispatch(&self, name: &str, arg: &RegisterValue) -> Option<usize> {
        if let Some(entries) = self.multimethod_table.get(name) {
            for entry in entries {
                if entry.pattern.matches(arg) {
                    return Some(entry.address);
                }
            }
        }
        None
    }

    /// Reset the VM state for a new execution.
    pub fn reset(&mut self) {
        self.registers.set("bc", RegisterValue::Array(vec![]));
        self.bytecode_parser.set_bytecode(vec![]);
        self.should_continue = true;
        self.call_stack.clear();
    }

    /// Abort the currently running program while keeping loaded methods and registers intact.
    pub fn abort_execution(&mut self) {
        self.should_continue = false;
        self.call_stack.clear();
    }

    /// Append machine code to the array in the bytecode register.
    pub fn push_bytecode(&mut self, bytes: Vec<u8>) {
        let mut bytecode = self.bc().to_vec();
        bytecode.extend(
            bytes
                .iter()
                .map(|b| RegisterValue::UInt8(*b))
                .collect::<Vec<RegisterValue>>(),
        );
        self.registers.set("bc", RegisterValue::Array(bytecode));
    }

    pub fn push_instruction(&mut self, instruction: Instruction) {
        //println!("pushing instruction: {:?}", instruction);
        let mut bytecode = self.bc().to_vec();
        let decoded: Vec<u8> = instruction.into();

        bytecode.append(
            &mut decoded
                .iter()
                .map(|b| RegisterValue::UInt8(*b))
                .collect::<Vec<RegisterValue>>(),
        );
        self.registers
            .set("bc", RegisterValue::Array(bytecode.clone()));
        self.bytecode_parser.set_bytecode(
            bytecode
                .iter()
                .map(|reg_value| match reg_value {
                    RegisterValue::UInt8(b) => *b,
                    _ => unreachable!(),
                })
                .collect(),
        );
    }

    pub fn parse_instruction(&mut self) -> Result<Instruction, StrontiumError> {
        Ok(self.bytecode_parser.parse_instruction()?)
    }

    /// Execute a single instruction.
    pub fn execute(&mut self) -> Result<bool, StrontiumError> {
        let opcode: Opcode = self.consume_u8()?.into();
        let executor = self.executors.get(&opcode).cloned();

        if self.debug {
            println!(
                "Launching instruction executor: {:?}",
                Opcode::from(opcode.clone())
            );
        }

        self.should_continue = match executor {
            Some(executor) => executor.execute(self)?,
            None => return Err(StrontiumError::IllegalOpcode(opcode as u8)),
        };

        Ok(self.should_continue)
    }

    pub fn execute_until_eof(&mut self) -> Result<bool, StrontiumError> {
        self.execute_until_eof_interruptible(|| false)
    }

    pub fn execute_until_eof_cancellable(
        &mut self,
        cancellation: &CancellationToken,
    ) -> Result<bool, StrontiumError> {
        self.execute_until_eof_interruptible(|| cancellation.is_cancelled())
    }

    pub fn execute_until_eof_interruptible<F>(
        &mut self,
        mut should_interrupt: F,
    ) -> Result<bool, StrontiumError>
    where
        F: FnMut() -> bool,
    {
        self.should_continue = true;

        while self.should_continue && !self.eof() {
            if should_interrupt() {
                self.abort_execution();
                return Err(StrontiumError::Interrupted);
            }

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

        if self.debug {
            println!("IP: {}", ip);
        }

        if ip + size > bytecode.len() {
            Err(StrontiumError::UnexpectedEof)
        } else {
            let bytes = bytecode[ip..ip + size].to_vec();
            if self.debug {
                println!("Advancing by: {}", size);
            }
            self.advance_by(size)?;

            Ok(bytes
                .iter()
                .map(|b| match b {
                    RegisterValue::UInt8(b) => *b,
                    _ => unreachable!(),
                })
                .collect::<Vec<u8>>())
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
        if self.debug {
            println!("Consume String");
        }
        // First, consume the length of the string (assuming it's stored as a 32-bit unsigned integer)
        let length = self.consume_u32()? as usize;
        if self.debug {
            println!("Length: {}", length);
        }

        // Now, consume the actual string bytes
        let bytes = self.consume_bytes(length)?;

        // Convert the bytes to a UTF-8 string
        match String::from_utf8(bytes) {
            Ok(string) => Ok(string),
            Err(_) => Err(StrontiumError::InvalidUtf8String),
        }
    }

    #[allow(dead_code)]
    fn peek(&self) -> u8 {
        self.bytecode_parser.bytecode[self.ip()]
    }

    fn advance_by(&mut self, n: usize) -> Result<(), StrontiumError> {
        let ip = self.ip().clone();

        if ip + n < self.bc().len() {
            self.registers
                .set("ip", RegisterValue::UInt64((ip + n) as u64));
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

mod test {}
