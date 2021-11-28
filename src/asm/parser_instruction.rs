use nom::{
    character::complete::{multispace0, multispace1},
    sequence::tuple,
    IResult,
};

use crate::asm::parser_op::*;
use crate::asm::parser_operand::integer_operand;
use crate::asm::parser_reg::register;
use crate::asm::Token;

#[derive(Debug, PartialEq)]
pub struct AssemblerInstruction {
    pub opcode: Token,
    pub operand1: Option<Token>,
    pub operand2: Option<Token>,
    pub operand3: Option<Token>,
}

impl AssemblerInstruction {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut res = vec![];
        match self.opcode {
            Token::Op { code } => res.push(code as u8),
            _ => {
                println!("Non-opcode found in opcode field");
                std::process::exit(1)
            }
        };
        for op in vec![&self.operand1, &self.operand2, &self.operand3] {
            match op {
                Some(op) => AssemblerInstruction::extract_operand(op, &mut res),
                None => {}
            }
        }
        return res;
    }

    fn extract_operand(t: &Token, res: &mut Vec<u8>) {
        match t {
            Token::Reg { reg } => res.push(*reg),
            Token::IntegerOperand { i } => {
                let v = *i as u16;
                let byte1 = v;
                let byte2 = v >> 8;
                res.push(byte2 as u8);
                res.push(byte1 as u8);
            }
            _ => {
                println!("Non-operand found in operand field");
                std::process::exit(1)
            }
        }
    }
}

pub fn instruction_one(input: &str) -> IResult<&str, AssemblerInstruction> {
    let input = input.trim();
    let (input, (o, _, r, _, i, _)) = tuple((
        opcode_load,
        multispace1,
        register,
        multispace0,
        integer_operand,
        multispace0,
    ))(input)?;
    Ok((
        input,
        AssemblerInstruction {
            opcode: o,
            operand1: Some(r),
            operand2: Some(i),
            operand3: None,
        },
    ))
}

mod tests {
    use super::*;
    use crate::instruction::Opcode;

    #[test]
    fn test_parse_instruction_one() {
        assert_eq!(
            instruction_one("load  $0     #100 \n"),
            Ok((
                "",
                AssemblerInstruction {
                    opcode: Token::Op { code: Opcode::LOAD },
                    operand1: Some(Token::Reg { reg: 0 }),
                    operand2: Some(Token::IntegerOperand { i: 100 }),
                    operand3: None,
                }
            ))
        )
    }
}
