pub mod value;
pub mod program;

#[derive(Debug, PartialEq)]
pub enum StrontiumError {
	/// A division by zero has occured.
	DivisionByZero,
	/// An invalid memory or register address has been accessed.
	OutOfBounds,
	/// The machine encountered an invalid operation code.
	IllegalOpcode(u8),
	/// The bytecode sequence has ended unexpectedly.
	UnexpectedEof,
	/// The type of a register does not match the provided value.
	TypeMismatch,
}
