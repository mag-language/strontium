use crate::machine::{Executor, Strontium, StrontiumError};
use crate::machine::register::RegisterValue;
use crate::Instruction;

#[derive(Debug, Clone, PartialEq)]
pub struct JumpCExecutor;

impl Executor for JumpCExecutor {
    fn execute(&self, machine: &mut Strontium) -> Result<bool, StrontiumError> {
        if machine.debug {
            println!("JUMPC instruction");
        }
        let instruction = machine.parse_instruction()?;

        if let Instruction::JumpC {
            destination,
            conditional_address,
        } = instruction
        {
            let value = machine.registers.get(&conditional_address).cloned();
            if matches!(value, Some(RegisterValue::Boolean(false))) {
                machine.bytecode_parser.index = destination as usize;
            }
        }

        Ok(true)
    }
}
