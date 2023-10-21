use crate::machine::{
    Executor,
    Strontium,
    StrontiumError,
};

/// Load a dynamically typed value into a named register.
///
/// This not only includes atomic values like integers and floats, but also more complex types like
/// arrays, records and methods, simplifying the implementation of a compiler for Strontium.
#[derive(Debug, Clone, PartialEq)]
pub struct LoadExecutor;

impl Executor for LoadExecutor {
    fn execute(&self, _machine: &mut Strontium) -> Result<bool, StrontiumError> {
        
        Ok(true)
    }
}