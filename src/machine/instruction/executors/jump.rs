use crate::machine::{Executor, Strontium, StrontiumError};

use crate::Instruction;

#[derive(Debug, Clone, PartialEq)]
pub struct JumpExecutor;

impl Executor for JumpExecutor {
    fn execute(&self, machine: &mut Strontium) -> Result<bool, StrontiumError> {
        if machine.debug {
            println!("JUMP instruction");
        }
        let instruction = machine.parse_instruction()?;

        if let Instruction::Jump { destination } = instruction {
            // Set the bytecode parser index to the destination address
            machine.bytecode_parser.index = destination as usize;
        }

        Ok(true)
    }
}
