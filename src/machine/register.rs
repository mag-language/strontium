use num_derive::{FromPrimitive, ToPrimitive};
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};

use std::convert::TryFrom;
use std::io::{Cursor, Read};
use std::collections::{HashMap, BTreeMap};
use std::string::String;
use std::ops::{Add, Sub, Mul, Div};
use serde::{Serialize, Deserialize};

use self::RegisterValue::*;

#[derive(Debug, Clone)]
pub struct Registers {
    /// A set of registers which can be resized dynamically.
    pub registers: BTreeMap<String, RegisterValue>,
}

impl Registers {
    pub fn new() -> Self {
        let mut registers = BTreeMap::new();

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

    pub fn allocate_register(&mut self) -> String {
        // Collect the names of empty registers
        let empty_registers: Vec<String> = self.registers
            .iter()
            .filter_map(|(name, value)| {
                if matches!(value, RegisterValue::Empty) {
                    Some(name.clone())
                } else {
                    None
                }
            })
            .collect();

        // If an empty register is found, mark it as occupied and return its name
        if let Some(name) = empty_registers.first() {
            self.registers.insert(name.clone(), UInt64(0));
            return name.clone();
        }

        // If no empty register is found, create a new one
        let new_register_name = format!("r{}", self.registers.len() + 1);
        self.registers.insert(new_register_name.clone(), UInt64(0)); // Mark as occupied
        new_register_name
    }

    // Add a method to free a register
    pub fn free_register(&mut self, name: &str) {
        self.registers.insert(name.to_string(), RegisterValue::Empty);
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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

impl Into<Vec<u8>> for RegisterValue {
    fn into(self) -> Vec<u8> {
        let mut bytes = Vec::new();
        match self {
            RegisterValue::Empty => {
                bytes.push(1);
            },
            RegisterValue::Int8(i) => {
                bytes.push(2); // Type tag for Int32
                bytes.extend_from_slice(&i.to_le_bytes());
            },
            RegisterValue::Int16(i) => {
                bytes.push(3); // Type tag for Int32
                bytes.extend_from_slice(&i.to_le_bytes());
            },
            RegisterValue::Int32(i) => {
                bytes.push(4); // Type tag for Int32
                bytes.extend_from_slice(&i.to_le_bytes());
            },
            RegisterValue::Int64(i) => {
                bytes.push(5); // Type tag for Int32
                bytes.extend_from_slice(&i.to_le_bytes());
            },
            RegisterValue::UInt8(i) => {
                bytes.push(6); // Type tag for UInt8
                bytes.extend_from_slice(&i.to_le_bytes());
            },
            RegisterValue::UInt16(i) => {
                bytes.push(7); // Type tag for UInt16
                bytes.extend_from_slice(&i.to_le_bytes());
            },
            RegisterValue::UInt32(i) => {
                bytes.push(8); // Type tag for UInt32
                bytes.extend_from_slice(&i.to_le_bytes());
            },
            RegisterValue::UInt64(i) => {
                bytes.push(9); // Type tag for UInt64
                bytes.extend_from_slice(&i.to_le_bytes());
            },
            RegisterValue::Float32(f) => {
                bytes.push(10); // Type tag for Float64
                bytes.extend_from_slice(&f.to_le_bytes());
            },
            RegisterValue::Float64(f) => {
                bytes.push(11); // Type tag for Float64
                bytes.extend_from_slice(&f.to_le_bytes());
            },
            RegisterValue::String(s) => {
                bytes.push(12); // Type tag for String
                let string_bytes = s.into_bytes();
                let length = string_bytes.len() as u32;
                bytes.extend_from_slice(&length.to_le_bytes());
                bytes.extend_from_slice(&string_bytes);
            },
            RegisterValue::Boolean(b) => {
                bytes.push(4); // Type tag for Boolean
                bytes.push(b as u8);
            },
            RegisterValue::Map(_) => {
                println!("map register type is unimplemented");
            },
            RegisterValue::Array(_) => {
                println!("array register type is unimplemented");
            },
        }
        bytes
    }
}

impl TryFrom<Vec<u8>> for RegisterValue {
    type Error = RegisterValueError;

    fn try_from(mut bytes: Vec<u8>) -> Result<Self, Self::Error> {
        if bytes.is_empty() {
            return Err(RegisterValueError::NotEnoughBytes);
        }

        let type_tag = bytes.remove(0);
        let mut cursor = Cursor::new(bytes);

        match type_tag {
            1 => Ok(RegisterValue::Empty),
            2 => Ok(RegisterValue::Int8(cursor.read_i8().map_err(|_| RegisterValueError::NotEnoughBytes)?)),
            3 => Ok(RegisterValue::Int16(cursor.read_i16::<LittleEndian>().map_err(|_| RegisterValueError::NotEnoughBytes)?)),
            4 => Ok(RegisterValue::Int32(cursor.read_i32::<LittleEndian>().map_err(|_| RegisterValueError::NotEnoughBytes)?)),
            5 => Ok(RegisterValue::Int64(cursor.read_i64::<LittleEndian>().map_err(|_| RegisterValueError::NotEnoughBytes)?)),
            6 => Ok(RegisterValue::UInt8(cursor.read_u8().map_err(|_| RegisterValueError::NotEnoughBytes)?)),
            7 => Ok(RegisterValue::UInt16(cursor.read_u16::<LittleEndian>().map_err(|_| RegisterValueError::NotEnoughBytes)?)),
            8 => Ok(RegisterValue::UInt32(cursor.read_u32::<LittleEndian>().map_err(|_| RegisterValueError::NotEnoughBytes)?)),
            9 => Ok(RegisterValue::UInt64(cursor.read_u64::<LittleEndian>().map_err(|_| RegisterValueError::NotEnoughBytes)?)),
            10 => Ok(RegisterValue::Float32(cursor.read_f32::<LittleEndian>().map_err(|_| RegisterValueError::NotEnoughBytes)?)),
            11 => Ok(RegisterValue::Float64(cursor.read_f64::<LittleEndian>().map_err(|_| RegisterValueError::NotEnoughBytes)?)),
            12 => {
                let length = cursor.read_u32::<LittleEndian>().map_err(|_| RegisterValueError::NotEnoughBytes)?;
                let mut string_bytes = vec![0; length as usize];
                cursor.read_exact(&mut string_bytes).map_err(|_| RegisterValueError::NotEnoughBytes)?;
                let string = String::from_utf8(string_bytes).map_err(RegisterValueError::Utf8Error)?;
                Ok(RegisterValue::String(string))
            },
            13 => Ok(RegisterValue::Boolean(cursor.read_u8().map_err(|_| RegisterValueError::NotEnoughBytes)? != 0)),
            _ => Err(RegisterValueError::InvalidTypeTag),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum RegisterValueError {
    InvalidTypeTag,
    NotEnoughBytes,
    Utf8Error(std::string::FromUtf8Error),
    // Add other error types as needed
}

impl Add for RegisterValue {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        match (self, other) {
            (Self::Int8(a), Self::Int8(b)) => Self::Int8(a.wrapping_add(b)),
            (Self::Int16(a), Self::Int16(b)) => Self::Int16(a.wrapping_add(b)),
            (Self::Int32(a), Self::Int32(b)) => Self::Int32(a.wrapping_add(b)),
            (Self::Int64(a), Self::Int64(b)) => Self::Int64(a.wrapping_add(b)),
            (Self::UInt8(a), Self::UInt8(b)) => Self::UInt8(a.wrapping_add(b)),
            (Self::UInt16(a), Self::UInt16(b)) => Self::UInt16(a.wrapping_add(b)),
            (Self::UInt32(a), Self::UInt32(b)) => Self::UInt32(a.wrapping_add(b)),
            (Self::UInt64(a), Self::UInt64(b)) => Self::UInt64(a.wrapping_add(b)),
            (Self::Float32(a), Self::Float32(b)) => Self::Float32(a + b),
            (Self::Float64(a), Self::Float64(b)) => Self::Float64(a + b),
            _ => panic!("Incompatible types for addition"),
        }
    }
}

impl Sub for RegisterValue {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        match (self, other) {
            (Self::Int8(a), Self::Int8(b)) => Self::Int8(a.wrapping_sub(b)),
            (Self::Int16(a), Self::Int16(b)) => Self::Int16(a.wrapping_sub(b)),
            (Self::Int32(a), Self::Int32(b)) => Self::Int32(a.wrapping_sub(b)),
            (Self::Int64(a), Self::Int64(b)) => Self::Int64(a.wrapping_sub(b)),
            (Self::UInt8(a), Self::UInt8(b)) => Self::UInt8(a.wrapping_sub(b)),
            (Self::UInt16(a), Self::UInt16(b)) => Self::UInt16(a.wrapping_sub(b)),
            (Self::UInt32(a), Self::UInt32(b)) => Self::UInt32(a.wrapping_sub(b)),
            (Self::UInt64(a), Self::UInt64(b)) => Self::UInt64(a.wrapping_sub(b)),
            (Self::Float32(a), Self::Float32(b)) => Self::Float32(a - b),
            (Self::Float64(a), Self::Float64(b)) => Self::Float64(a - b),
            _ => panic!("Incompatible types for subtraction"),
        }
    }
}

impl Mul for RegisterValue {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        match (self, other) {
            (Self::Int8(a), Self::Int8(b)) => Self::Int8(a.wrapping_mul(b)),
            (Self::Int16(a), Self::Int16(b)) => Self::Int16(a.wrapping_mul(b)),
            (Self::Int32(a), Self::Int32(b)) => Self::Int32(a.wrapping_mul(b)),
            (Self::Int64(a), Self::Int64(b)) => Self::Int64(a.wrapping_mul(b)),
            (Self::UInt8(a), Self::UInt8(b)) => Self::UInt8(a.wrapping_mul(b)),
            (Self::UInt16(a), Self::UInt16(b)) => Self::UInt16(a.wrapping_mul(b)),
            (Self::UInt32(a), Self::UInt32(b)) => Self::UInt32(a.wrapping_mul(b)),
            (Self::UInt64(a), Self::UInt64(b)) => Self::UInt64(a.wrapping_mul(b)),
            (Self::Float32(a), Self::Float32(b)) => Self::Float32(a * b),
            (Self::Float64(a), Self::Float64(b)) => Self::Float64(a * b),
            _ => panic!("Incompatible types for multiplication"),
        }
    }
}

impl Div for RegisterValue {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        match (self, other) {
            (Self::Int8(a), Self::Int8(b)) => Self::Int8(a.wrapping_div(b)),
            (Self::Int16(a), Self::Int16(b)) => Self::Int16(a.wrapping_div(b)),
            (Self::Int32(a), Self::Int32(b)) => Self::Int32(a.wrapping_div(b)),
            (Self::Int64(a), Self::Int64(b)) => Self::Int64(a.wrapping_div(b)),
            (Self::UInt8(a), Self::UInt8(b)) => Self::UInt8(a.wrapping_div(b)),
            (Self::UInt16(a), Self::UInt16(b)) => Self::UInt16(a.wrapping_div(b)),
            (Self::UInt32(a), Self::UInt32(b)) => Self::UInt32(a.wrapping_div(b)),
            (Self::UInt64(a), Self::UInt64(b)) => Self::UInt64(a.wrapping_div(b)),
            (Self::Float32(a), Self::Float32(b)) => Self::Float32(a / b),
            (Self::Float64(a), Self::Float64(b)) => Self::Float64(a / b),
            _ => panic!("Incompatible types for division"),
        }
    }
}

impl std::fmt::Display for RegisterValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RegisterValue::Empty => write!(f, "Empty"),
            RegisterValue::Int8(i) => write!(f, "{}", i),
            RegisterValue::Int16(i) => write!(f, "{}", i),
            RegisterValue::Int32(i) => write!(f, "{}", i),
            RegisterValue::Int64(i) => write!(f, "{}", i),
            RegisterValue::UInt8(i) => write!(f, "{}", i),
            RegisterValue::UInt16(i) => write!(f, "{}", i),
            RegisterValue::UInt32(i) => write!(f, "{}", i),
            RegisterValue::UInt64(i) => write!(f, "{}", i),
            RegisterValue::Float32(n) => write!(f, "{}", n),
            RegisterValue::Float64(n) => write!(f, "{}", n),
            RegisterValue::String(s) => write!(f, "{}", s),
            RegisterValue::Boolean(b) => write!(f, "{}", b),
            RegisterValue::Map(m) => write!(f, "{:?}", m),
            RegisterValue::Array(a) => write!(f, "{:?}", a),
        }
    }
}

/// Define possible register types which support conversion to byte values for encoding.
#[derive(Debug, PartialEq, ToPrimitive, FromPrimitive)]
pub enum RegisterType {
    Empty   = 100,
    Int8    = 101,
    Int16   = 102,
    Int32   = 103,
    Int64   = 104,
    UInt8   = 105,
    UInt16  = 106,
    UInt32  = 107,
    UInt64  = 108,
    Float32 = 109,
    Float64 = 110,
    String  = 111,
    Boolean = 112,
    Map     = 113,
    Array   = 114,
}