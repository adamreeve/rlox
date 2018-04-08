use std::io::{Cursor, Read};
use ::chunk::Chunk;
use ::instructions;
use ::instructions::Instruction;
use ::instructions::OpCode;
use ::value::Value;

pub struct VirtualMachine<'a> {
    chunk: &'a Chunk,
    cursor: Cursor<&'a Vec<u8>>,
}

pub enum InterpretResult {
    Ok,
    CompileError,
    RuntimeError
}

impl<'a> VirtualMachine<'a> {
    pub fn new(chunk: &Chunk) -> VirtualMachine {
        VirtualMachine {
            chunk,
            cursor: Cursor::new(&chunk.code),
        }
    }

    pub fn interpret(&mut self) -> InterpretResult {
        loop {
            #[cfg(feature="debug-trace-execution")]
            {
                ::debug::disassemble_instruction(&self.chunk, self.cursor.position() as usize);
            }
            let instruction = self.read_byte();
            match instruction {
                Some(OpCode::Return) => {
                    return InterpretResult::Ok
                },
                Some(OpCode::Constant) => {
                    let value = self.read_constant();
                    println!("{}", value);
                },
                Some(OpCode::ConstantLong) => {
                    let value = self.read_constant_long();
                    println!("{}", value);
                },
                None => {
                    return InterpretResult::CompileError
                },
            }
        }
    }

    fn read_byte(&mut self) -> Option<OpCode> {
        let mut opcode_byte = [0u8];
        self.cursor.read(&mut opcode_byte).unwrap();
        OpCode::from_byte(opcode_byte[0])
    }

    fn read_constant(&mut self) -> Value {
        let instruction = instructions::ConstantInstruction::parse(&mut self.cursor);
        self.chunk.constants[instruction.constant_index as usize]
    }

    fn read_constant_long(&mut self) -> Value {
        let instruction = instructions::ConstantLongInstruction::parse(&mut self.cursor);
        self.chunk.constants[instruction.constant_index as usize]
    }
}
