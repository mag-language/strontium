use crate::machine::instruction::{
    CalculationMethod,
    ComparisonMethod,
};
use super::*;
use super::decode::BytecodeParser;

#[test]
/// Decode and re-encode a halt instruction.
fn transcode_halt() {
    let ins = Instruction::HALT;
    let mut decoded: Vec<u8> = ins.clone().into();

    assert_eq!(decoded.len(), 8);
    let mut parser = BytecodeParser::new();
    parser.add_bytecode(&mut decoded);

    assert_eq!(parser.parse_instruction().unwrap(), ins);
}

#[test]
/// Decode and re-encode a halt instruction.
fn transcode_load() {
    let ins = Instruction::LOAD {
        register: "r5".to_string(),
        value: RegisterValue::UInt64(42),
    };
    let mut decoded: Vec<u8> = ins.clone().into();
    println!("Decoded: {:?}", decoded);

    let mut parser = BytecodeParser::new();
    parser.add_bytecode(&mut decoded);

    assert_eq!(parser.parse_instruction().unwrap(), ins);
}


#[test]
fn transcode_move() {
    let ins = Instruction::MOVE {
        source: "r5".to_string(),
        destination: "r7".to_string(),
    };
    let mut decoded: Vec<u8> = ins.clone().into();
    let mut parser = BytecodeParser::new();
    parser.add_bytecode(&mut decoded);

    assert_eq!(parser.parse_instruction().unwrap(), ins);
}

#[test]
fn transcode_copy() {
    let ins = Instruction::COPY {
        source: "r2".to_string(),
        destination: "r9".to_string(),
    };
    let mut decoded: Vec<u8> = ins.clone().into();
    let mut parser = BytecodeParser::new();
    parser.add_bytecode(&mut decoded);

    assert_eq!(parser.parse_instruction().unwrap(), ins);
}

#[test]
fn transcode_push() {
    let ins = Instruction::PUSH {
        destination: "r9".to_string(),
        value: RegisterValue::Float32(2.2),
    };
    let mut decoded: Vec<u8> = ins.clone().into();
    let mut parser = BytecodeParser::new();
    parser.add_bytecode(&mut decoded);

    assert_eq!(parser.parse_instruction().unwrap(), ins);
}

#[test]
fn transcode_append() {
    let ins = Instruction::APPEND {
        destination: "r9".to_string(),
        value: vec![
            RegisterValue::Float32(2.2),
            RegisterValue::Float32(3.1415)
        ],
    };
    let mut decoded: Vec<u8> = ins.clone().into();
    let mut parser = BytecodeParser::new();
    parser.add_bytecode(&mut decoded);

    assert_eq!(parser.parse_instruction().unwrap(), ins);
}

#[test]
fn transcode_calculate() {
    let ins = Instruction::CALCULATE {
        method: CalculationMethod::ADD,
        operand1: "r2".to_string(),
        operand2: "r3".to_string(),
        destination: "r4".to_string(),
    };
    let mut decoded: Vec<u8> = ins.clone().into();
    let mut parser = BytecodeParser::new();
    parser.add_bytecode(&mut decoded);

    assert_eq!(parser.parse_instruction().unwrap(), ins);
}

#[test]
fn transcode_compare() {
    let ins = Instruction::COMPARE {
        method: ComparisonMethod::GT,
        operand1: "r2".to_string(),
        operand2: "r3".to_string(),
        destination: "r4".to_string(),
    };
    let mut decoded: Vec<u8> = ins.clone().into();
    let mut parser = BytecodeParser::new();
    parser.add_bytecode(&mut decoded);

    assert_eq!(parser.parse_instruction().unwrap(), ins);
}

#[test]
fn transcode_bitwise_and() {
    let ins = Instruction::BITWISE { method: 
        BitwiseMethod::AND {
            a: "r2".to_string(),
            b: "r3".to_string(),
            out: "r4".to_string(),
        }
    };
    let mut decoded: Vec<u8> = ins.clone().into();
    let mut parser = BytecodeParser::new();
    parser.add_bytecode(&mut decoded);

    assert_eq!(parser.parse_instruction().unwrap(), ins);
}

#[test]
fn transcode_jump() {
    let ins = Instruction::JUMP {
        destination: 42,
    };
    let mut decoded: Vec<u8> = ins.clone().into();

    assert_eq!(decoded.len(), 5);
    let mut parser = BytecodeParser::new();
    parser.add_bytecode(&mut decoded);

    assert_eq!(parser.parse_instruction().unwrap(), ins);
}

#[test]
fn transcode_jumpc() {
    let ins = Instruction::JUMPC {
        destination: 42,
        conditional_address: "r5".to_string(),
    };
    let mut decoded: Vec<u8> = ins.clone().into();

    assert_eq!(decoded.len(), 8);
    let mut parser = BytecodeParser::new();
    parser.add_bytecode(&mut decoded);

    assert_eq!(parser.parse_instruction().unwrap(), ins);
}