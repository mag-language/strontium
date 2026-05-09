use crate::machine::{Executor, Strontium, StrontiumError};

use crate::Instruction;

#[derive(Debug, Clone, PartialEq)]
pub struct ReturnExecutor;

impl Executor for ReturnExecutor {
    fn execute(&self, machine: &mut Strontium) -> Result<bool, StrontiumError> {
        if machine.debug {
            println!("RETURN instruction");
        }
        let instruction = machine.parse_instruction()?;

        if let Instruction::Return = instruction {
            if let Some(frame) = machine.call_stack.pop() {
                // Return to instruction after the CALL
                machine.bytecode_parser.index = frame.return_address;

                // Restore caller-saved registers
                for (name, value) in frame.saved_registers {
                    machine.registers.set(&name, value);
                }
            } else {
                return Err(StrontiumError::EmptyCallStack);
            }
        }

        Ok(true)
    }
}
