extern crate byteorder;
extern crate clap;
#[macro_use]
extern crate enum_primitive_derive;
extern crate num_traits;

mod chunk;
mod errors;
mod instructions;
mod debug;
mod util;
mod value;
mod virtual_machine;
mod compiler;
mod scanner;

use clap::{Arg, App};
use errors::InterpretResult;
use std::io::{self, BufRead, Read};
use std::fs::File;
use virtual_machine::VirtualMachine;

fn main() {
    let args = App::new("rlox")
        .about("Interpreter for the lox language")
        .arg(Arg::with_name("input")
             .help("Source file to run")
             .index(1))
        .get_matches();

    let result = match args.value_of("input") {
        Some(input_path) => run_file(input_path),
        _ => run_repl()
    };

    if let Err(err) = result {
        eprintln!("{}", err);
        std::process::exit(1);
    };
}

fn run_file(file_path: &str) -> InterpretResult<()> {
    let mut f = File::open(file_path)?;
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;
    Ok(())
}

fn run_repl() -> InterpretResult<()> {
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        interpret(&line?);
    }
    println!("");
    Ok(())
}

fn interpret(line: &str) {
}
