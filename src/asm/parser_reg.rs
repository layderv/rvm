use nom::{bytes::complete::tag, character::complete::u8, combinator::all_consuming, IResult};

use crate::asm::Token;

/* recognize exactly: $n where n is i32 */
fn register(input: &str) -> IResult<&str, Token> {
    let input = input.trim();
    let (input, _eaten) = tag("$")(input)?;
    let (input, r) = all_consuming(u8)(input)?;
    Ok((input, Token::Reg { reg: r }))
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
        assert_eq!(register(" $0   ").unwrap(), ("", Token::Reg { reg: 0 }));
    }
}
