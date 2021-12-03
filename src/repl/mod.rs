use crate::asm::parser_program::*;
use crate::asm::Assembler;
use crate::asm::PIE_HEADER_LENGTH;
use crate::asm::PIE_HEADER_PREFIX;
use crate::vm;
use std;
use std::io;
use std::io::Write;
use std::num::ParseIntError;

pub struct REPL {
    cmd: Vec<String>,
    vm: vm::VM,
    asm: Assembler,
}

impl REPL {
    pub fn new() -> Self {
        REPL {
            vm: vm::VM::new(),
            cmd: vec![],
            asm: Assembler::new(),
        }
    }

    pub fn run(&mut self, prog_bytes: Option<Vec<u8>>) {
        println!("REPL version 0.1");
        if let Some(bytes) = prog_bytes {
            if !self.verify_header(&bytes) {
                println!("Wrong file or missing magic bytes");
            } else {
                let bytes = &bytes[PIE_HEADER_LENGTH..];
                match self.asm.assemble(std::str::from_utf8(&bytes).unwrap()) {
                    Ok(mut prog) => {
                        self.vm.program.append(&mut prog);
                        self.vm.pc = PIE_HEADER_LENGTH as usize;
                        self.vm.ro_data = self.asm.ro.clone();
                        println!("Parsed.");
                    }
                    Err(e) => {
                        println!("Cannot parse file, {:#?}", e);
                    }
                }
            }
        }
        loop {
            let mut input = String::new();
            let stdin = io::stdin();
            print!("> ");
            io::stdout().flush().expect("flush");
            stdin.read_line(&mut input).expect("cannot read line");
            let input = input.trim();
            let buf: Vec<&str> = input.split(" ").collect();
            let cmd = buf.first();
            if let None = cmd {
                println!("No command");
                continue;
            }
            let (cmd, args) = buf.split_at(1);
            match cmd.join("").as_str() {
                ".help" => {
                    println!("Commands:");
                    for cmd in vec![
                        ".help",
                        ".quit",
                        ".registers",
                        ".history",
                        ".program",
                        ".instruct BYTES",
                        ".run",
                        ".step",
                        ".clear_program",
                        ".ro_data",
                        ".load_file FILE",
                    ] {
                        println!("\t{}", cmd);
                    }
                }
                ".quit" => std::process::exit(0),
                ".history" => {
                    for (i, cmd) in self.cmd.iter().enumerate() {
                        println!("{}\t{}", i + 1, cmd)
                    }
                }
                ".program" => {
                    println!("Loaded program:");
                    for i in &self.vm.program {
                        println!("{:x}", i);
                    }
                }
                ".registers" => {
                    println!("Registers:");
                    for (i, reg) in self.vm.regs.iter().enumerate() {
                        print!("reg{:02}: {}\t", i, reg);
                        if i > 0 && (i % 4) == 3 {
                            println!()
                        }
                    }
                    println!();
                    println!(
                        "remainder:{}\nflag:{}",
                        self.vm.remainder, self.vm.bool_flag
                    );
                    println!("pc:{}", self.vm.pc);
                }
                ".instruct" => match self.parse_hex(&args.join(" ")) {
                    Ok(mut bytes) => self.vm.program.append(&mut bytes),
                    Err(e) => println!("Unable to parse hex, {:?}", e),
                },
                ".step" => {
                    self.vm.step();
                    ()
                }
                ".run" => {
                    self.vm.run();
                    ()
                }
                ".ro_data" => println!("Read-Only data: {:?}", self.vm.ro_data),
                ".clear_program" => self.vm.program.clear(),
                ".load_file" => {
                    if args.len() == 0 {
                        println!("No filename specified");
                        continue;
                    }
                    println!("Loading {}", args[0]);
                    match std::fs::read(args[0]) {
                        Ok(data) => {
                            if !self.verify_header(&data) {
                                println!("Wrong file or missing magic bytes");
                                continue;
                            }
                            let data = &data[PIE_HEADER_LENGTH..];
                            match self.asm.assemble(std::str::from_utf8(&data).unwrap()) {
                                Ok(mut prog) => {
                                    self.vm.program.append(&mut prog);
                                    println!("Parsed.");
                                }
                                Err(e) => {
                                    println!("Cannot parse file, {:#?}", e);
                                }
                            }
                        }
                        Err(e) => {
                            println!("Error reading the file: {}", e);
                        }
                    }
                }
                _ => {
                    println!("Invalid input <{}>. Try the .help command", input)
                }
            }
            self.cmd.push(input.to_string());
        }
    }

    fn verify_header(&self, bytes: &Vec<u8>) -> bool {
        bytes.len() > PIE_HEADER_PREFIX.len()
            && bytes[0..PIE_HEADER_PREFIX.len()] == PIE_HEADER_PREFIX
    }

    fn parse_hex(&mut self, i: &str) -> Result<Vec<u8>, ParseIntError> {
        let split = i.split(" ").collect::<Vec<&str>>();
        let mut res: Vec<u8> = vec![];
        for s in split {
            match u8::from_str_radix(&s, 0x10) {
                Ok(byte) => res.push(byte),
                Err(e) => return Err(e),
            }
        }
        Ok(res)
    }
}
