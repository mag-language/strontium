use crate::machine::{Executor, Strontium, StrontiumError};

use crate::Instruction;

#[derive(Debug, Clone, PartialEq)]
pub struct LoadLocalExecutor;

impl Executor for LoadLocalExecutor {
    fn execute(&self, machine: &mut Strontium) -> Result<bool, StrontiumError> {
        if machine.debug {
            println!("LoadLocal instruction");
        }
        let instruction = machine.parse_instruction()?;

        if let Instruction::LoadLocal { name, register } = instruction {
            if let Some(frame) = machine.call_stack.last() {
                let value = frame
                    .local_variables
                    .get(&name)
                    .ok_or(StrontiumError::LocalVariableNotFound(name.clone()))?
                    .clone();
                machine.registers.set(&register, value);
            } else {
                return Err(StrontiumError::EmptyCallStack);
            }
        }

        Ok(true)
    }
}
