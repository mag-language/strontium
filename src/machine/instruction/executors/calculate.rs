use crate::machine::{
    Executor,
    Strontium,
    StrontiumError,
};

use crate::Instruction;
use super::super::CalculationMethod;

/// Perform a calculation two registers and write the result to a third.
#[derive(Debug, Clone, PartialEq)]
pub struct CalculateExecutor;

impl Executor for CalculateExecutor {
    fn execute(&self, machine: &mut Strontium, instruction: Instruction) -> Result<bool, StrontiumError> {
        if let Instruction::CALCULATE { method, operand1, operand2, destination } = instruction {
            let op1 = machine.registers.get(&operand1);
            let op2 = machine.registers.get(&operand2);

            if let (Some(op1), Some(op2)) = (op1, op2) {
                let result = match method {
                    CalculationMethod::ADD => op1.clone() + op2.clone(),
                    CalculationMethod::SUBTRACT => op1.clone() - op2.clone(),
                    CalculationMethod::MULTIPLY => op1.clone() * op2.clone(),
                    CalculationMethod::DIVIDE => op1.clone() / op2.clone(),
/*
                    CalculationMethod::Modulo => op1 % op2,
                    CalculationMethod::Power => op1.pow(op2),
*/
                    _ => unimplemented!(),
                };

                machine.registers.set(&destination, result);
            }
        }
        Ok(true)
    }
}