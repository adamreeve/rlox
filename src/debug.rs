use std::io::{Cursor, Read, Seek, SeekFrom};

use ::chunk::Chunk;
use ::instructions::*;

pub fn disassemble_chunk(chunk: &Chunk, name: &str) {
    println!("== {} ==", name);

    let mut reader = Cursor::new(&chunk.code);
    let mut lines = chunk.lines.into_iter();
    let mut line_index = 0;
    let mut prev_line;
    let mut line = None;

    while reader.position() < (chunk.code.len() as u64) {
        prev_line = line;
        while line_index <= reader.position() {
            line = lines.next();
            line_index += 1;
        }
        disassemble_instruction_impl(chunk, &mut reader, prev_line, line.unwrap());
    }
}

pub fn disassemble_instruction(chunk: &Chunk, offset: usize) {
    let line = chunk.lines.into_iter().nth(offset).unwrap();
    let prev_line = if offset > 0 { chunk.lines.into_iter().nth(offset - 1) } else { None };

    let mut reader = Cursor::new(&chunk.code);
    reader.seek(SeekFrom::Start(offset as u64)).unwrap();

    disassemble_instruction_impl(chunk, &mut reader, prev_line, line);
}

fn disassemble_instruction_impl(chunk: &Chunk, reader: &mut Cursor<&Vec<u8>>, prev_line: Option<&usize>, line: &usize) {
    print!("{:04} ", reader.position());

    match prev_line {
        Some(prev_line) if prev_line == line => {
            print!("   | ");
        }
        _ => {
            print!("{:4} ", line);
        }
    };

    let mut opcode_byte = [0u8];
    reader.read(&mut opcode_byte).unwrap();
    let opcode = OpCode::from_byte(opcode_byte[0]);
    match opcode {
        Some(o @ OpCode::Add) => simple_instruction(o),
        Some(o @ OpCode::Constant) => constant_instruction(o, chunk, reader),
        Some(o @ OpCode::ConstantLong) => constant_long_instruction(o, chunk, reader),
        Some(o @ OpCode::True) => simple_instruction(o),
        Some(o @ OpCode::False) => simple_instruction(o),
        Some(o @ OpCode::Nil) => simple_instruction(o),
        Some(o @ OpCode::Divide) => simple_instruction(o),
        Some(o @ OpCode::Multiply) => simple_instruction(o),
        Some(o @ OpCode::Negate) => simple_instruction(o),
        Some(o @ OpCode::Return) => simple_instruction(o),
        Some(o @ OpCode::Subtract) => simple_instruction(o),
        Some(o @ OpCode::Not) => simple_instruction(o),
        Some(o @ OpCode::Equal) => simple_instruction(o),
        Some(o @ OpCode::Greater) => simple_instruction(o),
        Some(o @ OpCode::Less) => simple_instruction(o),
        None => {
            println!("Unknown opcode: {}", opcode_byte[0]);
        }
    }
}

fn simple_instruction(opcode: OpCode) {
    println!("OpCode::{:?}", opcode);
}

fn constant_instruction<R: Read>(opcode: OpCode, chunk: &Chunk, reader: &mut R) {
    let ConstantInstruction { constant_index } = ConstantInstruction::parse(reader);
    let value = chunk.constants[constant_index as usize];
    println!("OpCode::{:?} {:4} '{}'", opcode, constant_index, value);
}

fn constant_long_instruction<R: Read>(opcode: OpCode, chunk: &Chunk, reader: &mut R) {
    let ConstantLongInstruction { constant_index } = ConstantLongInstruction::parse(reader);
    let value = chunk.constants[constant_index as usize];
    println!("OpCode::{:?} {:4} '{}'", opcode, constant_index, value);
}
