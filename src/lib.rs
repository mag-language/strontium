//! ![Strontium](https://s3.fr-par.scw.cloud/strontium.dev/banner_slim.svg)
//!
//! # Introduction
//!
//! Strontium is a bytecode interpreter for static and dynamic programming languages.
//!
//! It is built primarily to support Mag as the underlying, executive portion of the language
//! engine. Any Mag source code first runs through the compiler, is then compiled to Strontium
//! bytecode and finally interpreted by this virtual machine.
//!
//! ## A small example
//!
//! Performing a simple arithmetic operation like `2 + 3` can be expressed like this
//! using Strontium instructions (and an assembler which does not exist yet):
//!
//! ```ignore
//! LOAD::Int64 r1 2
//! LOAD::Int64 r2 3
//! ADD r1 r2 r3
//! PRINT r3
//! ```
//!
//! This will load both numbers into registers `r1` and `r2`, add them together and store the
//! result in register `r3`. Finally, the result is printed to standard output.
pub mod types;
pub mod machine;
// pub mod repl;

pub use self::machine::Strontium;
pub use self::machine::instruction::Instruction;