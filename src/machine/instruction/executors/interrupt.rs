use super::super::InterruptKind;
use crate::machine::register::RegisterValue;
use crate::machine::{Executor, Strontium};
use crate::types::StrontiumError;
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
                        match value {
                            RegisterValue::Empty => println!("nothing"),
                            value => println!("{}", value),
                        }
                    } else {
                        println!("Invalid register address: {}", interrupt.address);
                    }
                }

                InterruptKind::Panic => {
                    let msg = machine.registers.get(&interrupt.address)
                        .map(|v| format!("{}", v))
                        .unwrap_or_else(|| interrupt.address.clone());
                    let arg_display = machine.registers.get("arg")
                        .map(|v| format!("{:?}({})", v.get_type(), v))
                        .unwrap_or_else(|| "unknown".to_string());
                    return Err(StrontiumError::MethodNotFound(format!("{}: {}", msg, arg_display)));
                }

                _ => {}
            }
        }

        Ok(true)
    }
}
