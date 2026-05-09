use crate::machine::register::RegisterValue;
use crate::machine::{Executor, StackFrame, Strontium, StrontiumError};

use crate::Instruction;

use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct CallExecutor;

impl Executor for CallExecutor {
    fn execute(&self, machine: &mut Strontium) -> Result<bool, StrontiumError> {
        if machine.debug {
            println!("CALL instruction");
        }
        let instruction = machine.parse_instruction()?;

        if let Instruction::Call { address } = instruction {
            // Save caller's temporary registers to protect from callee clobbering
            let mut saved_registers: HashMap<String, RegisterValue> = HashMap::new();
            for (name, value) in machine.registers.registers.iter() {
                if name.starts_with('r') && name[1..].parse::<u32>().is_ok() {
                    saved_registers.insert(name.clone(), value.clone());
                }
            }

            // Save current position as return address
            let return_address = machine.bytecode_parser.index;
            machine.call_stack.push(StackFrame {
                return_address,
                local_variables: HashMap::new(),
                saved_registers,
            });
            // Jump to method body
            machine.bytecode_parser.index = address;
        }

        Ok(true)
    }
}
