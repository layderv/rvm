#[derive(Debug, PartialEq)]
pub enum Opcode {
    HLT = 0,
    LOAD,
    ADD,
    SUB,
    MUL,
    DIV,
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
