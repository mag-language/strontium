use pest::Parser;
use super::machine::instruction::Instruction;

#[derive(Parser)]
#[grammar = "assembler/grammar.pest"] // relative to src
struct AssemblyParser;

pub fn parse(input: String) -> Vec<Instruction> {
	let mut accumulator = vec![];
    let instructions = AssemblyParser::parse(Rule::Program, &input).unwrap_or_else(|e| panic!("{}", e));

    // Because Program is silent, the iterator will contain Instructions
    for ins in instructions {
        // A pair is a combination of the rule which matched and a span of input
        /*println!("Rule:    {:?}", pair.as_rule());
        println!("Span:    {:?}", pair.as_span());
        println!("Text:    {}", pair.as_str());*/

        match ins.as_rule() {
        	Rule::Halt => accumulator.push(Instruction::HALT),
        	Rule::Load => println!("Load"),
        	Rule::Move => println!("Move"),
        	Rule::Copy => println!("Copy"),
        	Rule::Calculate => println!("Calculate"),
        	Rule::Compare => println!("Compare"),
        	Rule::And => println!("AND"),
        	Rule::Or => println!("OR"),
        	Rule::Xor => println!("XOR"),
        	Rule::Not => println!("NOT"),
        	Rule::Lsh => println!("LSH"),
        	Rule::Rsh => println!("RSH"),
        	Rule::Grow => println!("Grow"),
        	Rule::Shrink => println!("Shrink"),
        	Rule::Set => println!("Set"),
        	Rule::Unset => println!("Unset"),

        	_ => unreachable!()
        }
        /*
        // A pair can be converted to an iterator of the tokens which make it up:
        for inner_pair in pair.into_inner() {
            match inner_pair.as_rule() {
                Rule::alpha => println!("Letter:  {}", inner_pair.as_str()),
                Rule::digit => println!("Digit:   {}", inner_pair.as_str()),
                _ => unreachable!()
            };
        }*/


    }

    accumulator
}

#[cfg(test)]
mod tests {
	use super::*;

    #[test]
    fn halt_instruction() {
    	assert_eq!(parse("HALT\nLOAD R1 22".to_string()), vec![Instruction::HALT])
    }
}