use crate::instruction::Opcode;
pub mod parser_directive;
pub mod parser_instruction;
pub mod parser_label;
pub mod parser_op;
pub mod parser_operand;
pub mod parser_program;
pub mod parser_reg;

use crate::asm::parser_program::{program, Program};

use self::parser_instruction::AssemblerInstruction;

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
    String { name: String },
}

#[derive(Debug)]
pub struct Assembler {
    pub phase: AssemblerPhase,
    pub symbols: SymbolTable,
    pub ro: Vec<u8>,       // read-only data section for constants
    pub bytecode: Vec<u8>, // compiled bytecode

    sections: Vec<AssemblerSection>,
    current_section: Option<AssemblerSection>,
    current_instruction: usize,
    errors: Vec<AssemblerError>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum AssemblerSection {
    Data,
    Code,
}

#[derive(Debug, Clone)]
pub enum AssemblerError {
    ParseError(String),
    NoSegmentFor(usize, String),     // where, what
    SymbolRedeclared(usize, String), // where, what
    UnknownDirective(usize, String), // where, what
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
            ro: vec![],
            bytecode: vec![],
            sections: vec![],
            current_section: None,
            current_instruction: 0,
            errors: vec![],
        }
    }

    pub fn assemble(&mut self, raw: &str) -> Result<Vec<u8>, Vec<AssemblerError>> {
        match program(raw) {
            Ok((_rest, prog)) => {
                let mut program = self.write_pie_header();
                self.process_first_phase(&prog);
                if self.errors.is_empty() && self.sections.contains(&AssemblerSection::Code) {
                    let mut body = self.process_second_phase(&prog);
                    program.append(&mut body);
                    Ok(program)
                } else {
                    Err(self.errors.clone())
                }
            }
            Err(e) => {
                println!("Error assembling: {}", e);
                Err(vec![AssemblerError::ParseError(e.to_string())])
            }
        }
    }

    fn process_first_phase(&mut self, p: &Program) {
        for (idx, i) in p.instructions.iter().enumerate() {
            if let Some(label) = i.label_name() {
                match self.current_section {
                    None => self
                        .errors
                        .push(AssemblerError::NoSegmentFor(idx * 4, label)),
                    _ => {
                        // label: .directive operands
                        if self.symbols.has_symbol(&label) {
                            self.errors
                                .push(AssemblerError::SymbolRedeclared(idx, label))
                        } else {
                            self.symbols.add_symbol(Symbol::new(
                                label,
                                SymbolType::Label,
                                idx as u32 * 4,
                            ))
                        }
                    }
                }
            }
            match i.directive_name() {
                Some(directive) if i.operand1.is_some() && i.label_name().is_some() => {
                    match directive.as_str() {
                        "asciiz" => self.do_asciiz(&i),
                        _ => self
                            .errors
                            .push(AssemblerError::UnknownDirective(idx * 4, directive)),
                    }
                }
                Some(directive) if i.operand1.is_none() => match directive.as_str() {
                    "code" => {
                        self.sections.push(AssemblerSection::Code);
                        self.current_section = Some(AssemblerSection::Code)
                    }
                    "data" => {
                        self.sections.push(AssemblerSection::Data);
                        self.current_section = Some(AssemblerSection::Data)
                    }
                    _ => self
                        .errors
                        .push(AssemblerError::UnknownDirective(idx * 4, directive)),
                },
                _ => {}
            }
        }
        self.phase = AssemblerPhase::Second;
    }

    fn process_second_phase(&mut self, p: &Program) -> Vec<u8> {
        let mut prog = vec![];
        for i in &p.instructions {
            if let Some(Token::Op { code }) = i.opcode {
                let mut bytes = i.to_bytes(&self.symbols);
                prog.append(&mut bytes);
            }
        }
        prog
    }

    fn do_asciiz(&mut self, i: &AssemblerInstruction) {
        // checked by the caller: this is in a label and operand1.is_some()
        if self.phase != AssemblerPhase::First {
            return;
        }
        if let Some(Token::String { name }) = &i.operand1 {
            self.symbols.set_symbol_offset(&name, self.ro.len() as u32);
            name.as_bytes().iter().for_each(|b| self.ro.push(*b));
            self.ro.push(0);
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

    pub fn has_symbol(&self, name: &String) -> bool {
        self.symbols.iter().any(|el| el.name == *name)
    }

    /// returns the offset of the symbol in input
    pub fn symbol_value(&self, s: &str) -> Option<u32> {
        for sym in &self.symbols {
            if sym.name == s {
                return Some(sym.offset);
            }
        }
        None
    }

    pub fn set_symbol_offset(&mut self, s: &String, offset: u32) {
        for sym in &mut self.symbols {
            if sym.name == *s {
                sym.offset = offset;
            }
        }
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

    #[test]
    fn test_ro_data() {
        let mut asm = Assembler::new();
        let prog = asm
            .assemble(
                ".data
            str: .asciiz 'Test String'
            .code
            hlt
            ",
            )
            .unwrap();
        assert_eq!(asm.symbols.has_symbol(&String::from("str")), true);
        assert_eq!(asm.symbols.symbol_value("str").unwrap(), 4);
        assert_eq!(asm.ro, "Test String\0".as_bytes());
    }
}
