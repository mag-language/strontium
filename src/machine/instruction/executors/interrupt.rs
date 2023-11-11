use crate::machine::{Executor, Strontium, StrontiumError};
use crate::Instruction;
use super::super::InterruptKind;

/// Attend to an event that needs immediate attention.
#[derive(Debug, Clone, PartialEq)]
pub struct InterruptExecutor;

impl Executor for InterruptExecutor {
    fn execute(&self, machine: &mut Strontium) -> Result<bool, StrontiumError> {
        let instruction = machine.parse_instruction()?;

        if let Instruction::INTERRUPT { address: _, interrupt } = instruction {
            match interrupt.kind {
                InterruptKind::Print => {
                    let value = machine.registers.get(&interrupt.address);
                    if let Some(value) = value {
                        println!("{}", value);
                    } else {
                        println!("Invalid register address: {}", interrupt.address);
                    }
                },

                _ => {},
            }
        }

        Ok(true)
    }
}