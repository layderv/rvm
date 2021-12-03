use nom::bytes::complete::take_until;
use nom::{
    branch::alt, bytes::complete::tag, character::complete::digit1,
    character::complete::multispace0, sequence::terminated, IResult,
};

use crate::asm::parser_reg::register;
use crate::asm::Token;

use super::parser_label::label_usage;

/* recognize: #n where n is i32 with 0+ spaces around */
pub fn integer_operand(input: &str) -> IResult<&str, Token> {
    let input = input.trim();
    let (input, _) = tag("#")(input)?;
    let (input, i) = terminated(digit1, multispace0)(input)?;
    Ok((
        input,
        Token::IntegerOperand {
            i: i.parse::<i32>().unwrap(),
        },
    ))
}

pub fn string_operand(input: &str) -> IResult<&str, Token> {
    let input = input.trim();
    let (input, _) = tag("'")(input)?;
    let (input, s) = take_until("'")(input)?;
    let (input, _) = tag("'")(input)?;
    Ok((
        input,
        Token::String {
            name: s.to_string(),
        },
    ))
}

pub fn operand(input: &str) -> IResult<&str, Token> {
    alt((integer_operand, label_usage, register, string_operand))(input)
}

mod tests {
    use super::*;
    #[test]
    fn test_parse_integer_operand() {
        assert_eq!(
            integer_operand("#0").unwrap(),
            ("", Token::IntegerOperand { i: 0 })
        );
        assert_eq!(
            integer_operand("#10").unwrap(),
            ("", Token::IntegerOperand { i: 10 })
        );
        //assert_eq!(integer_operand("#1a").is_ok(), false);
        assert_eq!(integer_operand("1").is_ok(), false);
    }
    #[test]
    fn test_parse_string_operand() {
        assert_eq!(
            string_operand("'hi'").unwrap(),
            (
                "",
                Token::String {
                    name: "hi".to_string()
                }
            )
        );
    }
}
