use crate::machine::{Executor, Strontium, StrontiumError};
use super::super::InterruptKind;
use num_traits::FromPrimitive;

/// Attend to an event that needs immediate attention.
#[derive(Debug, Clone, PartialEq)]
pub struct InterruptExecutor;

impl Executor for InterruptExecutor {
    fn execute(&self, machine: &mut Strontium) -> Result<bool, StrontiumError> {
        let kind = machine.consume_byte()?;

        if let Some(interrupt_kind) = InterruptKind::from_u8(kind) {
            match interrupt_kind {
                InterruptKind::Print => {
                    let address = machine.consume_string()?;
                    let value = machine.registers.get(&address);
                    if let Some(value) = value {
                        println!("{:?}", value);
                    } else {
                        println!("Invalid register address: {}", address);
                    }
                },
                /*InterruptKind::Read => {
                    let address = machine.consume_string()?;
                    let value = machine.registers.get(&address);
                    if let Some(value) = value {
                        println!("{:?}: ", value);
                        let mut input = String::new();
                        std::io::stdin().read_line(&mut input).unwrap();
                        machine.registers.set(&address, input.trim().to_string());
                    } else {
                        println!("Invalid register address: {}", address);
                    }
                },*/
                _ => unimplemented!(),
            }
        } else {
            println!("Invalid numeric value");
        }
        Ok(true)
    }
}