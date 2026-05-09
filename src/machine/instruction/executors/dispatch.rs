use crate::machine::register::RegisterValue;
use crate::machine::{Executor, StackFrame, Strontium, StrontiumError};

use crate::Instruction;

use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct DispatchExecutor;

impl Executor for DispatchExecutor {
    fn execute(&self, machine: &mut Strontium) -> Result<bool, StrontiumError> {
        if machine.debug {
            println!("DISPATCH instruction");
        }
        let instruction = machine.parse_instruction()?;

        if let Instruction::Dispatch { method_name } = instruction {
            // Get the argument value from the 'arg' register
            let arg = machine
                .registers
                .get("arg")
                .ok_or(StrontiumError::RegisterNotFound("arg".to_string()))?
                .clone();

            if machine.debug {
                println!("  Dispatching {} with arg: {:?}", method_name, arg);
            }

            // Find the matching method in the dispatch table
            if let Some(address) = machine.dispatch(&method_name, &arg) {
                if machine.debug {
                    println!("  Matched! Jumping to address {}", address);
                }

                // Save caller's temporary registers to protect from callee clobbering
                // This is a caller-save convention for r0-r99 registers
                let mut saved_registers: HashMap<String, RegisterValue> = HashMap::new();
                for (name, value) in machine.registers.registers.iter() {
                    // Save all "r" numbered registers (r0, r1, r2, etc.)
                    if name.starts_with('r') && name[1..].parse::<u32>().is_ok() {
                        saved_registers.insert(name.clone(), value.clone());
                    }
                }

                // Save return address and create stack frame with saved registers
                let return_address = machine.bytecode_parser.index;
                machine.call_stack.push(StackFrame {
                    return_address,
                    local_variables: HashMap::new(),
                    saved_registers,
                });

                // Jump to the method body
                machine.bytecode_parser.index = address;
            } else {
                return Err(StrontiumError::MethodNotFound(method_name));
            }
        }

        Ok(true)
    }
}
