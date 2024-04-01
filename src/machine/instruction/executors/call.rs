use crate::machine::{
    Executor,
    Strontium,
    StrontiumError, StackFrame,
};

use crate::Instruction;

use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct CallExecutor;

impl Executor for CallExecutor {
    fn execute(&self, machine: &mut Strontium) -> Result<bool, StrontiumError> {
        println!("CALL instruction");
        let instruction = machine.parse_instruction()?;

        if let Instruction::CALL { address } = instruction {
            let return_address = machine.ip;
            machine.call_stack.push(StackFrame {
                return_address,
                local_variables: HashMap::new(),
            });
            machine.ip = address;
        }

        Ok(true)
    }
}