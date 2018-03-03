use num_traits::FromPrimitive;
use byteorder::{LittleEndian, WriteBytesExt};

use ::value::Value;

#[derive(Debug,Copy,Clone,Primitive)]
pub enum OpCode {
    Return = 0,
    Constant = 1,
    ConstantLong = 2,
}

impl OpCode {
    pub fn as_byte(self) -> u8 {
        self as u8
    }

    pub fn from_byte(byte: u8) -> Option<OpCode> {
        OpCode::from_u8(byte)
    }
}

pub struct Chunk {
    pub code: Vec<u8>,
    pub constants: Vec<Value>,
    pub lines: Vec<usize>,
}

impl Chunk {
    pub fn new() -> Chunk {
        Chunk {
            code: Vec::new(),
            constants: Vec::new(),
            lines: Vec::new(),
        }
    }

    pub fn write_chunk(&mut self, byte: u8, line: usize) {
        self.code.push(byte);
        self.lines.push(line);
    }

    pub fn write_constant(&mut self, value: Value, line: usize) {
        // if we have over 256 constants, need to start
        // saving using load long instructions
        self.constants.push(value);
        let constant_index = self.constants.len() - 1;
        if constant_index > 255  {
            self.write_chunk(OpCode::ConstantLong.as_byte(), line);
            let constant_index = (self.constants.len() - 1) as u32;
            self.code.write_u32::<LittleEndian>(constant_index).unwrap();
            for _ in 0..4 {
                self.lines.push(line);
            }
        }
        else
        {
            self.write_chunk(OpCode::Constant.as_byte(), line);
            self.write_chunk(constant_index as u8, line);
        }
    }
}
