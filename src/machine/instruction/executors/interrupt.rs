use super::super::InterruptKind;
use crate::machine::register::RegisterValue;
use crate::machine::{Executor, Strontium, StrontiumError};
use crate::Instruction;

/// Attend to an event that needs immediate attention.
#[derive(Debug, Clone, PartialEq)]
pub struct InterruptExecutor;

impl Executor for InterruptExecutor {
    fn execute(&self, machine: &mut Strontium) -> Result<bool, StrontiumError> {
        if machine.debug {
            println!("INTERRUPT");
            println!("INTERRUPT :: Parsing expression");
        }
        let instruction = machine.parse_instruction()?;

        if machine.debug {
            println!("INTERRUPT :: Parsed expression");
        }
        if let Instruction::Interrupt { interrupt } = instruction {
            match interrupt.kind {
                InterruptKind::Print => {
                    if machine.debug {
                        println!("INTERRUPT :: Got InterruptKind::Print");
                    }
                    let value = machine.registers.get(&interrupt.address);
                    if let Some(value) = value {
                        if !matches!(value, RegisterValue::Empty) {
                            println!("{}", value);
                        }
                    } else {
                        println!("Invalid register address: {}", interrupt.address);
                    }
                }

                _ => {}
            }
        }

        Ok(true)
    }
}
