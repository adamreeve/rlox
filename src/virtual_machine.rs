use std::io::{Cursor, Read};
use ::chunk::Chunk;
use ::errors::{InterpretError, InterpretResult};
use ::instructions;
use ::instructions::InstructionRead;
use ::instructions::OpCode;
use ::object;
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
                    self.binary_op(|a, b| {a + b}, Value::number)?;
                },
                Some(OpCode::Constant) => {
                    let value = self.read_constant();
                    self.push(value);
                },
                Some(OpCode::ConstantLong) => {
                    let value = self.read_constant_long();
                    self.push(value);
                },
                Some(OpCode::True) => {
                    self.push(Value::bool(true));
                },
                Some(OpCode::False) => {
                    self.push(Value::bool(false));
                },
                Some(OpCode::Nil) => {
                    self.push(Value::nil());
                },
                Some(OpCode::Divide) => {
                    self.binary_op(|a, b| {a / b}, Value::number)?;
                },
                Some(OpCode::Multiply) => {
                    self.binary_op(|a, b| {a * b}, Value::number)?;
                },
                Some(OpCode::Negate) => {
                    let value = self.pop();
                    match value {
                        Value::NumberValue(value) => {
                            self.push(Value::number(-value));
                        },
                        _ => {
                            return self.runtime_error("Operand must be a number");
                        }
                    }
                },
                Some(OpCode::Return) => {
                    return Ok(());
                },
                Some(OpCode::Subtract) => {
                    self.binary_op(|a, b| {a - b}, Value::number)?;
                },
                Some(OpCode::Not) => {
                    let value = is_falsey(self.pop());
                    self.push(Value::bool(value));
                },
                Some(OpCode::Equal) => {
                    let left = self.pop();
                    let right = self.pop();
                    self.push(Value::bool(values_equal(left, right)));
                },
                Some(OpCode::Greater) => {
                    self.binary_op(|a, b| a > b, Value::bool)?;
                },
                Some(OpCode::Less) => {
                    self.binary_op(|a, b| a < b, Value::bool)?;
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

    fn peek(&self, distance: usize) -> &Value {
        &self.stack[self.stack.len() - distance - 1]
    }

    fn binary_op<F, FC, T>(&mut self, binary_fn: F, value_creator: FC) -> InterpretResult<()>
        where F: Fn(f64, f64) -> T, FC: Fn(T) -> Value
    {
        if !(self.peek(0).is_number() && self.peek(1).is_number()) {
            return self.runtime_error("Operands must be numbers");
        }
        let b = self.pop();
        let a = self.pop();
        self.push(value_creator(binary_fn(a.as_number(), b.as_number())));
        Ok(())
    }

    fn read_byte(&mut self) -> Option<OpCode> {
        let mut opcode_byte = [0u8];
        self.cursor.read(&mut opcode_byte).unwrap();
        OpCode::from_byte(opcode_byte[0])
    }

    fn read_constant(&mut self) -> Value {
        let instruction = instructions::ConstantInstruction::parse(&mut self.cursor);
        self.chunk.constants[instruction.constant_index as usize].clone()
    }

    fn read_constant_long(&mut self) -> Value {
        let instruction = instructions::ConstantLongInstruction::parse(&mut self.cursor);
        self.chunk.constants[instruction.constant_index as usize].clone()
    }

    fn reset_stack(&mut self) {
        self.stack.clear();
    }

    fn runtime_error(&mut self, message: &str) -> InterpretResult<()> {
        let chunk_pos = self.cursor.position();
        let line_number = self.chunk.lines.nth(chunk_pos as usize);
        eprintln!("[line {}] {}", line_number, message);
        self.reset_stack();
        return Err(InterpretError::RuntimeError(message.to_string()));
    }
}

fn is_falsey(value: Value) -> bool {
    match value {
        Value::NilValue => true,
        Value::BoolValue(val) => !val,
        _ => false,
    }
}

fn values_equal(left: Value, right: Value) -> bool {
    match (left, right) {
        (Value::BoolValue(left), Value::BoolValue(right)) => left == right,
        (Value::NumberValue(left), Value::NumberValue(right)) => left == right,
        (Value::NilValue, Value::NilValue) => true,
        (Value::ObjValue(left), Value::ObjValue(right)) => object::objects_equal(&*left, &*right),
        (_, _) => false,
    }
}
