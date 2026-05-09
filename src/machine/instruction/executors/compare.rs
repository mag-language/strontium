use crate::machine::{Executor, Strontium, StrontiumError};
use crate::machine::register::RegisterValue;
use crate::machine::instruction::ComparisonMethod;
use crate::Instruction;

#[derive(Debug, Clone, PartialEq)]
pub struct CompareExecutor;

impl Executor for CompareExecutor {
    fn execute(&self, machine: &mut Strontium) -> Result<bool, StrontiumError> {
        if machine.debug {
            println!("COMPARE instruction");
        }
        let instruction = machine.parse_instruction()?;

        if let Instruction::Compare {
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
                    ComparisonMethod::EQ => op1 == op2,
                    ComparisonMethod::NEQ => op1 != op2,
                    ComparisonMethod::GT => op1 > op2,
                    ComparisonMethod::GTE => op1 >= op2,
                    ComparisonMethod::LT => op1 < op2,
                    ComparisonMethod::LTE => op1 <= op2,
                };
                machine.registers.set(&destination, RegisterValue::Boolean(result));
            }
        }
        Ok(true)
    }
}
