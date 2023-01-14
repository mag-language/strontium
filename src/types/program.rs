use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct Program {
    /// The major version this project was compiled for.
    pub version: u8,
    /// Constant values which may be stored in the header.
    pub constants: Constants,
    /// The program as compiled Strontium bytecode.
    pub bytecode: Vec<u8>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct Constants {
    pub constants: Vec<(String, Constant)>,
}

impl Constants {
    pub fn new(constants: Vec<(String, Constant)>) -> Self {
        Self {
            constants,
        }
    }
}

impl Program {
    pub fn new(
        version: u8,
        constants: Constants,
        bytecode: Vec<u8>,
    ) -> Self {

        Self {
            version,
            constants,
            bytecode,
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();

        // Append the magic "strontium" string to the start of the executable.
        bytes.append(&mut "strontium".as_bytes().to_vec());
        // Serialize the program to bytes.
        bytes.append(&mut bincode::serialize(&self).unwrap());

        bytes
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self, BytecodeError> {
        println!("{}", String::from_utf8_lossy(&bytes[0 .. 9]));
        if "strontium".to_string() == String::from_utf8_lossy(&bytes[0 .. 9]) {
            // Decode a [Program] from the remaining bytes after the signature string.
            Ok(bincode::deserialize(&bytes[9 .. bytes.len()]).unwrap())
        } else {
            Err(BytecodeError::NoSignatureFound)
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
pub enum BytecodeError {
    /// The file ends unexpectedly.
    UnexpectedEOF,
    /// The magic `strontium` ASCII string is not at the beginning of the file.
    NoSignatureFound,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub enum Constant {
    /// A signed 8-bit integer
    Int8(i8),
    /// A signed 16-bit integer
    Int16(i16),
    /// A signed 32-bit integer
    Int32(i32),
    /// A signed 64-bit integer
    Int64(i64),
    /// An unsigned 8-bit integer
    UInt8(u8),
    /// An unsigned 16-bit integer
    UInt16(u16),
    /// An unsigned 32-bit integer
    UInt32(u32),
    /// An unsigned 64-bit integer
    UInt64(u64),
    /// A 32-bit floating point value
    Float32(f32),
    /// A 64-bit floating point value
    Float64(f64),
    /// A sequence of UTF-8 graphemes
    String(String),
}

#[cfg(test)]
mod tests {
	use super::*;

    #[test]
    fn serialize_deserialize() {
 		let program = Program::new(
            14,
            Constants::new(
                vec![
                    ("pi".to_string(), Constant::Float32(3.14159265358)),
                ]
            ),
            vec![],
        );

        let encoded = program.to_bytes();
        let decoded = Program::from_bytes(&encoded[..]).unwrap();

 		assert_eq!(
            decoded,
            Program::new(
                14,
                Constants::new(vec![
                    ("pi".to_string(), Constant::Float32(3.14159265358)),
                ]),
                vec![],
            ),
        );
    }
}