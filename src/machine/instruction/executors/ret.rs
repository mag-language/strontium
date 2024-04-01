use crate::machine::{
    Executor,
    Strontium,
    StrontiumError, StackFrame,
};

use crate::Instruction;

use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct ReturnExecutor;

impl Executor for ReturnExecutor {
    fn execute(&self, machine: &mut Strontium) -> Result<bool, StrontiumError> {
        let instruction = machine.parse_instruction()?;

        if let Instruction::RETURN = instruction {
            if let Some(frame) = machine.call_stack.pop() {
                machine.ip = frame.return_address;
            } else {
                return Err(StrontiumError::EmptyCallStack);
            }
        }

        Ok(true)
    }
}