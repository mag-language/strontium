use crate::machine::{Executor, Strontium, StrontiumError};

use crate::Instruction;

#[derive(Debug, Clone, PartialEq)]
pub struct StoreLocalExecutor;

impl Executor for StoreLocalExecutor {
    fn execute(&self, machine: &mut Strontium) -> Result<bool, StrontiumError> {
        if machine.debug {
            println!("StoreLocal instruction");
        }
        let instruction = machine.parse_instruction()?;

        if let Instruction::StoreLocal { name, register } = instruction {
            let value = machine
                .registers
                .get(&register)
                .ok_or(StrontiumError::RegisterNotFound(register.clone()))?
                .clone();

            if let Some(frame) = machine.call_stack.last_mut() {
                frame.local_variables.insert(name, value);
            } else {
                return Err(StrontiumError::EmptyCallStack);
            }
        }

        Ok(true)
    }
}
