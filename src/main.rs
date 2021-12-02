#[macro_use]
pub mod asm;
pub mod instruction;
pub mod repl;
pub mod vm;

extern crate clap;
extern crate nom;

use clap::{load_yaml, App, Arg, SubCommand};

fn main() {
    let mut repl = repl::REPL::new();
    let yaml = load_yaml!("cli.yaml");
    let matches = App::from_yaml(yaml).get_matches();
    let target = matches.value_of("INPUT_FILE");
    match target {
        Some(filename) => match std::fs::read(filename) {
            Ok(bytes) => {
                repl.run(Some(bytes));
            }
            _ => {
                println!("Can't read file: {}", filename);
                std::process::exit(1);
            }
        },
        None => repl.run(None),
    }
}
