use crate::machine::{Executor, Strontium, StrontiumError};

use crate::Instruction;

#[derive(Debug, Clone, PartialEq)]
pub struct CopyExecutor;

impl Executor for CopyExecutor {
    fn execute(&self, machine: &mut Strontium) -> Result<bool, StrontiumError> {
        if machine.debug {
            println!("COPY instruction");
        }
        let instruction = machine.parse_instruction()?;

        if let Instruction::Copy {
            source,
            destination,
        } = instruction
        {
            let value = machine
                .registers
                .get(&source)
                .ok_or(StrontiumError::RegisterNotFound(source.clone()))?
                .clone();
            machine.registers.set(&destination, value);
        }

        Ok(true)
    }
}
