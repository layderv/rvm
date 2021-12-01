use crate::asm::Token;
use crate::instruction::Opcode;
use nom::{
    character::complete::alpha1, character::complete::space0, sequence::terminated, IResult,
};

pub fn opcode(input: &str) -> IResult<&str, Token> {
    let (input, chars) = terminated(alpha1, space0)(input)?;
    Ok((
        input,
        Token::Op {
            code: Opcode::from(chars),
        },
    ))
}

mod tests {
    use super::*;
    #[test]
    fn test_opcode() {
        let r = opcode("load");
        assert_eq!(r.is_ok(), true);
        let (r, token) = r.unwrap();
        assert_eq!(token, Token::Op { code: Opcode::LOAD });
        assert_eq!(r, "");
        assert_eq!(
            opcode("invalid").unwrap().1,
            Token::Op { code: Opcode::IGL }
        );
    }
}
