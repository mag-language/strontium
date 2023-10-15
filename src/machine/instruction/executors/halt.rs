use crate::machine::{Executor, Strontium, StrontiumError};

#[derive(Debug, Clone, PartialEq)]
/// Stop all code execution immediately.
///
/// This is a general purpose instruction commonly used to signal the end of processing or a transition
/// to a different state for error handling, debugging breakpoints, power saving and other purposes.
pub struct HaltExecutor;

impl Executor for HaltExecutor {
    fn execute(&self, _machine: &mut Strontium) -> Result<bool, StrontiumError> {
        Ok(false)
    }
}