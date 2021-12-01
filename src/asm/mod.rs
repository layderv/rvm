use crate::instruction::Opcode;
pub mod parser_directive;
pub mod parser_instruction;
pub mod parser_label;
pub mod parser_op;
pub mod parser_operand;
pub mod parser_program;
pub mod parser_reg;

#[derive(Debug, PartialEq)]
pub enum Token {
    Op { code: Opcode },
    Reg { reg: u8 },
    IntegerOperand { i: i32 },
    LabelDeclaration { name: String },
    LabelUsage { name: String },
    Directive { name: String },
}
