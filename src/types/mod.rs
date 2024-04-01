use crate::machine::bytecode::BytecodeError;

pub mod value;
pub mod program;

pub use self::value::*;

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
}

impl From<BytecodeError> for StrontiumError {
	fn from(e: BytecodeError) -> Self {
		Self::BytecodeError(e)
	}
}
