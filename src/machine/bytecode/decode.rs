use super::{BitwiseMethod, BytecodeError, Instruction, Opcode, RegisterValue};

use crate::machine::Interrupt;

use std::convert::{TryFrom, TryInto};

pub struct BytecodeParser {
    /// A reference to the contents of the `bc` register, which contains program bytecode.
    pub bytecode: Vec<u8>,
    /// The current position in the bytecode register.
    pub index: usize,
    /// Used to accumulate errors while parsing.
    errors: Vec<BytecodeError>,
    /// Whether to print debug output.
    pub debug: bool,
}

impl BytecodeParser {
    pub fn new(bytecode: Vec<u8>, debug: bool) -> Self {
        BytecodeParser {
            bytecode,
            index: 0,
            errors: vec![],
            debug,
        }
    }

    pub fn set_bytecode(&mut self, bytecode: Vec<u8>) {
        self.bytecode = bytecode;
        self.index = 0;
        self.errors = vec![];
    }

    /// Advance the parser by one byte if possible.
    fn advance(&mut self) -> Result<(), BytecodeError> {
        if !self.eof() {
            self.index += 1;
            Ok(())
        } else {
            Err(BytecodeError::UnexpectedEof(self.index as u32))
        }
    }

    fn peek(&self) -> u8 {
        self.bytecode[self.index]
    }

    /// Check if the parser has reached the end of the bytecode.
    fn eof(&self) -> bool {
        self.index >= self.bytecode.len()
    }

    /// Consume a number of bytes starting from the current position.
    fn consume_n_bytes(&mut self, n: usize) -> Result<Vec<u8>, BytecodeError> {
        if self.index + n > self.bytecode.len() {
            Err(BytecodeError::UnexpectedEof(self.index as u32))
        } else {
            let start = self.index;
            let end = start + n;
            self.index = end;
            if self.debug {
                println!("End: {}", end);
            }
            Ok(self.bytecode[start..end].to_vec())
        }
    }

    fn consume_byte(&mut self) -> u8 {
        let byte = self.peek();

        if !self.eof() {
            self.advance().unwrap()
        };

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

        if end_index > self.bytecode.len() {
            return Err(BytecodeError::UnexpectedEof(self.index as u32));
        }

        let actual = &self.bytecode[self.index..end_index];

        if actual.to_vec() == expected.as_slice() {
            self.index = end_index;
            Ok(())
        } else {
            Err(BytecodeError::UnexpectedBytes {
                expected,
                actual: actual.to_vec(),
            })
        }
    }

    pub fn value_to_byte(&self, value: RegisterValue) -> u8 {
        if let RegisterValue::UInt8(byte) = value {
            byte
        } else {
            unreachable!()
        }
    }

    pub fn values_to_bytes(&self, values: Vec<RegisterValue>) -> Vec<u8> {
        values
            .iter()
            .filter_map(|val| {
                if let RegisterValue::UInt8(byte) = val {
                    Some(*byte)
                } else {
                    None // Or handle differently, maybe even panic if you expect only UInt8 values.
                }
            })
            .collect()
    }

    pub fn parse_instruction(&mut self) -> Result<Instruction, BytecodeError> {
        if self.eof() {
            return Err(BytecodeError::UnexpectedEof(self.index as u32));
        };

        let opcode = Opcode::from(self.bytecode[self.index].clone());
        self.advance()?;

        let instruction = match opcode {
            Opcode::Halt => {
                if self.debug {
                    println!(":: HALT");
                }
                self.expect_bytes(vec![0, 0, 0, 0, 0, 0, 0])?;
                Instruction::Halt
            }

            Opcode::Load => {
                if self.debug {
                    println!(":: LOAD");
                }
                let register = self.consume_string()?;
                if self.debug {
                    println!("Decoded register {}, index {}", register, self.index);
                }
                let value_len = self.consume_byte();
                if self.debug {
                    println!(
                        "Decoded value length {}, index {}, parsing register value",
                        value_len, self.index
                    );
                }

                let consumed = self.consume_n_bytes(value_len as usize)?.to_vec();

                if self.debug {
                    println!("consumed: {:?}", consumed);
                }

                let value = RegisterValue::try_from(consumed).unwrap();

                Instruction::Load { value, register }
            }

            Opcode::Move | Opcode::Copy => {
                if self.debug {
                    println!(":: MOVE/COPY");
                }
                let source_len = self.peek();
                self.advance()?;
                let source = self.consume_n_bytes(source_len as usize)?;

                let dest_len = self.peek();
                self.advance()?;
                let destination = self.consume_n_bytes(dest_len as usize)?;

                match opcode {
                    Opcode::Move => Instruction::Move {
                        source: String::from_utf8(source.to_vec()).unwrap(),
                        destination: String::from_utf8(destination.to_vec()).unwrap(),
                    },
                    Opcode::Copy => Instruction::Copy {
                        source: String::from_utf8(source.to_vec()).unwrap(),
                        destination: String::from_utf8(destination.to_vec()).unwrap(),
                    },
                    _ => unreachable!(),
                }
            }

            Opcode::Push => {
                if self.debug {
                    println!(":: PUSH");
                }
                let destination_len = self.peek();
                self.advance()?;
                let destination = self.consume_n_bytes(destination_len as usize)?;

                let value_len = self.peek();
                self.advance()?;
                let value =
                    RegisterValue::try_from(self.consume_n_bytes(value_len as usize)?.to_vec())
                        .unwrap();

                Instruction::Push {
                    destination: String::from_utf8(destination.to_vec()).unwrap(),
                    value,
                }
            }

            Opcode::Append => {
                if self.debug {
                    println!(":: APPEND");
                }
                let destination_len = self.peek();
                self.advance()?;
                let destination = self.consume_n_bytes(destination_len as usize)?;

                let mut values = vec![];

                while !self.eof() {
                    let value_len = self.peek();
                    self.advance()?;
                    let value =
                        RegisterValue::try_from(self.consume_n_bytes(value_len as usize)?.to_vec())
                            .unwrap();

                    values.push(value);
                }

                Instruction::Append {
                    destination: String::from_utf8(destination.to_vec()).unwrap(),
                    value: values,
                }
            }

            Opcode::Interrupt => {
                if self.debug {
                    println!(":: INTERRUPT");
                }
                let interrupt = self.consume_byte();
                let address = self.consume_string();

                Instruction::Interrupt {
                    interrupt: Interrupt {
                        address: address?,
                        kind: interrupt.into(),
                    },
                }
            }

            Opcode::Calculate => {
                if self.debug {
                    println!(":: CALCULATE");
                }
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

                Instruction::Calculate {
                    method: method.into(),
                    operand1: String::from_utf8(operand1.to_vec()).unwrap(),
                    operand2: String::from_utf8(operand2.to_vec()).unwrap(),
                    destination: String::from_utf8(destination.to_vec()).unwrap(),
                }
            }

            Opcode::Compare => {
                if self.debug {
                    println!(":: COMPARE");
                }
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

                Instruction::Compare {
                    method: method.into(),
                    operand1: String::from_utf8(operand1.to_vec()).unwrap(),
                    operand2: String::from_utf8(operand2.to_vec()).unwrap(),
                    destination: String::from_utf8(destination.to_vec()).unwrap(),
                }
            }

            Opcode::Bitwise => {
                if self.debug {
                    println!(":: BITWISE");
                }
                let method_byte = self.consume_byte();

                let method = match method_byte {
                    0 | 1 | 2 => {
                        let a = self.consume_string()?;
                        let b = self.consume_string()?;
                        let out = self.consume_string()?;

                        match method_byte {
                            0 => Ok(BitwiseMethod::AND { a, b, out }),
                            1 => Ok(BitwiseMethod::OR { a, b, out }),
                            2 => Ok(BitwiseMethod::XOR { a, b, out }),
                            _ => unreachable!(),
                        }
                    }
                    3 => Ok(BitwiseMethod::NOT {
                        a: String::new(),
                        out: String::new(),
                    }),
                    4 | 5 => {
                        let a = self.consume_string()?;
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
                    }
                    _ => Err(BytecodeError::InvalidOpcode(method_byte)),
                };

                Instruction::Bitwise { method: method? }
            }

            Opcode::Jump => {
                if self.debug {
                    println!(":: JUMP");
                }
                let destination = self.consume_u32()?;

                Instruction::Jump { destination }
            }

            Opcode::JumpC => {
                if self.debug {
                    println!(":: JUMPC");
                }
                let destination = self.consume_u32()?;
                let conditional_address = self.consume_string()?;

                Instruction::JumpC {
                    destination,
                    conditional_address,
                }
            }

            Opcode::Call => {
                if self.debug {
                    println!(":: CALL");
                }
                let address = self.consume_u32()? as usize;
                Instruction::Call { address }
            }

            Opcode::Return => {
                if self.debug {
                    println!(":: RETURN");
                }
                Instruction::Return
            }

            Opcode::StoreLocal => {
                if self.debug {
                    println!(":: StoreLocal");
                }
                let name = self.consume_string()?;
                let register = self.consume_string()?;
                Instruction::StoreLocal { name, register }
            }

            Opcode::LoadLocal => {
                if self.debug {
                    println!(":: LoadLocal");
                }
                let name = self.consume_string()?;
                let register = self.consume_string()?;
                Instruction::LoadLocal { name, register }
            }

            Opcode::Dispatch => {
                if self.debug {
                    println!(":: DISPATCH");
                }
                let method_name = self.consume_string()?;
                Instruction::Dispatch { method_name }
            }

            _ => return Err(BytecodeError::InvalidOpcode(opcode as u8)),
        };

        Ok(instruction)
    }
}
