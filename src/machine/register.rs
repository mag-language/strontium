use std::collections::HashMap;

pub enum Reserved {
    Bc,
    Ip,
    Fp,
    Sp,
}

impl Reserved {
    pub fn index(&self) -> usize {
        match self {
            Reserved::Bc => 0,
            Reserved::Ip => 1,
            Reserved::Fp => 2,
            Reserved::Sp => 3,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Registers {
    /// A set of registers which can be resized dynamically.
    pub registers: Vec<RegisterValue>,
}

impl Registers {
    pub fn new() -> Self {
        // The number of total registers to allocate at initalization.
        let size = 16;

        // The program bytecode as a list of `UInt8`s
        let bc = RegisterValue::Array(vec![]);
        // The instruction pointer, which points to the next instruction in the bytecode.
        let ip = RegisterValue::UInt(0);
        // The frame pointer, which points to the current index in the stack frame.
        let fp = RegisterValue::UInt(0);
        // The stack pointer, which points to the current index in the stack.
        let sp = RegisterValue::UInt(0);

        let mut registers = vec![
            bc,
            ip,
            fp,
            sp,
        ];

        let mut i = 0;

        while i < size {
            registers.push(RegisterValue::Empty);
            i += 1;
        }

        Self {
            registers,
        }
    }

    pub fn get(&self, index: usize) -> Option<&RegisterValue> {
        self.registers.get(index)
    }

    pub fn set(&mut self, index: usize, value: RegisterValue) {
        if index < self.registers.len() {
            self.registers[index] = value;
        } else {
            // Possibly resize the Vec if needed
        }
    }

    pub fn get_reserved(&self, r: Reserved) -> &RegisterValue {
        let index = r as usize;
        // Safe unwrap with always-on reserved registers.
        self.get(index).unwrap()
    }

    pub fn set_reserved(&mut self, r: Reserved, value: RegisterValue) {
        let index = r.index();
        self.set(index, value);
    }

    pub fn ip(&self) -> usize {
        match self.get_reserved(Reserved::Ip) {
			RegisterValue::UInt(ip) => *ip as usize,
			_ => unreachable!(),
		}
    }

    pub fn bc(&self) -> Vec<u8> {
        match self.get_reserved(Reserved::Bc) {
			RegisterValue::Array(instructions) => instructions
                .iter()
                .map(|reg_value| {
                    match reg_value {
                        RegisterValue::UInt8(byte) => *byte,
			            _ => unreachable!(),
                    }
                })
                .collect(),
			_ => unreachable!(),
		}
    }
}

#[derive(Debug, Clone)]
pub enum RegisterValue {
    Empty,
    Int8(i8),
    Int16(i16),
    Int32(i32),
    Int(i64),
    UInt8(u8),
    UInt16(u16),
    UInt32(u32),
    UInt(u64),
    Float32(f32),
    Float(f64),
    String(String),
    Boolean(bool),
    Map(HashMap<String, RegisterValue>),
    Array(Vec<RegisterValue>),
}

impl RegisterValue {
    pub fn get_type(&self) -> RegisterType {
        match self {
            Self::Empty      => RegisterType::Empty,
            Self::Int8(_)    => RegisterType::Int8,
            Self::Int16(_)   => RegisterType::Int16,
            Self::Int32(_)   => RegisterType::Int32,
            Self::Int(_)     => RegisterType::Int,
            Self::UInt8(_)   => RegisterType::UInt8,
            Self::UInt16(_)  => RegisterType::UInt16,
            Self::UInt32(_)  => RegisterType::UInt32,
            Self::UInt(_)    => RegisterType::UInt,
            Self::Float32(_) => RegisterType::Float32,
            Self::Float(_)   => RegisterType::Float,
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
    Int,
    UInt8,
    UInt16,
    UInt32,
    UInt,
    Float32,
    Float,
    String,
    Boolean,
    Map,
    Array,
}