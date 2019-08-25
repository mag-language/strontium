#![feature(custom_attribute)]

#[macro_use] extern crate pest_derive;

pub mod assembler;
pub mod types;
pub mod machine;

pub use self::machine::Strontium;
pub use self::machine::instruction::Instruction;