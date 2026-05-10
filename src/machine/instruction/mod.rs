use serde::{Deserialize, Serialize};

use super::{RegisterValue, Strontium};
use crate::machine::opcode::Opcode;
use crate::types::StrontiumError;

mod executors;
pub use self::executors::*;

pub trait Executor {
    fn execute(&self, machine: &mut Strontium) -> Result<bool, StrontiumError>;
}


/// A signal indicating that an event needs immediate attention. This enumeration
/// contains the interrupt types supported by the virtual machine.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Interrupt {
    /// The address to print to or read from
    pub address: String,
    pub kind: InterruptKind,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum InterruptKind {
    Print,
    Read,
}

impl Into<u8> for InterruptKind {
    fn into(self) -> u8 {
        match self {
            InterruptKind::Print => 0,
            InterruptKind::Read => 1,
        }
    }
}

impl From<u8> for InterruptKind {
    fn from(byte: u8) -> InterruptKind {
        match byte {
            0 => InterruptKind::Print,
            1 => InterruptKind::Read,
            _ => unreachable!(),
        }
    }
}

/// Represents a callable machine instruction
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Instruction {
    /// Stop all further execution
    Halt,

    /// Load a numeric value into a register
    Load {
        value: RegisterValue,
        register: String,
    },

    /// Move a value from one register address to another
    Move {
        source: String,
        destination: String,
    },

    /// Copy a value from a register to memory or vice versa
    Copy {
        source: String,
        destination: String,
    },

    /// Add a value to an array in a register
    Push {
        /// The value to be pushed
        value: RegisterValue,
        /// The name of the array register to append to
        destination: String,
    },

    /// Add a list of values to an array in a register
    Append {
        /// The values to append
        value: Vec<RegisterValue>,
        /// The name of the array register to append to
        destination: String,
    },

    /// Perform a calculation on two registers and write the result to a third
    Calculate {
        // The type of calculation to perform
        method: CalculationMethod,
        // Left side operand as a named register address
        operand1: String,
        // Right side operand as a named register address
        operand2: String,
        // Output register
        destination: String,
    },

    // Compare two registers and write the result (`0` or `1`) into a third
    Compare {
        method: ComparisonMethod,
        operand1: String,
        operand2: String,
        destination: String,
    },

    // Perform a memory operation (`and`, `or`, `xor`, `not`, `lsh`, `rsh`)
    Bitwise {
        method: BitwiseMethod,
    },

    /// Set the program counter to the given value
    Jump {
        destination: u32,
    },

    /// Set the program counter to a value if the given byte has the value of `1`
    JumpC {
        destination: u32,
        conditional_address: String,
    },

    /// Set off an interrupt, for example to print a character to standard output
    Interrupt {
        interrupt: Interrupt,
    },
    Call {
        /// The address to jump to (the start of the method body).
        address: usize,
    },

    Return,

    /// Store a value from a register into the current stack frame's local variables.
    StoreLocal {
        /// The name of the local variable.
        name: String,
        /// The register to read the value from.
        register: String,
    },

    /// Load a value from the current stack frame's local variables into a register.
    LoadLocal {
        /// The name of the local variable.
        name: String,
        /// The register to store the value in.
        register: String,
    },

    /// Load the runtime type tag of a register's value into another register as Int64.
    LoadType {
        source: String,
        destination: String,
    },

    // Compile-time-only pseudo-instructions. These are resolved to real instructions
    // in link_bytecode and must never reach the encoding stage.
    LabelTarget { id: usize },
    JumpToLabel { id: usize },
    JumpCToLabel { id: usize, conditional_address: String },
    /// Call a named dispatch shim; resolved to Call { address } during linking.
    CallShim { method_name: String },
}

impl Instruction {
    pub fn get_opcode(&self) -> Opcode {
        match self {
            Instruction::Halt => Opcode::Halt,
            Instruction::Load { .. } => Opcode::Load,
            Instruction::Move { .. } => Opcode::Move,
            Instruction::Copy { .. } => Opcode::Copy,
            Instruction::Calculate { .. } => Opcode::Calculate,
            Instruction::Compare { .. } => Opcode::Compare,
            Instruction::Bitwise { .. } => Opcode::Bitwise,
            Instruction::Jump { .. } => Opcode::Jump,
            Instruction::JumpC { .. } => Opcode::JumpC,
            Instruction::Interrupt { .. } => Opcode::Interrupt,
            Instruction::Call { .. } => Opcode::Call,
            Instruction::Return { .. } => Opcode::Return,
            Instruction::Push { .. } => Opcode::Push,
            Instruction::Append { .. } => Opcode::Append,
            Instruction::StoreLocal { .. } => Opcode::StoreLocal,
            Instruction::LoadLocal { .. } => Opcode::LoadLocal,
            Instruction::LoadType { .. } => Opcode::LoadType,
            Instruction::LabelTarget { .. }
            | Instruction::JumpToLabel { .. }
            | Instruction::JumpCToLabel { .. }
            | Instruction::CallShim { .. } => {
                unreachable!("compile-time pseudo-instruction reached get_opcode")
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum BytecodeError {
    UnexpectedEof,
    InvalidOpcode(u8),
}

/// Basic arithmetic operations which act on two registers and write the result to a third.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CalculationMethod {
    ADD,
    SUBTRACT,
    MULTIPLY,
    DIVIDE,
    POWER,
    MODULO,
    SQRT,
    CONCAT,
}

impl Into<u8> for CalculationMethod {
    fn into(self) -> u8 {
        match self {
            CalculationMethod::ADD => 0,
            CalculationMethod::SUBTRACT => 1,
            CalculationMethod::MULTIPLY => 2,
            CalculationMethod::DIVIDE => 3,
            CalculationMethod::POWER => 4,
            CalculationMethod::MODULO => 5,
            CalculationMethod::SQRT => 6,
            CalculationMethod::CONCAT => 7,
        }
    }
}

impl From<u8> for CalculationMethod {
    fn from(byte: u8) -> CalculationMethod {
        match byte {
            0 => CalculationMethod::ADD,
            1 => CalculationMethod::SUBTRACT,
            2 => CalculationMethod::MULTIPLY,
            3 => CalculationMethod::DIVIDE,
            4 => CalculationMethod::POWER,
            5 => CalculationMethod::MODULO,
            6 => CalculationMethod::SQRT,
            7 => CalculationMethod::CONCAT,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ComparisonMethod {
    EQ,
    NEQ,
    GT,
    GTE,
    LT,
    LTE,
}

impl Into<u8> for ComparisonMethod {
    fn into(self) -> u8 {
        match self {
            ComparisonMethod::EQ => 0,
            ComparisonMethod::NEQ => 1,
            ComparisonMethod::GT => 2,
            ComparisonMethod::GTE => 3,
            ComparisonMethod::LT => 4,
            ComparisonMethod::LTE => 5,
        }
    }
}

impl From<u8> for ComparisonMethod {
    fn from(byte: u8) -> ComparisonMethod {
        match byte {
            0 => ComparisonMethod::EQ,
            1 => ComparisonMethod::NEQ,
            2 => ComparisonMethod::GT,
            3 => ComparisonMethod::GTE,
            4 => ComparisonMethod::LT,
            5 => ComparisonMethod::LTE,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub enum BitwiseMethod {
    AND { a: String, b: String, out: String },

    OR { a: String, b: String, out: String },

    XOR { a: String, b: String, out: String },

    NOT { a: String, out: String },

    LSH { a: String, out: String, amount: u32 },

    RSH { a: String, out: String, amount: u32 },
}

impl BitwiseMethod {
    pub fn get_method_byte(&self) -> u8 {
        match self {
            BitwiseMethod::AND { .. } => 0,
            BitwiseMethod::OR { .. } => 1,
            BitwiseMethod::XOR { .. } => 2,
            BitwiseMethod::NOT { .. } => 3,
            BitwiseMethod::LSH { .. } => 4,
            BitwiseMethod::RSH { .. } => 5,
        }
    }
}
