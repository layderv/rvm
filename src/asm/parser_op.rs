use crate::asm::Token;
use crate::instruction::Opcode;
use nom::{bytes::complete::tag, IResult};

fn opcode_load(input: &str) -> IResult<&str, Token> {
    let input = input.trim();
    let (input, _) = tag("load")(input)?;
    Ok((input, Token::Op { code: Opcode::LOAD }))
}

mod tests {
    use super::*;
    #[test]
    fn test_opcode_load() {
        let r = opcode_load("load");
        assert_eq!(r.is_ok(), true);
        let (r, token) = r.unwrap();
        assert_eq!(token, Token::Op { code: Opcode::LOAD });
        assert_eq!(r, "");
        assert_eq!(opcode_load("invalid").is_ok(), false);
    }
}
