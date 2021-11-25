#[derive(Debug, PartialEq)]
pub enum Opcode {
    NOP = 0,
    HLT,
    LOAD,
    ADD,
    SUB,
    MUL,
    DIV,
    JMP,
    JMPB,
    JMPF,
    EQ,
    NEQ,
    GT,
    LT,
    GEQ,
    LEQ,
    JEQ,
    JNE,
    IGL,
}

#[derive(Debug, PartialEq)]
pub struct Instruction {
    opcode: Opcode,
}

impl Instruction {
    pub fn new(opcode: Opcode) -> Instruction {
        Instruction { opcode: opcode }
    }
}

impl From<u8> for Opcode {
    fn from(v: u8) -> Self {
        match v {
            x if x == Opcode::HLT as u8 => return Opcode::HLT,
            x if x == Opcode::LOAD as u8 => return Opcode::LOAD,
            x if x == Opcode::ADD as u8 => Opcode::ADD,
            x if x == Opcode::SUB as u8 => Opcode::SUB,
            x if x == Opcode::MUL as u8 => Opcode::MUL,
            x if x == Opcode::DIV as u8 => Opcode::DIV,
            x if x == Opcode::JMP as u8 => Opcode::JMP,
            x if x == Opcode::JMPB as u8 => Opcode::JMPB,
            x if x == Opcode::JMPF as u8 => Opcode::JMPF,
            x if x == Opcode::EQ as u8 => Opcode::EQ,
            x if x == Opcode::NEQ as u8 => Opcode::NEQ,
            x if x == Opcode::GT as u8 => Opcode::GT,
            x if x == Opcode::LT as u8 => Opcode::LT,
            x if x == Opcode::GEQ as u8 => Opcode::GEQ,
            x if x == Opcode::LEQ as u8 => Opcode::LEQ,
            x if x == Opcode::JEQ as u8 => Opcode::JEQ,
            x if x == Opcode::JNE as u8 => Opcode::JNE,
            _ => return Opcode::IGL,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_create_hlt() {
        let opcode = Opcode::HLT;
        assert_eq!(opcode, Opcode::HLT);
    }
    #[test]
    fn test_create_instruction() {
        let i = Instruction::new(Opcode::HLT);
        assert_eq!(i.opcode, Opcode::HLT);
    }
}
