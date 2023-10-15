//! ![Strontium](https://s3.fr-par.scw.cloud/strontium.dev/banner_slim.svg)
//!
//! Strontium is a bytecode interpreter for static and dynamic programming languages.
//!
//! It is built primarily to support Mag as the underlying, executive portion of the language engine.
//! Any Mag source code first runs through the compiler, is then compiled to Strontium bytecode and
//! finally interpreted by this virtual machine.
pub mod types;
pub mod machine;
pub mod repl;

pub use self::machine::Strontium;
// pub use self::machine::instruction::Instruction;