use super::{
    BytecodeError,
    Instruction,
    Opcode,
    RegisterValue,
    BitwiseMethod,
};

use std::convert::{TryFrom, TryInto};

#[derive(Debug, Clone)]
pub struct BytecodeParser {
    bytes: Vec<u8>,
    index: usize,
    errors: Vec<BytecodeError>,
}

impl<'a> BytecodeParser {
    pub fn new() -> Self {
        BytecodeParser {
            bytes: vec![],
            index: 0,
            errors: vec![],
        }
    }

    pub fn add_bytecode(&mut self, bytecode: &mut Vec<u8>) {
        self.bytes.append(bytecode);
    }

    /// Advance the parser by one byte if possible.
    fn advance(&mut self) -> Result<(), BytecodeError> {
        if !self.eof() {
            self.index += 1;
            Ok(())
        } else {
            println!("ADVANCE");
            Err(BytecodeError::UnexpectedEof(self.index as u32))
        }
    }

    fn peek(&self) -> u8 {
        self.bytes[self.index]
    }

    /// Check if the parser has reached the end of the bytecode.
    fn eof(&self) -> bool {
        println!("Calling byte parser eof()? index: {}, len: {}", self.index, self.bytes.len());
        self.index >= self.bytes.len()
    }


    /// Consume a number of bytes starting from the current position.
    fn consume_n_bytes(&mut self, n: usize) -> Result<Vec<u8>, BytecodeError> {
        if self.index + n > self.bytes.len() {
            println!("CONSUME_N_BYTES");
            Err(BytecodeError::UnexpectedEof((self.index + n) as u32))
        } else {
            let start = self.index;
            let end = start + n;
            self.index = end;
            Ok(self.bytes[start .. end].to_vec())
        }
    }

    fn consume_byte(&mut self) -> u8 {
        let byte = self.peek();

        if !self.eof() { self.advance().unwrap() };

        byte
    }

    fn consume_u32(&mut self) -> Result<u32, BytecodeError> {
        let bytes = self.consume_n_bytes(4)?;
        Ok(u32::from_le_bytes(bytes.try_into().unwrap()))
    }

    fn consume_string(&mut self) -> Result<String, BytecodeError> {
        let length = self.consume_byte() as usize;
        let bytes = self.consume_n_bytes(length)?;

        match String::from_utf8(bytes.to_vec()) {
            Ok(string) => Ok(string),
            Err(e) => Err(BytecodeError::Utf8Error(e)),
        }
    }

    /// Expect an exact array of bytes starting at the current position.
    fn expect_bytes(&mut self, expected: Vec<u8>) -> Result<(), BytecodeError> {
        let end_index = self.index + expected.len();

        if end_index > self.bytes.len() {
            println!("EXPECT BYTES");
            return Err(BytecodeError::UnexpectedEof(self.index as u32));
        }

        let actual = &self.bytes[self.index .. end_index];

        if actual == expected.as_slice() {
            self.index = end_index;
            Ok(())
        } else {
            Err(BytecodeError::UnexpectedBytes {
                expected,
                actual: actual.to_vec(),
            })
        }
    }

    pub fn parse_instruction(&mut self) -> Result<Instruction, BytecodeError> {
        if self.eof() {
            println!("PARSE INSTRUCTION");
            return Err(BytecodeError::UnexpectedEof(self.index as u32));
        }

        let opcode = Opcode::from(self.bytes[self.index]);
        self.advance()?;

        println!("Opcode: {:?}", opcode);

        let instruction = match opcode {
            Opcode::HALT =>  {
                self.expect_bytes(vec![0, 0, 0, 0, 0, 0, 0])?;
                Instruction::HALT
            },

            Opcode::LOAD => {
                let register = self.consume_string()?;
                println!("Decoded register {}, index {}", register, self.index);
                let value_len = self.consume_byte();
                println!("Decoded value length {}, index {}, parsing register value", value_len, self.index);

                let consumed = self.consume_n_bytes(value_len as usize)?.to_vec();

                println!("consumed: {:?}", consumed);

                let value = RegisterValue::try_from(consumed).unwrap();

                Instruction::LOAD {
                    value,
                    register,
                }
            },

            Opcode::MOVE | Opcode::COPY => {
                let source_len = self.peek();
                self.advance()?;
                let source = self.consume_n_bytes(source_len as usize)?;

                let dest_len = self.peek();
                self.advance()?;
                let destination = self.consume_n_bytes(dest_len as usize)?;

                match opcode {
                    Opcode::MOVE => Instruction::MOVE {
                        source: String::from_utf8(source.to_vec()).unwrap(),
                        destination: String::from_utf8(destination.to_vec()).unwrap(),
                    },
                    Opcode::COPY => Instruction::COPY {
                        source: String::from_utf8(source.to_vec()).unwrap(),
                        destination: String::from_utf8(destination.to_vec()).unwrap(),
                    },
                    _ => unreachable!(),
                }
            },

            Opcode::PUSH => {
                let destination_len = self.peek();
                self.advance()?;
                let destination = self.consume_n_bytes(destination_len as usize)?;

                let value_len = self.peek();
                self.advance()?;
                let value = RegisterValue::try_from(self.consume_n_bytes(value_len as usize)?.to_vec()).unwrap();

                Instruction::PUSH {
                    destination: String::from_utf8(destination.to_vec()).unwrap(),
                    value,
                }
            },

            Opcode::APPEND => {
                let destination_len = self.peek();
                self.advance()?;
                let destination = self.consume_n_bytes(destination_len as usize)?;

                let mut values = vec![];

                while !self.eof() {
                    let value_len = self.peek();
                    self.advance()?;
                    let value = RegisterValue::try_from(self.consume_n_bytes(value_len as usize)?.to_vec()).unwrap();

                    values.push(value);
                }

                Instruction::APPEND {
                    destination: String::from_utf8(destination.to_vec()).unwrap(),
                    value: values,
                }
            },

            Opcode::CALCULATE => {
                let method = self.peek();
                self.advance()?;

                let operand1_len = self.peek();
                self.advance()?;
                let operand1 = self.consume_n_bytes(operand1_len as usize)?;

                let operand2_len = self.peek();
                self.advance()?;
                let operand2 = self.consume_n_bytes(operand2_len as usize)?;

                let destination_len = self.peek();
                self.advance()?;
                let destination = self.consume_n_bytes(destination_len as usize)?;

                Instruction::CALCULATE {
                    method: method.into(),
                    operand1: String::from_utf8(operand1.to_vec()).unwrap(),
                    operand2: String::from_utf8(operand2.to_vec()).unwrap(),
                    destination: String::from_utf8(destination.to_vec()).unwrap(),
                }
            },

            Opcode::COMPARE => {
                let method = self.peek();
                self.advance()?;

                let operand1_len = self.peek();
                self.advance()?;
                let operand1 = self.consume_n_bytes(operand1_len as usize)?;

                let operand2_len = self.peek();
                self.advance()?;
                let operand2 = self.consume_n_bytes(operand2_len as usize)?;

                let destination_len = self.peek();
                self.advance()?;
                let destination = self.consume_n_bytes(destination_len as usize)?;

                Instruction::COMPARE {
                    method: method.into(),
                    operand1: String::from_utf8(operand1.to_vec()).unwrap(),
                    operand2: String::from_utf8(operand2.to_vec()).unwrap(),
                    destination: String::from_utf8(destination.to_vec()).unwrap(),
                }
            },

            Opcode::BITWISE => {
                let method_byte = self.consume_byte();

                let method = match method_byte {
                    0 | 1 | 2 => {
                        let a   = self.consume_string()?;
                        let b   = self.consume_string()?;
                        let out = self.consume_string()?;

                        match method_byte {
                            0 => Ok(BitwiseMethod::AND {
                                a,
                                b,
                                out,
                            }),
                            1 => Ok(BitwiseMethod::OR {
                                a,
                                b,
                                out,
                            }),
                            2 => Ok(BitwiseMethod::XOR {
                                a,
                                b,
                                out,
                            }),
                            _ => unreachable!(),
                        }
                    },
                    3 => Ok(BitwiseMethod::NOT { a: String::new(), out: String::new() }),
                    4 | 5 => {
                        let a   = self.consume_string()?;
                        let out = self.consume_string()?;
                        let amount = self.consume_byte();

                        match method_byte {
                            4 => Ok(BitwiseMethod::LSH {
                                a,
                                out,
                                amount: amount.into(),
                            }),
                            5 => Ok(BitwiseMethod::RSH {
                                a,
                                out,
                                amount: amount.into(),
                            }),
                            _ => unreachable!(),
                        }
                    },
                    _ => Err(BytecodeError::InvalidOpcode(method_byte)),
                };

                Instruction::BITWISE {
                    method: method?,
                }
            },

            Opcode::JUMP => {
                let destination = self.consume_u32()?;
            
                Instruction::JUMP {
                    destination,
                }
            },
            
            Opcode::JUMPC => {
                let destination = self.consume_u32()?;
                let conditional_address = self.consume_string()?;
            
                Instruction::JUMPC {
                    destination,
                    conditional_address,
                }
            },

            _ => return Err(BytecodeError::InvalidOpcode(opcode as u8)),
        };

        Ok(instruction)
    }
}