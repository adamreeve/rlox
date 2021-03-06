use std::io::{Read,Write};
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use num_traits::FromPrimitive;

#[derive(Debug,Copy,Clone,Primitive)]
pub enum OpCode {
    Return = 0,
    Constant = 1,
    ConstantLong = 2,
    Nil = 3,
    True = 4,
    False = 5,
    Negate = 6,
    Add = 7,
    Subtract = 8,
    Multiply = 9,
    Divide = 10,
    Not = 11,
    Equal = 12,
    Greater = 13,
    Less = 14,
}

impl OpCode {
    pub fn as_byte(self) -> u8 {
        self as u8
    }

    pub fn from_byte(byte: u8) -> Option<OpCode> {
        OpCode::from_u8(byte)
    }
}

pub trait InstructionRead {
    fn parse<R: Read>(reader: &mut R) -> Self;
}

pub trait InstructionWrite {
    fn write<W: Write>(&self, writer: &mut W);
}

pub struct ConstantInstruction {
    pub constant_index: u8,
}

impl ConstantInstruction {
    pub fn new(constant_index: u8) -> ConstantInstruction {
        ConstantInstruction { constant_index }
    }
}

impl InstructionRead for ConstantInstruction {
    fn parse<R: Read>(reader: &mut R) -> ConstantInstruction {
        let mut index = [0u8];
        reader.read(&mut index).unwrap();
        ConstantInstruction {
            constant_index: index[0],
        }
    }
}

impl InstructionWrite for ConstantInstruction {
    fn write<W: Write>(&self, writer: &mut W) {
        writer.write(&[OpCode::Constant.as_byte(), self.constant_index as u8]).unwrap();
    }
}

pub struct ConstantLongInstruction {
    pub constant_index: u32,
}

impl ConstantLongInstruction {
    pub fn new(constant_index: u32) -> ConstantLongInstruction {
        ConstantLongInstruction { constant_index }
    }
}

impl InstructionRead for ConstantLongInstruction {
    fn parse<R: Read>(reader: &mut R) -> ConstantLongInstruction {
        let constant_index = reader.read_u32::<LittleEndian>().unwrap();
        ConstantLongInstruction {
            constant_index,
        }
    }
}

impl InstructionWrite for ConstantLongInstruction {
    fn write<W: Write>(&self, writer: &mut W) {
        writer.write(&[OpCode::ConstantLong.as_byte()]).unwrap();
        writer.write_u32::<LittleEndian>(self.constant_index).unwrap();
    }
}

pub struct SimpleInstruction {
    op_code: OpCode
}

impl SimpleInstruction {
    pub fn new(op_code: OpCode) -> SimpleInstruction {
        SimpleInstruction { op_code }
    }
}

impl InstructionWrite for SimpleInstruction {
    fn write<W: Write>(&self, writer: &mut W) {
        writer.write(&[self.op_code.as_byte()]).unwrap();
    }
}
