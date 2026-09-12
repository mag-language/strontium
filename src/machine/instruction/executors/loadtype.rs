use crate::machine::register::RegisterValue;
use crate::machine::{Executor, Strontium, StrontiumError};
use crate::Instruction;

#[derive(Debug, Clone, PartialEq)]
pub struct LoadTypeExecutor;

impl Executor for LoadTypeExecutor {
    fn execute(&self, machine: &mut Strontium) -> Result<bool, StrontiumError> {
        if machine.debug {
            println!("LOADTYPE instruction");
        }
        let instruction = machine.parse_instruction()?;

        if let Instruction::LoadType { source, destination } = instruction {
            let value = machine
                .registers
                .get(&source)
                .ok_or_else(|| StrontiumError::RegisterNotFound(source))?
                .clone();
            let type_tag = value.get_type() as i64;
            machine.registers.set(&destination, RegisterValue::Int64(type_tag));
        }

        Ok(true)
    }
}
