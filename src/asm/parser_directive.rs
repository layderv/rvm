use nom::{
    branch::alt, bytes::complete::tag, character::complete::alpha1, character::complete::space0,
    sequence::terminated, sequence::tuple, IResult,
};

use crate::asm::parser_instruction::*;
use crate::asm::parser_operand::operand;
use crate::asm::Token;

pub fn directive_declaration(input: &str) -> IResult<&str, Token> {
    let (input, _) = tag(".")(input)?;
    let (input, dir) = terminated(alpha1, space0)(input)?;
    Ok((
        input,
        Token::Directive {
            name: dir.to_string(),
        },
    ))
}

pub fn directive_all(input: &str) -> IResult<&str, AssemblerInstruction> {
    let (input, (_, _, name, _, operand1, _, operand2, _, operand3, _)) = tuple((
        tag("."),
        space0,
        directive_declaration,
        space0,
        operand,
        space0,
        operand,
        space0,
        operand,
        space0,
    ))(input)?;
    Ok((
        input,
        AssemblerInstruction {
            opcode: None,
            directive: Some(name),
            label: None,
            operand1: Some(operand1),
            operand2: Some(operand2),
            operand3: Some(operand3),
        },
    ))
}

pub fn directive(input: &str) -> IResult<&str, AssemblerInstruction> {
    //alt((directive_all))(input)
    directive_all(input)
}
