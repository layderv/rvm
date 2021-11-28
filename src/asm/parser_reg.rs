use nom::{
    bytes::complete::tag, character::complete::digit1, character::complete::multispace0,
    sequence::terminated, IResult,
};

use crate::asm::Token;

/* recognize exactly: $n where n is i32 with 0+ spaces around */
pub fn register(input: &str) -> IResult<&str, Token> {
    let (input, _eaten) = tag("$")(input)?;
    let (input, r) = terminated(digit1, multispace0)(input)?;
    Ok((
        input,
        Token::Reg {
            reg: r.parse::<u8>().unwrap(),
        },
    ))
}

mod tests {
    use super::*;
    #[test]
    fn test_parse_reg() {
        assert_eq!(register("$0").is_ok(), true);
        assert_eq!(register("$1").is_ok(), true);
        assert_eq!(register("0").is_ok(), false);
        assert_eq!(register("$a").is_ok(), false);
        assert_eq!(register("$0").unwrap(), ("", Token::Reg { reg: 0 }));
        assert_eq!(register("$0 ").unwrap(), ("", Token::Reg { reg: 0 }));
        assert_eq!(register("$0 a").unwrap(), ("a", Token::Reg { reg: 0 }));
    }
}
