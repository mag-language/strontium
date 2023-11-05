use std::convert::{TryFrom, TryInto};
use crate::Instruction;
use crate::machine::{Opcode, BitwiseMethod};
use crate::machine::register::{
    RegisterValue,
    RegisterValueError,
};

pub mod encode;
pub mod decode;
pub mod parselets;

#[cfg(test)] mod tests;

/// A single bytecode program with metadata.
pub struct Program {
    pub metadata: ProgramMetadata,
    pub bytecode: Bytecode,
}

pub struct ProgramMetadata {
    pub name:        String,
    pub version:     String,
    pub author:      String,
    pub description: String,
}

pub struct Bytecode {
    pub bytes: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum BytecodeError {
    UnexpectedEof(u32),            // Unexpected end of bytecode
    UnexpectedBytes {
        expected: Vec<u8>,
        actual: Vec<u8>,
    },                        // Unexpected bytes encountered
    InvalidOpcode(u8),        // Invalid opcode encountered
    InvalidOperand,           // Invalid operand encountered
    OperandTypeMismatch,      // Mismatched operand type
    Utf8Error(std::string::FromUtf8Error), // UTF-8 error
    RegisterNotFound(String), // Referenced register not found
    Other(String),            // Other error with a description
}

impl std::fmt::Display for BytecodeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BytecodeError::UnexpectedEof(pos) => write!(f, "Unexpected end of bytecode at position {}", pos),
            BytecodeError::UnexpectedBytes { expected, actual } => write!(f, "Unexpected bytes: expected {:?}, got {:?}", expected, actual),
            BytecodeError::InvalidOpcode(opcode) => write!(f, "Invalid opcode: {}", opcode),
            BytecodeError::InvalidOperand => write!(f, "Invalid operand"),
            BytecodeError::OperandTypeMismatch => write!(f, "Operand type mismatch"),
            BytecodeError::Utf8Error(e) => write!(f, "UTF-8 error: {}", e),
            BytecodeError::RegisterNotFound(register) => write!(f, "Register not found: {}", register),
            BytecodeError::Other(description) => write!(f, "{}", description),
        }
    }
}

impl std::error::Error for BytecodeError {}