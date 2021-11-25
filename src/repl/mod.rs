use crate::vm;
use std;
use std::io;
use std::io::Write;

pub struct REPL {
    cmd: Vec<String>,
    vm: vm::VM,
}

impl REPL {
    pub fn new() -> Self {
        REPL {
            vm: vm::VM::new(),
            cmd: vec![],
        }
    }

    pub fn run(&mut self) {
        println!("REPL version 0.1");
        loop {
            let mut buf = String::new();
            let stdin = io::stdin();
            print!("> ");
            io::stdout().flush().expect("flush");
            stdin.read_line(&mut buf).expect("cannot read line");
            let buf = buf.trim();
            match buf {
                ".help" => {
                    println!("Commands:");
                    println!("\t.help");
                    println!("\t.quit");
                    println!("\t.history");
                }
                ".quit" => std::process::exit(0),
                ".history" => {
                    for (i, cmd) in self.cmd.iter().enumerate() {
                        println!("{}\t{}", i + 1, cmd)
                    }
                }
                _ => {
                    println!("invalid input")
                }
            }
            self.cmd.push(buf.to_string());
        }
    }
}
