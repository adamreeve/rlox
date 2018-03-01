use num_traits::FromPrimitive;

#[derive(Debug,Copy,Clone,Primitive)]
pub enum OpCode {
    Return = 0
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
    pub code: Vec<u8>
}

impl Chunk {
    pub fn new() -> Chunk {
        Chunk {
            code: Vec::new()
        }
    }

    pub fn write_chunk(&mut self, byte: u8) {
        self.code.push(byte);
    }
}
