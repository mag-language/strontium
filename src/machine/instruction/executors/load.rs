use crate::machine::{
    Executor,
    Strontium,
    StrontiumError,
};

use crate::Instruction;

/// Load a dynamically typed value into a named register.
///
/// This not only includes atomic values like integers and floats, but also more complex types like
/// arrays, records and methods, simplifying the implementation of a compiler for Strontium.
#[derive(Debug, Clone, PartialEq)]
pub struct LoadExecutor;

impl Executor for LoadExecutor {
    fn execute(&self, machine: &mut Strontium, instruction: Instruction) -> Result<bool, StrontiumError> {
        if let Instruction::LOAD { value, register } = instruction {
            machine.registers.set(&register, value);
        }

        Ok(true)
    }
}