use std::io::{Cursor, Read};
use ::chunk::Chunk;
use ::errors::{InterpretError, InterpretResult};
use ::instructions;
use ::instructions::InstructionRead;
use ::instructions::OpCode;
use ::value::Value;

pub struct VirtualMachine<'a> {
    chunk: &'a Chunk,
    cursor: Cursor<&'a Vec<u8>>,
    stack: Vec<Value>,
}

impl<'a> VirtualMachine<'a> {
    pub fn new(chunk: &Chunk) -> VirtualMachine {
        VirtualMachine {
            chunk,
            cursor: Cursor::new(&chunk.code),
            stack: Vec::with_capacity(256),
        }
    }

    pub fn interpret(&mut self) -> InterpretResult<()> {
        loop {
            #[cfg(feature="debug-trace-execution")]
            {
                println!("Stack: {:?}", &self.stack);
                ::debug::disassemble_instruction(&self.chunk, self.cursor.position() as usize);
                println!("");
            }
            let instruction = self.read_byte();
            match instruction {
                Some(OpCode::Add) => {
                    self.binary_op(|a, b| {a + b});
                },
                Some(OpCode::Constant) => {
                    let value = self.read_constant();
                    self.push(value);
                },
                Some(OpCode::ConstantLong) => {
                    let value = self.read_constant_long();
                    self.push(value);
                },
                Some(OpCode::Divide) => {
                    self.binary_op(|a, b| {a / b});
                },
                Some(OpCode::Multiply) => {
                    self.binary_op(|a, b| {a * b});
                },
                Some(OpCode::Negate) => {
                    let value = self.pop();
                    self.push(Value::new(-value.value()));
                },
                Some(OpCode::Return) => {
                    println!("{}", self.pop());
                    return Ok(());
                },
                Some(OpCode::Subtract) => {
                    self.binary_op(|a, b| {a - b});
                },
                None => {
                    let message = format!("Unrecognised op code: {:?}", instruction);
                    return Err(InterpretError::CompileError(message));
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

    fn binary_op<F>(&mut self, binary_fn: F)
        where F: Fn(f64, f64) -> f64
    {
        let b = self.pop();
        let a = self.pop();
        self.push(Value::new(binary_fn(a.value(), b.value())));
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
