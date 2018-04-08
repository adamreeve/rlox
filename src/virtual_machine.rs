use std::io::{Cursor, Read};
use ::chunk::Chunk;
use ::instructions;
use ::instructions::InstructionRead;
use ::instructions::OpCode;
use ::value::Value;

pub struct VirtualMachine<'a> {
    chunk: &'a Chunk,
    cursor: Cursor<&'a Vec<u8>>,
    stack: Vec<Value>,
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
            stack: Vec::with_capacity(256),
        }
    }

    pub fn interpret(&mut self) -> InterpretResult {
        loop {
            #[cfg(feature="debug-trace-execution")]
            {
                println!("Stack: {:?}", &self.stack);
                ::debug::disassemble_instruction(&self.chunk, self.cursor.position() as usize);
                println!("");
            }
            let instruction = self.read_byte();
            match instruction {
                Some(OpCode::Constant) => {
                    let value = self.read_constant();
                    self.push(value);
                },
                Some(OpCode::ConstantLong) => {
                    let value = self.read_constant_long();
                    self.push(value);
                },
                Some(OpCode::Negate) => {
                    let value = self.pop();
                    self.push(Value::new(-value.value()));
                },
                Some(OpCode::Return) => {
                    println!("{}", self.pop());
                    return InterpretResult::Ok
                },
                None => {
                    return InterpretResult::CompileError
                },
            }
        }
    }

    fn push(&mut self, value: Value) {
        self.stack.push(value);
    }

    fn pop(&mut self) -> Value {
        self.stack.pop().unwrap()
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
