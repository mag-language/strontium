use super::decode::BytecodeParser;
use crate::machine::instruction::Instruction;
use super::BytecodeError;

pub trait BytecodeParselet {
    fn parse(&self, parser: &mut BytecodeParser) -> Result<Instruction, BytecodeError>;
}