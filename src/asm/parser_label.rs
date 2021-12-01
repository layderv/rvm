use nom::{
    branch::alt, bytes::complete::tag, character::complete::alpha1, character::complete::space0,
    sequence::terminated, sequence::tuple, IResult,
};

use crate::asm::parser_instruction::*;
use crate::asm::parser_operand::operand;
use crate::asm::Token;

pub fn label_declaration(input: &str) -> IResult<&str, Token> {
    let (input, (_, name, _, _tag, _)) = tuple((space0, alpha1, space0, tag(":"), space0))(input)?;
    Ok((
        input,
        Token::LabelDeclaration {
            name: name.to_string(),
        },
    ))
}

pub fn label_usage(input: &str) -> IResult<&str, Token> {
    let (input, (_, _tag, _, name, _)) = tuple((space0, tag("@"), space0, alpha1, space0))(input)?;
    Ok((
        input,
        Token::LabelUsage {
            name: name.to_string(),
        },
    ))
}

mod tests {
    use super::*;
    #[test]
    fn test_parse_label_declaration() {
        assert_eq!(
            label_declaration(" test: "),
            Ok((
                "",
                Token::LabelDeclaration {
                    name: "test".to_string()
                }
            ))
        );
        assert_eq!(label_declaration(" test_ ").is_ok(), false);
    }
    #[test]
    fn test_parse_label_usage() {
        assert_eq!(
            label_usage(" @test "),
            Ok((
                "",
                Token::LabelUsage {
                    name: "test".to_string()
                }
            ))
        );
    }
}
