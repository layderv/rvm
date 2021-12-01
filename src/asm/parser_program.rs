use crate::asm::parser_instruction::*;
use nom::{multi::many1, IResult};

#[derive(Debug, PartialEq)]
pub struct Program {
    instructions: Vec<AssemblerInstruction>,
}

impl Program {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut prog = vec![];
        for i in &self.instructions {
            prog.append(&mut i.to_bytes())
        }
        return prog;
    }
}

pub fn program(input: &str) -> IResult<&str, Program> {
    let (input, is) = many1(instruction)(input)?;
    Ok((input, Program { instructions: is }))
}

mod tests {
    use super::*;
    use crate::asm::parser_op::*;
    use crate::asm::parser_operand::integer_operand;
    use crate::asm::parser_reg::register;
    use crate::asm::Token;
    use crate::instruction::Opcode;
    #[test]
    fn test_parse_program() {
        assert_eq!(
            program("load $0 #0\n"),
            Ok((
                "",
                Program {
                    instructions: vec![AssemblerInstruction {
                        opcode: Some(Token::Op { code: Opcode::LOAD }),
                        operand1: Some(Token::Reg { reg: 0 }),
                        operand2: Some(Token::IntegerOperand { i: 0 }),
                        operand3: None,
                        directive: None,
                        label: None,
                    }]
                }
            ))
        );
        assert_eq!(
            program("load $0 #0\nload $1 #100"),
            Ok((
                "",
                Program {
                    instructions: vec![
                        AssemblerInstruction {
                            opcode: Some(Token::Op { code: Opcode::LOAD }),
                            operand1: Some(Token::Reg { reg: 0 }),
                            operand2: Some(Token::IntegerOperand { i: 0 }),
                            operand3: None,
                            directive: None,
                            label: None,
                        },
                        AssemblerInstruction {
                            opcode: Some(Token::Op { code: Opcode::LOAD }),
                            operand1: Some(Token::Reg { reg: 1 }),
                            operand2: Some(Token::IntegerOperand { i: 100 }),
                            operand3: None,
                            directive: None,
                            label: None,
                        }
                    ]
                }
            ))
        );
    }
    #[test]
    fn test_program_to_bytes() {
        let prog = program("load $2 #100\n").unwrap().1;
        assert_eq!(prog.to_bytes().len(), 4);
        assert_eq!(prog.to_bytes()[0], Opcode::LOAD as u8);
        assert_eq!(prog.to_bytes()[1], 02u8);
        assert_eq!(prog.to_bytes()[2], 0);
        assert_eq!(prog.to_bytes()[3], 100u8);
    }
}
