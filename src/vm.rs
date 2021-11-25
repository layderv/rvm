use crate::instruction::Opcode;

pub struct VM {
    regs: [i32; 32],
    pc: usize,
    program: Vec<u8>,
}

impl VM {
    pub fn new() -> VM {
        VM {
            regs: [0; 32],
            pc: 0,
            program: vec![],
        }
    }

    pub fn run(&mut self) {
        loop {
            if self.pc >= self.program.len() {
                break;
            }
            match self.decode_opcode() {
                Opcode::HLT => {
                    println!("HLTing");
                    return;
                }
                _ => {
                    println!("Unrecognized opcode");
                    return;
                }
            }
        }
    }

    fn decode_opcode(&mut self) -> Opcode {
        let op = Opcode::from(self.program[self.pc]);
        self.pc += 1;
        return op;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_create_vm() {
        let vm = VM::new();
        for x in vm.regs {
            assert_eq!(x, 0)
        }
    }
    #[test]
    fn test_opcode_hlt() {
        let mut vm = VM::new();
        let b = vec![0, 0, 0, 0];
        vm.program = b;
        vm.run();
        assert_eq!(vm.pc, 1);
    }
    #[test]
    fn test_opcode_illegal() {
        let mut vm = VM::new();
        let b = vec![255, 0, 0];
        vm.program = b;
        vm.run();
        assert_eq!(vm.pc, 1);
    }
}
