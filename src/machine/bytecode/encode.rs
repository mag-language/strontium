use crate::Instruction;
use crate::machine::BitwiseMethod;

impl Into<Vec<u8>> for BitwiseMethod {
	fn into(self) -> Vec<u8> {
        match self {
            BitwiseMethod::AND { ref a, ref b, ref out }
            | BitwiseMethod::OR { ref a, ref b, ref out }
            | BitwiseMethod::XOR { ref a, ref b, ref out } => {
                let mut bytes = vec![];

                // Push the method as a single byte
                bytes.push(self.get_method_byte());

                bytes.push(a.as_bytes().to_vec().len() as u8);
                bytes.append(&mut a.as_bytes().to_vec());

                bytes.push(b.as_bytes().to_vec().len() as u8);
                bytes.append(&mut b.as_bytes().to_vec());

                bytes.push(out.as_bytes().to_vec().len() as u8);
                bytes.append(&mut out.as_bytes().to_vec());

                bytes
            },

            BitwiseMethod::LSH { ref a, ref out, amount} | BitwiseMethod::RSH { ref a, ref out, amount} => {
                let mut bytes = vec![];

                // Push the method as a single byte
                bytes.push(self.get_method_byte());

                bytes.push(a.as_bytes().to_vec().len() as u8);
                bytes.append(&mut a.as_bytes().to_vec());

                bytes.push(out.as_bytes().to_vec().len() as u8);
                bytes.append(&mut out.as_bytes().to_vec());

                bytes.push(amount as u8);

                bytes
            }
        
            BitwiseMethod::NOT { ref a, ref out} => {
                let mut bytes = vec![];

                // Push the method as a single byte
                bytes.push(self.get_method_byte());

                bytes.push(a.as_bytes().to_vec().len() as u8);
                bytes.append(&mut a.as_bytes().to_vec());

                bytes.push(out.as_bytes().to_vec().len() as u8);
                bytes.append(&mut out.as_bytes().to_vec());

                bytes
            }
        }
    }
}

impl Into<Vec<u8>> for Instruction {
    fn into(self) -> Vec<u8> {
        let mut bytes = vec![];

        let opcode = self.get_opcode();

        bytes.push(opcode.into());

        let mut data: Vec<u8> = match self {
            // Add seven zeroes for a total of eight zero bytes.
            Instruction::HALT => vec![0, 0, 0, 0, 0, 0, 0],
            Instruction::LOAD { value, register } => {
                let mut b = vec![];

                b.push(register.len() as u8);
                b.append(&mut register.as_bytes().to_vec());

                let value_bytes: Vec<u8> = value.into();
                //println!("Value bytes: {:?}", value_bytes);
                //println!("Value bytes len: {:?}", value_bytes.len());
                b.push(value_bytes.len() as u8);
                b.append(&mut value_bytes[0 .. value_bytes.len()].to_vec());

                b
            },

            Instruction::MOVE { source, destination }
            | Instruction::COPY { source, destination } => {
                let mut b = vec![];

                b.push(source.len() as u8);
                b.append(&mut source.as_bytes().to_vec());

                b.push(destination.len() as u8);
                b.append(&mut destination.as_bytes().to_vec());

                b
            },

            Instruction::PUSH { value, destination } => {
                let mut b = vec![];

                b.push(destination.len() as u8);
                b.append(&mut destination.as_bytes().to_vec());

                let mut value_bytes: Vec<u8> = value.into();

                b.push(value_bytes.len() as u8);
                b.append(&mut value_bytes);

                b
            },

            Instruction::APPEND { value, destination } => {
                let mut b = vec![];

                b.push(destination.len() as u8);
                b.append(&mut destination.as_bytes().to_vec());

                for v in value {
                    let mut v_bytes: Vec<u8> = v.into();
                    b.push(v_bytes.len() as u8);
                    b.append(&mut v_bytes);
                }

                b
            },

            Instruction::CALCULATE { method, operand1, operand2, destination } => {
                let mut b = vec![];

                b.push(method.into());

                b.push(operand1.len() as u8);
                b.append(&mut operand1.as_bytes().to_vec());

                b.push(operand2.len() as u8);
                b.append(&mut operand2.as_bytes().to_vec());

                b.push(destination.len() as u8);
                b.append(&mut destination.as_bytes().to_vec());

                b
            },

            Instruction::COMPARE {
                method,
                operand1,
                operand2,
                destination,
            } => {
                let mut b = vec![];

                b.push(method.into());

                b.push(operand1.len() as u8);
                b.append(&mut operand1.as_bytes().to_vec());

                b.push(operand2.len() as u8);
                b.append(&mut operand2.as_bytes().to_vec());

                b.push(destination.len() as u8);
                b.append(&mut destination.as_bytes().to_vec());

                b
            },

            Instruction::BITWISE {
                method,
            } => {
                let mut bytes = vec![];

                bytes.push(method.clone().get_method_byte());

                match method {
                    BitwiseMethod::AND { a, b, out }
                    | BitwiseMethod::OR { a, b, out }
                    | BitwiseMethod::XOR { a, b, out } => {
                        bytes.push(a.len() as u8);
                        bytes.append(&mut a.as_bytes().to_vec());

                        bytes.push(b.len() as u8);
                        bytes.append(&mut b.as_bytes().to_vec());

                        bytes.push(out.len() as u8);
                        bytes.append(&mut out.as_bytes().to_vec());
                    },

                    BitwiseMethod::LSH { a, out, amount }
                    | BitwiseMethod::RSH { a, out, amount } => {
                        bytes.push(a.len() as u8);
                        bytes.append(&mut a.as_bytes().to_vec());

                        bytes.push(out.len() as u8);
                        bytes.append(&mut out.as_bytes().to_vec());

                        bytes.push(amount as u8);
                    },

                    BitwiseMethod::NOT { a, out } => {
                        bytes.push(a.len() as u8);
                        bytes.append(&mut a.as_bytes().to_vec());

                        bytes.push(out.len() as u8);
                        bytes.append(&mut out.as_bytes().to_vec());
                    },
                }

                bytes
            },

            Instruction::JUMP { destination } => {
                let mut b = vec![];

                b.append(&mut (destination as u32).to_le_bytes().to_vec());

                b
            },

            Instruction::JUMPC { destination, conditional_address } => {
                let mut b = vec![];

                b.append(&mut (destination as u32).to_le_bytes().to_vec());

                b.push(conditional_address.len() as u8);
                b.append(&mut conditional_address.as_bytes().to_vec());

                b
            },

            Instruction::INTERRUPT { interrupt } => {
                let mut b = vec![];

                b.push(interrupt.kind.into());

                b.push(interrupt.address.len() as u8);
                b.append(&mut interrupt.address.as_bytes().to_vec());

                b
            },

            _ => vec![],
        };

        bytes.append(&mut data);

        bytes
    }
}
