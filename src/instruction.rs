use std::fmt;

#[derive(Copy, Clone, Debug, PartialEq)]
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

impl fmt::Display for Opcode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<&str> for Opcode {
    fn from(v: &str) -> Self {
        match v.to_lowercase().as_str() {
            "nop" => Opcode::NOP,
            "hlt" => Opcode::HLT,
            "load" => Opcode::LOAD,
            "add" => Opcode::ADD,
            "sub" => Opcode::SUB,
            "mul" => Opcode::MUL,
            "div" => Opcode::DIV,
            "jmp" => Opcode::JMP,
            "jmpb" => Opcode::JMPB,
            "jmpf" => Opcode::JMPF,
            "eq" => Opcode::EQ,
            "neq" => Opcode::NEQ,
            "gt" => Opcode::GT,
            "lt" => Opcode::LT,
            "geq" => Opcode::GEQ,
            "leq" => Opcode::LEQ,
            "jeq" => Opcode::JEQ,
            "jne" => Opcode::JNE,
            _ => Opcode::IGL,
        }
    }
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
