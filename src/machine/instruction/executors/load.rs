use crate::machine::{
    Executor,
    RegisterValue,
    Strontium,
    StrontiumError,
};

#[derive(Debug, Clone, PartialEq)]
/// Load a dynamically typed value into a named register.
///
/// This not only includes atomic values like integers and floats, but also more complex types like
/// arrays, records and methods, simplifying the implementation of a compiler for Strontium.
pub struct LoadExecutor;

impl Executor for LoadExecutor {
    fn execute(&self, machine: &mut Strontium) -> Result<bool, StrontiumError> {
        
        Ok(true)
    }
}