use crate::asm::parser_program::*;
use crate::asm::Assembler;
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

    pub fn run(&mut self) {
        println!("REPL version 0.1");
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
                ".clear_program" => self.vm.program.clear(),
                ".load_file" => {
                    if args.len() == 0 {
                        println!("No filename specified");
                        continue;
                    }
                    println!("Loading {}", args[0]);
                    match std::fs::read(args[0]) {
                        Ok(data) => match program(std::str::from_utf8(&data).unwrap()) {
                            Ok((_rest, prog)) => {
                                self.vm
                                    .program
                                    .append(&mut prog.to_bytes(&self.asm.symbols));
                                println!("Parsed.");
                            }
                            Err(e) => {
                                println!("Cannot parse file, {}", e);
                            }
                        },
                        Err(e) => {
                            println!("Error reading the file: {}", e);
                        }
                    }
                }
                _ => {
                    println!("Invalid input. Try the .help command")
                }
            }
            self.cmd.push(input.to_string());
        }
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
