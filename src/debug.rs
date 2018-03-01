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

    let instruction = OpCode::from_byte(chunk.code[offset]);
    match instruction {
        Some(o @ OpCode::Return) => simple_instruction(o, offset),
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
