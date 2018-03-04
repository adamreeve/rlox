use std::io::{Read,Write};
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use num_traits::FromPrimitive;

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

pub trait Instruction {
    const OP_CODE: OpCode;
    fn parse<R: Read>(reader: &mut R) -> Self;
    fn write<W: Write>(&self, writer: &mut W);
}

pub struct ReturnInstruction {
}

impl ReturnInstruction {
    pub fn new() -> ReturnInstruction {
        ReturnInstruction { }
    }
}

impl Instruction for ReturnInstruction {
    const OP_CODE: OpCode = OpCode::Return;

    fn parse<R: Read>(_: &mut R) -> ReturnInstruction {
        ReturnInstruction {}
    }

    fn write<W: Write>(&self, writer: &mut W) {
        writer.write(&[OpCode::Return.as_byte()]).unwrap();
    }
}

pub struct ConstantInstruction {
    pub constant_index: u8,
}

impl ConstantInstruction {
    pub fn new(constant_index: u8) -> ConstantInstruction {
        ConstantInstruction { constant_index }
    }
}

impl Instruction for ConstantInstruction {
    const OP_CODE: OpCode = OpCode::Constant;

    fn parse<R: Read>(reader: &mut R) -> ConstantInstruction {
        let mut index = [0u8];
        reader.read(&mut index).unwrap();
        ConstantInstruction {
            constant_index: index[0],
        }
    }

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

impl Instruction for ConstantLongInstruction {
    const OP_CODE: OpCode = OpCode::ConstantLong;

    fn parse<R: Read>(reader: &mut R) -> ConstantLongInstruction {
        let constant_index = reader.read_u32::<LittleEndian>().unwrap();
        ConstantLongInstruction {
            constant_index,
        }
    }

    fn write<W: Write>(&self, writer: &mut W) {
        writer.write(&[OpCode::ConstantLong.as_byte()]).unwrap();
        writer.write_u32::<LittleEndian>(self.constant_index).unwrap();
    }
}
