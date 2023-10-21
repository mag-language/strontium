use std::collections::HashMap;
use std::string::String;

use self::RegisterValue::*;

#[derive(Debug, Clone)]
pub struct Registers {
    /// A set of registers which can be resized dynamically.
    pub registers: HashMap<String, RegisterValue>,
}

impl Registers {
    pub fn new() -> Self {
        let mut registers = HashMap::new();

        // Create instruction pointer.
        registers.insert("ip".to_string(), UInt64(0));
        // Create array containing program bytecode.
        registers.insert("bc".to_string(), Array(Vec::new()));

        registers.insert("r1".to_string(), Empty);
        registers.insert("r2".to_string(), Empty);
        registers.insert("r3".to_string(), Empty);
        registers.insert("r4".to_string(), Empty);
        registers.insert("r5".to_string(), Empty);
        registers.insert("r6".to_string(), Empty);
        registers.insert("r7".to_string(), Empty);
        registers.insert("r8".to_string(), Empty);

        Self {
            registers,
        }
    }

    pub fn set(&mut self, name: &str, value: RegisterValue) {
        self.registers.insert(name.to_string(), value);
    }

    pub fn get(&self, name: &str) -> Option<&RegisterValue> {
        self.registers.get(name)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum RegisterValue {
    Empty,
    /// Signed integer, 8 bit
    Int8(i8),
    /// Signed integer, 16 bit
    Int16(i16),
    /// Signed integer, 32 bit
    Int32(i32),
    /// Signed integer, 64 bit
    Int64(i64),
    /// Unsigned integer, 8 bit
    UInt8(u8),
    /// Unsigned integer, 16 bit
    UInt16(u16),
    /// Unsigned integer, 32 bit
    UInt32(u32),
    /// Unsigned integer, 64 bit
    UInt64(u64),
    /// Floating point number, 32 bit
    Float32(f32),
    /// Floating point number, 64 bit
    Float64(f64),
    /// UTF-8 string
    String(String),
    /// `true` or `false`
    Boolean(bool),
    /// A key-value assignment of strings and values.
    Map(HashMap<String, RegisterValue>),
    /// A linear sequence of values.
    Array(Vec<RegisterValue>),
}

impl RegisterValue {
    pub fn get_type(&self) -> RegisterType {
        match self {
            Self::Empty      => RegisterType::Empty,
            Self::Int8(_)    => RegisterType::Int8,
            Self::Int16(_)   => RegisterType::Int16,
            Self::Int32(_)   => RegisterType::Int32,
            Self::Int64(_)   => RegisterType::Int64,
            Self::UInt8(_)   => RegisterType::UInt8,
            Self::UInt16(_)  => RegisterType::UInt16,
            Self::UInt32(_)  => RegisterType::UInt32,
            Self::UInt64(_)  => RegisterType::UInt64,
            Self::Float32(_) => RegisterType::Float32,
            Self::Float64(_) => RegisterType::Float64,
            Self::String(_)  => RegisterType::String,
            Self::Boolean(_) => RegisterType::Boolean,
            Self::Map(_)     => RegisterType::Map,
            Self::Array(_)   => RegisterType::Array,
        }
    }
}


pub enum RegisterType {
    Empty,
    Int8,
    Int16,
    Int32,
    Int64,
    UInt8,
    UInt16,
    UInt32,
    UInt64,
    Float32,
    Float64,
    String,
    Boolean,
    Map,
    Array,
}