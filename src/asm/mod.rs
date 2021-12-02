use crate::instruction::Opcode;
pub mod parser_directive;
pub mod parser_instruction;
pub mod parser_label;
pub mod parser_op;
pub mod parser_operand;
pub mod parser_program;
pub mod parser_reg;

use crate::asm::parser_program::{program, Program};

pub const PIE_HEADER_PREFIX: [u8; 4] = [0x7e, 'P' as u8, 'I' as u8, 'E' as u8];
pub const PIE_HEADER_LENGTH: usize = 64;

#[derive(Debug, PartialEq)]
pub enum Token {
    Op { code: Opcode },
    Reg { reg: u8 },
    IntegerOperand { i: i32 },
    LabelDeclaration { name: String },
    LabelUsage { name: String },
    Directive { name: String },
}

#[derive(Debug)]
pub struct Assembler {
    pub phase: AssemblerPhase,
    pub symbols: SymbolTable,
}

#[derive(Debug, PartialEq, Clone)]
pub enum AssemblerPhase {
    First,
    Second,
}

#[derive(Debug)]
pub struct Symbol {
    name: String,
    offset: u32,
    type_: SymbolType,
}

#[derive(Debug)]
pub enum SymbolType {
    Label,
}

#[derive(Debug)]
pub struct SymbolTable {
    symbols: Vec<Symbol>,
}

impl Assembler {
    pub fn new() -> Assembler {
        Assembler {
            phase: AssemblerPhase::First,
            symbols: SymbolTable::new(),
        }
    }

    pub fn assemble(&mut self, raw: &str) -> Option<Vec<u8>> {
        match program(raw) {
            Ok((_rest, prog)) => {
                let mut program = self.write_pie_header();
                self.process_first_phase(&prog);
                let mut body = self.process_second_phase(&prog);
                program.append(&mut body);
                Some(program)
            }
            Err(e) => {
                println!("Error assembling: {}", e);
                None
            }
        }
    }

    fn process_first_phase(&mut self, p: &Program) {
        self.extract_labels(p);
        self.phase = AssemblerPhase::Second;
    }

    fn process_second_phase(&mut self, p: &Program) -> Vec<u8> {
        let mut prog = vec![];
        for i in &p.instructions {
            let mut bytes = i.to_bytes(&self.symbols);
            prog.append(&mut bytes);
        }
        prog
    }

    fn extract_labels(&mut self, p: &Program) {
        for (idx, i) in p.instructions.iter().enumerate() {
            match i.label_name() {
                Some(name) => {
                    let sym = Symbol::new(
                        name,
                        SymbolType::Label,
                        idx as u32 * 4, /* 4B per instr */
                    );
                    self.symbols.add_symbol(sym);
                }
                None => {}
            }
        }
    }

    fn write_pie_header(&self) -> Vec<u8> {
        let mut header: Vec<u8> = vec![];
        PIE_HEADER_PREFIX.iter().for_each(|b| header.push(*b));
        (header.len()..PIE_HEADER_LENGTH)
            .into_iter()
            .for_each(|_| header.push(0u8));
        header
    }
}

impl Symbol {
    pub fn new(name: String, type_: SymbolType, offset: u32) -> Symbol {
        Symbol {
            name,
            type_,
            offset,
        }
    }
}

impl SymbolTable {
    pub fn new() -> SymbolTable {
        SymbolTable { symbols: vec![] }
    }

    pub fn add_symbol(&mut self, s: Symbol) {
        self.symbols.push(s);
    }

    pub fn symbol_value(&self, s: &str) -> Option<u32> {
        for sym in &self.symbols {
            if sym.name == s {
                return Some(sym.offset);
            }
        }
        None
    }
}

mod tests {
    use super::*;
    #[test]
    fn test_symbol_table() {
        let mut st = SymbolTable::new();
        st.add_symbol(Symbol::new("test".to_string(), SymbolType::Label, 4 * 10));
        assert_eq!(st.symbols.len(), 1);
        assert_eq!(st.symbol_value("test").unwrap(), 4 * 10);
        assert_eq!(st.symbol_value("test2"), None);
    }
    #[test]
    fn test_assemble_program() {
        let mut asm = Assembler::new();
        let code = "load $0 #100\nload $1 #99\nlab:inc $0\njmp @test\nhlt";
        let program: Vec<u8> = asm.assemble(code).unwrap();
        assert_eq!(
            program[PIE_HEADER_LENGTH..], // TODO check header
            vec![
                Opcode::LOAD as u8,
                0,
                0,
                100,
                Opcode::LOAD as u8,
                1,
                0,
                99,
                Opcode::INC as u8,
                0,
                0,
                0,
                Opcode::JMP as u8,
                0,
                0,
                0,
            ]
        );
    }
}
