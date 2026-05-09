use serde::{Deserialize, Serialize};

use super::{RegisterValue, Strontium};
use crate::machine::opcode::Opcode;
use crate::types::StrontiumError;

mod executors;
pub use self::executors::*;

pub trait Executor {
    fn execute(&self, machine: &mut Strontium) -> Result<bool, StrontiumError>;
}

/// A pattern for multimethod dispatch - either a specific value or a wildcard.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DispatchPattern {
    /// Match a specific value (e.g., Int64(0) for fib(0))
    Value(RegisterValue),
    /// Match any value (variable pattern like fib(n))
    Any,
}

impl DispatchPattern {
    /// Check if this pattern matches the given value.
    pub fn matches(&self, value: &RegisterValue) -> bool {
        match self {
            DispatchPattern::Value(v) => v == value,
            DispatchPattern::Any => true,
        }
    }

    /// Get the precedence of this pattern (higher = more specific).
    pub fn precedence(&self) -> usize {
        match self {
            DispatchPattern::Value(_) => 2,
            DispatchPattern::Any => 1,
        }
    }
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
    /*
        /// Define a function and add it to the dispatch table.
        DEF {
            /// The name of the multimethod implementation.
            name: String,
            /// A pair, tuple, record or value type descriptor as a single argument.
            arg_type: String,
            /// The return type of the function.
            return_type: String,
            /// The position in the bytecode buffer where the function starts.
            bytecode_start: usize,
        },
    */
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

    /// Dispatch a multimethod call based on runtime argument value.
    /// Reads `arg` register, matches against dispatch table, jumps to winner.
    Dispatch {
        /// The name of the multimethod to dispatch.
        method_name: String,
    },
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
            Instruction::Dispatch { .. } => Opcode::Dispatch,
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
