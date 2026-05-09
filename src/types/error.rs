use crate::machine::bytecode::BytecodeError;

#[derive(Debug, PartialEq)]
pub enum StrontiumError {
    /// A division by zero has occured.
    DivisionByZero,
    /// An invalid memory or register address has been accessed.
    OutOfBounds,
    BytecodeError(BytecodeError),
    /// The machine encountered an invalid operation code.
    IllegalOpcode(u8),
    /// The bytecode sequence has ended unexpectedly.
    UnexpectedEof,
    /// The type of a register does not match the provided value.
    TypeMismatch,
    InvalidUtf8String,
    EmptyCallStack,
    /// The user has not called `strontium.init()`.
    Uninitialized,
    /// A register with the given name was not found.
    RegisterNotFound(String),
    /// A local variable with the given name was not found in the current stack frame.
    LocalVariableNotFound(String),
    /// No matching method was found in the dispatch table.
    MethodNotFound(String),
    /// Execution was cancelled by the embedding runtime.
    Interrupted,
}

impl From<BytecodeError> for StrontiumError {
    fn from(e: BytecodeError) -> Self {
        Self::BytecodeError(e)
    }
}
