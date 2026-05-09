use crate::machine::{Executor, Strontium, StrontiumError};
use crate::machine::register::RegisterValue;
use super::super::CalculationMethod;
use crate::Instruction;

/// Perform a calculation two registers and write the result to a third.
#[derive(Debug, Clone, PartialEq)]
pub struct CalculateExecutor;

impl Executor for CalculateExecutor {
    fn execute(&self, machine: &mut Strontium) -> Result<bool, StrontiumError> {
        if machine.debug {
            println!("CALCULATE instruction");
        }
        let instruction = machine.parse_instruction()?;

        if let Instruction::Calculate {
            method,
            operand1,
            operand2,
            destination,
        } = instruction
        {
            let op1 = machine.registers.get(&operand1);
            let op2 = machine.registers.get(&operand2);

            if let (Some(op1), Some(op2)) = (op1, op2) {
                let result = match method {
                    CalculationMethod::ADD => op1.clone() + op2.clone(),
                    CalculationMethod::SUBTRACT => op1.clone() - op2.clone(),
                    CalculationMethod::MULTIPLY => op1.clone() * op2.clone(),
                    CalculationMethod::DIVIDE => op1.clone() / op2.clone(),
                    CalculationMethod::MODULO => op1.clone() % op2.clone(),
                    CalculationMethod::POWER => match (op1.clone(), op2.clone()) {
                        (RegisterValue::Int64(a), RegisterValue::Int64(b)) => RegisterValue::Int64(a.pow(b as u32)),
                        (RegisterValue::Float64(a), RegisterValue::Float64(b)) => RegisterValue::Float64(a.powf(b)),
                        _ => panic!("Incompatible types for power"),
                    },
                };

                machine.registers.set(&destination, result);
            }
        }
        Ok(true)
    }
}
