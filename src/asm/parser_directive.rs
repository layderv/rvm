use nom::combinator::opt;
use nom::{
    branch::alt, bytes::complete::tag, character::complete::alpha1, character::complete::space0,
    sequence::terminated, sequence::tuple, IResult,
};

use crate::asm::parser_instruction::*;
use crate::asm::parser_operand::operand;
use crate::asm::Token;

use super::parser_label::label_declaration;

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
    let input = input.trim();
    let (input, (_, label, _, name, _, operand1, _, operand2, _, operand3, _)) = tuple((
        space0,
        opt(label_declaration),
        space0,
        directive_declaration,
        space0,
        opt(operand),
        space0,
        opt(operand),
        space0,
        opt(operand),
        space0,
    ))(input)?;
    Ok((
        input,
        AssemblerInstruction {
            opcode: None,
            directive: Some(name),
            label: label,
            operand1: operand1,
            operand2: operand2,
            operand3: operand3,
        },
    ))
}

pub fn directive(input: &str) -> IResult<&str, AssemblerInstruction> {
    //alt((directive_all))(input)
    directive_all(input)
}

mod tests {
    use super::*;
    #[test]
    fn test_string_directive() {
        assert_eq!(
            directive_all("test: .asciiz 'Hi'"),
            Ok((
                "",
                AssemblerInstruction {
                    opcode: None,
                    label: Some(Token::LabelDeclaration {
                        name: "test".to_string()
                    }),
                    directive: Some(Token::Directive {
                        name: "asciiz".to_string()
                    }),
                    operand1: Some(Token::String {
                        name: "Hi".to_string()
                    }),
                    operand2: None,
                    operand3: None,
                }
            ))
        );
    }
}
