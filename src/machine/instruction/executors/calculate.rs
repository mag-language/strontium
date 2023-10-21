use num_traits::FromPrimitive;

use crate::machine::{
    Executor,
    Strontium,
    StrontiumError,
};

use crate::machine::register::{RegisterType, RegisterValue};
use super::super::CalculationMethod;

/// Perform a calculation two registers and write the result to a third.
#[derive(Debug, Clone, PartialEq)]
pub struct CalculateExecutor;

impl Executor for CalculateExecutor {
    fn execute(&self, machine: &mut Strontium) -> Result<bool, StrontiumError> {
        let method = machine.consume_byte()?;

        if let Some(method) = CalculationMethod::from_u8(method) {
            let left = machine.consume_string()?;
            let right = machine.consume_string()?;
            let destination = machine.consume_string()?;

            let left_value = machine.registers.get(&left);
            let right_value = machine.registers.get(&right);

            if let (Some(left_value), Some(right_value)) = (left_value, right_value) {
                let result = match method {
                    CalculationMethod::ADD => left_value.clone() + right_value.clone(),
                    _ => {
                        eprintln!("Unsupported register type in bytecode: {:?}", method);
                        RegisterValue::Empty
                    }
                };

                machine.registers.set(&destination, result);
            } else {
                eprintln!("Invalid register type in bytecode: {:?}", method);
            }
        } else {
            eprintln!("Invalid register type in bytecode: {:?}", method);
        }
        if let Some(reg_type) = RegisterType::from_u8(102) {
            println!("Enum variant for 102 is {:?}", reg_type);
        } else {
            println!("Invalid numeric value");
        }
        Ok(true)
    }
}