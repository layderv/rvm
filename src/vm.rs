use crate::instruction::Opcode;

pub struct VM {
    pub regs: [i32; 32],
    pub pc: usize,
    pub program: Vec<u8>,
    pub remainder: u32,
    pub bool_flag: bool, // equality flag
}

impl VM {
    pub fn new() -> VM {
        VM {
            regs: [0; 32],
            pc: 0,
            program: vec![],
            remainder: 0,
            bool_flag: false,
        }
    }

    pub fn run(&mut self) {
        let mut done = false;
        while !done {
            done = self.step();
        }
    }

    pub fn step(&mut self) -> bool {
        if self.pc >= self.program.len() {
            return true;
        }
        match self.decode_opcode() {
            Opcode::NOP => {}
            Opcode::HLT => {
                println!("HLTing");
                return true;
            }
            Opcode::LOAD => {
                let reg = self.next_8b() as usize;
                let n = self.next_16b() as u16;
                self.regs[reg] = n as i32;
            }
            Opcode::ADD => {
                let p = self.regs[self.next_8b_reg() as usize];
                let q = self.regs[self.next_8b_reg() as usize];
                self.regs[self.next_8b_reg() as usize] = p.wrapping_add(q);
            }
            Opcode::SUB => {
                let p = self.regs[self.next_8b_reg() as usize];
                let q = self.regs[self.next_8b_reg() as usize];
                self.regs[self.next_8b_reg() as usize] = p.wrapping_sub(q);
            }
            Opcode::MUL => {
                let p = self.regs[self.next_8b_reg() as usize];
                let q = self.regs[self.next_8b_reg() as usize];
                self.regs[self.next_8b_reg() as usize] = p.wrapping_mul(q);
            }
            Opcode::DIV => {
                let p = self.regs[self.next_8b_reg() as usize];
                let q = self.regs[self.next_8b_reg() as usize];
                self.regs[self.next_8b_reg() as usize] = p / q;
                self.remainder = (p % q) as u32;
            }
            Opcode::JMP => {
                let t = self.regs[self.next_8b_reg() as usize];
                self.pc = t as usize;
            }
            Opcode::JMPB => {
                let t = self.regs[self.next_8b_reg() as usize];
                println!(
                    "t={:?} oldpc={:?} newpc={:?}",
                    t,
                    self.pc,
                    self.pc.wrapping_sub(t as usize)
                );
                self.pc = self.pc.wrapping_sub(t as usize);
            }
            Opcode::JMPF => {
                let t = self.regs[self.next_8b_reg() as usize];
                self.pc = self.pc.wrapping_add(t as usize);
            }
            Opcode::EQ => {
                let a = self.regs[self.next_8b_reg() as usize];
                let b = self.regs[self.next_8b_reg() as usize];
                self.bool_flag = a == b;
                self.discard_8b(); // discard 8b
            }
            Opcode::NEQ => {
                let a = self.regs[self.next_8b_reg() as usize];
                let b = self.regs[self.next_8b_reg() as usize];
                self.bool_flag = a != b;
                self.discard_8b(); // discard 8b
            }
            Opcode::GT => {
                let a = self.regs[self.next_8b_reg() as usize];
                let b = self.regs[self.next_8b_reg() as usize];
                self.bool_flag = a > b;
                self.discard_8b(); // discard 8b
            }
            Opcode::GEQ => {
                let a = self.regs[self.next_8b_reg() as usize];
                let b = self.regs[self.next_8b_reg() as usize];
                self.bool_flag = a >= b;
                self.discard_8b(); // discard 8b
            }
            Opcode::LT => {
                let a = self.regs[self.next_8b_reg() as usize];
                let b = self.regs[self.next_8b_reg() as usize];
                self.bool_flag = a < b;
                self.discard_8b(); // discard 8b
            }
            Opcode::LEQ => {
                let a = self.regs[self.next_8b_reg() as usize];
                let b = self.regs[self.next_8b_reg() as usize];
                self.bool_flag = a <= b;
                self.discard_8b(); // discard 8b
            }
            Opcode::JEQ => {
                let t = self.regs[self.next_8b_reg() as usize];
                if self.bool_flag {
                    self.pc = t as usize;
                }
            }
            Opcode::JNE => {
                let t = self.regs[self.next_8b_reg() as usize];
                if !self.bool_flag {
                    self.pc = t as usize;
                }
            }
            op => {
                println!("Unrecognized opcode: {:?}", op);
                return true;
            }
        }
        false
    }

    fn decode_opcode(&mut self) -> Opcode {
        let op = Opcode::from(self.program[self.pc]);
        println!("decoding byte: {:?}", self.program[self.pc]);
        self.pc += 1;
        return op;
    }

    fn discard_8b(&mut self) {
        self.pc += 1;
    }

    fn next_8b(&mut self) -> u8 {
        let r = self.program[self.pc];
        self.pc += 1;
        return r;
    }

    fn next_8b_reg(&mut self) -> u8 {
        let r = self.next_8b();
        if usize::from(r) >= self.regs.len() {
            panic!("reg index too high: {:?}", r);
        }
        return r;
    }

    fn next_16b(&mut self) -> u16 {
        let first = (self.program[self.pc] as u16) << 8;
        self.pc += 1;
        let second = self.program[self.pc] as u16;
        self.pc += 1;
        return first | second;
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
        let b = vec![Opcode::HLT as u8];
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
    #[test]
    fn test_opcode_load() {
        let mut vm = VM::new();
        /* 1: load, 0: target register, (1<<8)+244 == 500 */
        vm.program = vec![Opcode::LOAD as u8, 0, 1, 244, Opcode::HLT as u8];
        vm.run();
        assert_eq!(vm.regs[0], 500);
    }
    #[test]
    fn test_opcode_add() {
        let mut vm = VM::new();
        vm.program = vec![
            Opcode::LOAD as u8,
            0,
            0,
            1, // regs[0] = 1
            Opcode::LOAD as u8,
            1,
            1,
            1, // regs[1] = (1<<8)+1 = 257
            Opcode::ADD as u8,
            0,
            1,
            2, // regs[2] = regs[1]+regs[0]
            Opcode::HLT as u8,
        ]; // hlt
        vm.run();
        assert_eq!(vm.regs[2], 258);
    }
    #[test]
    fn test_opcode_sub() {
        let mut vm = VM::new();
        vm.program = vec![
            Opcode::LOAD as u8,
            1,
            0,
            1, // regs[1] = 1
            Opcode::LOAD as u8,
            0,
            1,
            1, // regs[0] = (1<<8)+1 = 257
            Opcode::SUB as u8,
            0,
            1,
            2, // regs[2] = regs[0]-regs[1]
            Opcode::HLT as u8,
        ]; // hlt
        vm.run();
        assert_eq!(vm.regs[2], 256);
    }
    #[test]
    fn test_opcode_mul() {
        let mut vm = VM::new();
        vm.program = vec![
            Opcode::LOAD as u8,
            0,
            0,
            2, // regs[0] = 2
            Opcode::LOAD as u8,
            1,
            1,
            1, // regs[1] = (1<<8)+1 = 257
            Opcode::MUL as u8,
            0,
            1,
            2, // regs[2] = regs[1]*regs[0]
            Opcode::HLT as u8,
        ]; // hlt
        vm.run();
        assert_eq!(vm.regs[2], 257 * 2);
    }
    #[test]
    fn test_opcode_div() {
        let mut vm = VM::new();
        vm.program = vec![
            Opcode::LOAD as u8,
            0,
            0,
            2, // regs[0] = 2
            Opcode::LOAD as u8,
            1,
            0,
            3, // regs[1] = 3
            Opcode::DIV as u8,
            1,
            0,
            2, // regs[2] = regs[1]/regs[0]
            Opcode::HLT as u8,
        ]; // hlt
        vm.run();
        assert_eq!(vm.regs[2], 1);
        assert_eq!(vm.remainder, 1);
    }
    #[test]
    fn test_opcode_jmp() {
        let mut vm = VM::new();
        vm.regs[1] = 5;
        vm.program = vec![Opcode::JMP as u8, 1, 255, 255, 255, Opcode::HLT as u8];
        vm.step();
        assert_eq!(vm.pc, 5);
    }
    #[test]
    fn test_opcode_jmpb() {
        let mut vm = VM::new();
        vm.regs[1] = 3;
        vm.pc = 1;
        vm.program = vec![Opcode::HLT as u8, Opcode::JMPB as u8, 1, 255, 255, 255];
        vm.run();
        assert_eq!(vm.pc, 1); // stop after executing hlt
    }
    #[test]
    fn test_opcode_jmpf() {
        let mut vm = VM::new();
        vm.regs[1] = 3;
        vm.program = vec![Opcode::JMPF as u8, 1, 255, 255, Opcode::HLT as u8];
        vm.step();
        assert_eq!(vm.pc, 5);
    }
    #[test]
    fn test_opcode_eq() {
        let mut vm = VM::new();
        vm.regs[0] = 1;
        vm.regs[1] = 1;
        vm.program = vec![Opcode::EQ as u8, 0, 1];
        vm.step();
        assert_eq!(vm.bool_flag, true);
        vm.pc = 0;
        vm.regs[0] = 0;
        vm.step();
        assert_eq!(vm.bool_flag, false);
    }
    #[test]
    fn test_opcode_neq() {
        let mut vm = VM::new();
        vm.regs[0] = 1;
        vm.regs[1] = 1;
        vm.program = vec![Opcode::NEQ as u8, 0, 1];
        vm.step();
        assert_eq!(vm.bool_flag, false);
        vm.pc = 0;
        vm.regs[0] = 0;
        vm.step();
        assert_eq!(vm.bool_flag, true);
    }
    #[test]
    fn test_opcode_gt() {
        let mut vm = VM::new();
        vm.regs[0] = 1;
        vm.regs[1] = 1;
        vm.program = vec![Opcode::GT as u8, 0, 1];
        vm.step();
        assert_eq!(vm.bool_flag, false);
        vm.pc = 0;
        vm.regs[0] = 5;
        vm.step();
        assert_eq!(vm.bool_flag, true);
    }
    #[test]
    fn test_opcode_lt() {
        let mut vm = VM::new();
        vm.regs[0] = 1;
        vm.regs[1] = 1;
        vm.program = vec![Opcode::LT as u8, 0, 1];
        vm.step();
        assert_eq!(vm.bool_flag, false);
        vm.pc = 0;
        vm.regs[1] = 2;
        vm.step();
        assert_eq!(vm.bool_flag, true);
    }
    #[test]
    fn test_opcode_geq() {
        let mut vm = VM::new();
        vm.regs[0] = 1;
        vm.regs[1] = 1;
        vm.program = vec![Opcode::GEQ as u8, 0, 1];
        vm.step();
        assert_eq!(vm.bool_flag, true);
        vm.pc = 0;
        vm.regs[0] = 0;
        vm.step();
        assert_eq!(vm.bool_flag, false);
    }
    #[test]
    fn test_opcode_leq() {
        let mut vm = VM::new();
        vm.regs[0] = 1;
        vm.regs[1] = 1;
        vm.program = vec![Opcode::LEQ as u8, 0, 1];
        vm.step();
        assert_eq!(vm.bool_flag, true);
        vm.pc = 0;
        vm.regs[0] = 0;
        vm.step();
        assert_eq!(vm.bool_flag, true);
    }
    #[test]
    fn test_opcode_jeq() {
        let mut vm = VM::new();
        vm.bool_flag = true;
        vm.regs[0] = 5;
        vm.program = vec![Opcode::JEQ as u8, 0, 255, 255, 255, Opcode::HLT as u8];
        vm.step();
        assert_eq!(vm.pc, 5);
        vm.pc = 0;
        vm.bool_flag = false;
        vm.regs[0] = 5;
        vm.program = vec![Opcode::JEQ as u8, 0, 255, 255, 255, Opcode::HLT as u8];
        vm.step();
        assert_eq!(vm.pc, 2);
    }
    #[test]
    fn test_opcode_jne() {
        let mut vm = VM::new();
        vm.bool_flag = true;
        vm.regs[0] = 5;
        vm.program = vec![Opcode::JNE as u8, 0, 255, 255, 255, Opcode::HLT as u8];
        vm.step();
        assert_eq!(vm.pc, 2);
        vm.pc = 0;
        vm.bool_flag = false;
        vm.regs[0] = 5;
        vm.program = vec![Opcode::JNE as u8, 0, 255, 255, 255, Opcode::HLT as u8];
        vm.step();
        assert_eq!(vm.pc, 5);
    }
}
