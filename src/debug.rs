use super::chunk::Chunk;
use super::chunk::OpCode;

pub fn disassemble_chunk(chunk: &Chunk, name: &str) {
    println!("== {} ==", name);

    let mut offset = 0;
    while offset < chunk.code.len() {
        offset = disassemble_instruction(&chunk, offset);
    }
}

pub fn disassemble_instruction(chunk: &Chunk, offset: usize) -> usize {
    print!("{:04} ", offset);

    let line = chunk.lines[offset];
    let previous_line = if offset > 0 { chunk.lines[offset - 1] } else { 0 };
    if line == previous_line {
        print!("   | ");
    } else {
        print!("{:4} ", line);
    }

    let instruction = OpCode::from_byte(chunk.code[offset]);
    match instruction {
        Some(o @ OpCode::Return) => simple_instruction(o, offset),
        Some(o @ OpCode::Constant) => constant_instruction(o, chunk, offset),
        None => {
            println!("Unknown opcode: {}", chunk.code[offset]);
            offset + 1
        }
    }
}

fn simple_instruction(opcode: OpCode, offset: usize) -> usize {
    println!("OpCode::{:?}", opcode);
    offset + 1
}

fn constant_instruction(opcode: OpCode, chunk: &Chunk, offset: usize) -> usize {
    let constant = chunk.code[offset + 1];
    let value = chunk.constants[constant as usize];
    println!("OpCode::{:?} {:4} '{}'", opcode, constant, value);
    offset + 2
}
