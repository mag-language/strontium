use num_traits::FromPrimitive;

use crate::machine::{
    Executor,
    Strontium,
    StrontiumError,
};

use crate::machine::register::{RegisterType, RegisterValue};

/// Load a dynamically typed value into a named register.
///
/// This not only includes atomic values like integers and floats, but also more complex types like
/// arrays, records and methods, simplifying the implementation of a compiler for Strontium.
#[derive(Debug, Clone, PartialEq)]
pub struct LoadExecutor;

impl Executor for LoadExecutor {
    fn execute(&self, machine: &mut Strontium) -> Result<bool, StrontiumError> {
        let register_address = machine.consume_string()?;
        let register_type_byte = machine.consume_byte()?;

        if let Some(register_type) = RegisterType::from_u8(register_type_byte as u8) {
            let value = match register_type {
                RegisterType::Empty => RegisterValue::Empty,
                RegisterType::Int8 => RegisterValue::Int8(machine.consume_i8()?),
                RegisterType::Int16 => RegisterValue::Int16(machine.consume_i16()?),
                RegisterType::Int32 => RegisterValue::Int32(machine.consume_i32()?),
                RegisterType::Int64 => RegisterValue::Int64(machine.consume_i64()?),
                RegisterType::UInt8 => RegisterValue::UInt8(machine.consume_u8()?),
                RegisterType::UInt16 => RegisterValue::UInt16(machine.consume_u16()?),
                RegisterType::UInt32 => RegisterValue::UInt32(machine.consume_u32()?),
                RegisterType::UInt64 => RegisterValue::UInt64(machine.consume_u64()?),
                RegisterType::Float32 => RegisterValue::Float32(machine.consume_f32()?),
                RegisterType::Float64 => RegisterValue::Float64(machine.consume_f64()?),
                RegisterType::String => RegisterValue::String(machine.consume_string()?),
                RegisterType::Boolean => RegisterValue::Boolean(machine.consume_bool()?),
                _ => {
                    eprintln!("Unsupported register type in bytecode: {}", register_type_byte);
                    RegisterValue::Empty
                }
            };

            machine.registers.set(&register_address, value);
        } else {
            eprintln!("Invalid register type in bytecode: {}", register_type_byte);
        }
        Ok(true)
    }
}