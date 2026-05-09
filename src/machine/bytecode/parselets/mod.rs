use super::decode::BytecodeParser;
use super::BytecodeError;
use crate::machine::instruction::Instruction;

pub trait BytecodeParselet {
    fn parse(&self, parser: &mut BytecodeParser) -> Result<Instruction, BytecodeError>;
}
