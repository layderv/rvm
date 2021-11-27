use nom::{bytes::complete::tag, character::complete::i32, combinator::all_consuming, IResult};

use crate::asm::Token;

/* recognize: #n where n is i32 */
fn integer_operand(input: &str) -> IResult<&str, Token> {
    let input = input.trim();
    let (input, _) = tag("#")(input)?;
    let (input, i) = all_consuming(i32)(input)?;
    Ok((input, Token::IntegerOperand { i: i }))
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
            integer_operand("   #0 ").unwrap(),
            ("", Token::IntegerOperand { i: 0 })
        );
        assert_eq!(
            integer_operand("#10").unwrap(),
            ("", Token::IntegerOperand { i: 10 })
        );
        println!("{:?}", integer_operand("#1a"));
        assert_eq!(integer_operand("#1a").is_ok(), false);
        assert_eq!(integer_operand("1").is_ok(), false);
    }
}
