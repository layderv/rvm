use nom::{
    branch::alt, character::complete::multispace0, character::complete::space0, combinator::opt,
    sequence::tuple, IResult,
};

use crate::asm::parser_directive::*;
use crate::asm::parser_label::{label_declaration, label_usage};
use crate::asm::parser_op::*;
use crate::asm::parser_operand::{integer_operand, operand};
use crate::asm::parser_reg::register;
use crate::asm::SymbolTable;
use crate::asm::Token;

#[derive(Debug, PartialEq)]
pub struct AssemblerInstruction {
    pub directive: Option<Token>,
    pub label: Option<Token>,
    pub opcode: Option<Token>,
    pub operand1: Option<Token>,
    pub operand2: Option<Token>,
    pub operand3: Option<Token>,
}

impl AssemblerInstruction {
    pub fn to_bytes(&self, st: &SymbolTable) -> Vec<u8> {
        let mut res = vec![];
        match self.opcode {
            Some(Token::Op { code }) => res.push(code as u8),
            _ => {
                println!("Non-opcode found in opcode field: {:?}", self.opcode);
                std::process::exit(1)
            }
        };
        for op in vec![&self.operand1, &self.operand2, &self.operand3] {
            match op {
                Some(op) => AssemblerInstruction::extract_operand(op, &mut res, st),
                None => {}
            }
        }
        while res.len() < 4 {
            res.push(0); // padding
        }
        return res;
    }

    fn extract_operand(t: &Token, res: &mut Vec<u8>, st: &SymbolTable) {
        match t {
            Token::Reg { reg } => res.push(*reg),
            Token::IntegerOperand { i } => {
                let v = *i as u16;
                let byte1 = v;
                let byte2 = v >> 8;
                res.push(byte2 as u8);
                res.push(byte1 as u8);
            }
            Token::LabelUsage { name } => match st.symbol_value(name) {
                Some(value) => {
                    let byte1 = value;
                    let byte2 = value >> 8;
                    res.push(byte2 as u8);
                    res.push(byte1 as u8);
                }
                None => panic!("No value for symbol:{}", name),
            },
            _ => {
                println!("Non-operand found in operand field {:?}", t);
                std::process::exit(1)
            }
        }
    }

    pub fn label_name(&self) -> Option<String> {
        match &self.label {
            Some(Token::LabelDeclaration { name }) => Some(name.clone()),
            _ => None,
        }
    }

    pub fn directive_name(&self) -> Option<String> {
        match &self.directive {
            Some(Token::Directive { name }) => Some(name.clone()),
            _ => None,
        }
    }
}

pub fn instruction_zero(input: &str) -> IResult<&str, AssemblerInstruction> {
    let input = input.trim();
    let (input, (o, _)) = tuple((opcode, multispace0))(input)?;
    Ok((
        input,
        AssemblerInstruction {
            opcode: Some(o),
            operand1: None,
            operand2: None,
            operand3: None,
            label: None,
            directive: None,
        },
    ))
}
pub fn instruction_one(input: &str) -> IResult<&str, AssemblerInstruction> {
    let input = input.trim();
    let (input, (o, _, operand, _)) = tuple((
        opcode,
        multispace0,
        alt((register, integer_operand)), /* TODO improve this */
        multispace0,
    ))(input)?;
    Ok((
        input,
        AssemblerInstruction {
            opcode: Some(o),
            operand1: Some(operand),
            operand2: None,
            operand3: None,
            label: None,
            directive: None,
        },
    ))
}
pub fn instruction_two(input: &str) -> IResult<&str, AssemblerInstruction> {
    let input = input.trim();
    let (input, (o, _, r, _, i, _)) = tuple((
        opcode,
        multispace0,
        register,
        multispace0,
        integer_operand,
        multispace0,
    ))(input)?;
    Ok((
        input,
        AssemblerInstruction {
            opcode: Some(o),
            operand1: Some(r),
            operand2: Some(i),
            operand3: None,
            label: None,
            directive: None,
        },
    ))
}
pub fn instruction_three(input: &str) -> IResult<&str, AssemblerInstruction> {
    let input = input.trim();
    let (input, (o, _, r, _, i, _, i2, _)) = tuple((
        opcode,
        multispace0,
        register,
        multispace0,
        integer_operand,
        multispace0,
        integer_operand,
        multispace0,
    ))(input)?;
    Ok((
        input,
        AssemblerInstruction {
            opcode: Some(o),
            operand1: Some(r),
            operand2: Some(i),
            operand3: Some(i2),
            label: None,
            directive: None,
        },
    ))
}
pub fn instruction(input: &str) -> IResult<&str, AssemblerInstruction> {
    alt((
        directive,
        instruction_all,
        instruction_three,
        instruction_two,
        instruction_one,
        instruction_zero,
    ))(input)
}
pub fn instruction_all(input: &str) -> IResult<&str, AssemblerInstruction> {
    let (input, (_, label, _, opcode, _, operand1, _, operand2, _, operand3, _)) = tuple((
        multispace0,
        opt(label_declaration),
        space0,
        opcode,
        space0,
        opt(operand),
        space0,
        opt(operand),
        space0,
        opt(operand),
        space0,
    ))(input)?;

    let label = if let Some(Token::LabelDeclaration { name: label }) = label {
        Some(Token::LabelDeclaration {
            name: label.to_string(),
        })
    } else {
        None
    };
    Ok((
        input,
        AssemblerInstruction {
            opcode: Some(opcode),
            label,
            directive: None,
            operand1,
            operand2,
            operand3,
        },
    ))
}

mod tests {
    use super::*;
    use crate::instruction::Opcode;

    #[test]
    fn test_parse_instruction_two() {
        assert_eq!(
            instruction_two("load  $0     #100 \n"),
            Ok((
                "",
                AssemblerInstruction {
                    opcode: Some(Token::Op { code: Opcode::LOAD }),
                    operand1: Some(Token::Reg { reg: 0 }),
                    operand2: Some(Token::IntegerOperand { i: 100 }),
                    operand3: None,
                    label: None,
                    directive: None,
                }
            ))
        )
    }
}
