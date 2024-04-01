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
#[derive(Debug, Clone, PartialEq)]
pub struct Program {
    pub metadata: ProgramMetadata,
    pub bytecode: Bytecode,
}

impl Into<Vec<u8>> for Program {
    fn into(self) -> Vec<u8> {
        let mut bytes = vec![];

        // Push the magic number
        bytes.append(&mut "strontium".as_bytes().to_vec());

        bytes.append(&mut self.metadata.into());
        bytes.append(&mut self.bytecode.bytes.clone());

        bytes
    }
}

impl From<Vec<u8>> for Program {
    fn from(bytes: Vec<u8>) -> Self {
        let mut bytes = bytes.clone();

        // Remove the magic number
        bytes.drain(0 .. "strontium".len());

        let metadata = ProgramMetadata::from(bytes.clone());
        let bytecode = Bytecode {
            bytes: bytes.drain(0 ..).collect(),
        };

        Program {
            metadata,
            bytecode,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ProgramMetadata {
    pub name:        String,
    pub version:     String,
    pub author:      String,
    pub description: String,
}

impl Into<Vec<u8>> for ProgramMetadata {
    fn into(self) -> Vec<u8> {
        let mut bytes = vec![];

        // Push the program name
        bytes.push(self.name.len() as u8);
        bytes.append(&mut self.name.as_bytes().to_vec());

        // Push the program version
        bytes.push(self.version.len() as u8);
        bytes.append(&mut self.version.as_bytes().to_vec());

        // Push the program author
        bytes.push(self.author.len() as u8);
        bytes.append(&mut self.author.as_bytes().to_vec());

        // Push the program description
        bytes.push(self.description.len() as u8);
        bytes.append(&mut self.description.as_bytes().to_vec());

        bytes
    }
}

impl From<Vec<u8>> for ProgramMetadata {
    fn from(bytes: Vec<u8>) -> Self {
        let mut bytes = bytes.clone();

        let name_length = bytes.remove(0) as usize;
        let name = String::from_utf8(bytes.drain(0 .. name_length).collect()).unwrap();

        let version_length = bytes.remove(0) as usize;
        let version = String::from_utf8(bytes.drain(0 .. version_length).collect()).unwrap();

        let author_length = bytes.remove(0) as usize;
        let author = String::from_utf8(bytes.drain(0 .. author_length).collect()).unwrap();

        let description_length = bytes.remove(0) as usize;
        let description = String::from_utf8(bytes.drain(0 .. description_length).collect()).unwrap();

        ProgramMetadata {
            name,
            version,
            author,
            description,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
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