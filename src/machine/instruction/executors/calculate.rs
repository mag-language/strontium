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
            let source = machine.registers.get(&operand1);
            let target = machine.registers.get(&operand2);

            if let (Some(source), Some(target)) = (source, target) {
                let result = match method {
                    CalculationMethod::ADD => source.clone() + target.clone(),
                    CalculationMethod::SUBTRACT => source.clone() - target.clone(),
                    CalculationMethod::MULTIPLY => source.clone() * target.clone(),
                    CalculationMethod::DIVIDE => source.clone() / target.clone(),
/*
                    CalculationMethod::Modulo => source % target,
                    CalculationMethod::Power => source.pow(target),
*/
                    _ => unimplemented!(),
                };

                machine.registers.set(&destination, result);
            }
        }
        Ok(true)
    }
}