use std;
use ::errors::{InterpretResult, InterpretError};
use ::instructions::*;
use ::run_length_encoding::RunLengthEncoded;
use ::value::Value;

pub struct Chunk {
    pub code: Vec<u8>,
    pub constants: Vec<Value>,
    pub lines: RunLengthEncoded<usize>,
}

impl Chunk {
    pub fn new() -> Chunk {
        Chunk {
            code: Vec::new(),
            constants: Vec::new(),
            lines: RunLengthEncoded::new(),
        }
    }

    pub fn write_instruction<I: InstructionWrite>(&mut self, instruction: I, line: usize) {
        let initial_code_len = self.code.len();
        instruction.write(&mut self.code);
        let new_code_len = self.code.len();
        self.lines.push_run(line, new_code_len - initial_code_len);
    }

    pub fn write_constant(&mut self, value: Value, line: usize) -> InterpretResult<()> {
        self.constants.push(value);
        let constant_index = self.constants.len() - 1;
        // Once we have over 256 constants, we need to start
        // saving constants using a constant long instruction:
        if constant_index <= std::u8::MAX as usize
        {
            self.write_instruction(ConstantInstruction::new(constant_index as u8), line);
            Ok(())
        }
        else if constant_index <= std::u32::MAX as usize  {
            self.write_instruction(ConstantLongInstruction::new(constant_index as u32), line);
            Ok(())
        }
        else {
            Err(InterpretError::CompileError("Too many constants to store".to_string()))
        }
    }
}

#[cfg(test)]
mod test {
    use std::io::{Cursor, Read, Write};
    use byteorder::{ReadBytesExt, LittleEndian};
    use ::chunk::Chunk;
    use ::value::Value;
    use ::instructions::InstructionWrite;
    use ::instructions::OpCode;

    struct TestInstruction
    {
        value: u8
    }

    impl InstructionWrite for TestInstruction {
        fn write<W: Write>(&self, writer: &mut W) {
            writer.write(&[self.value]).unwrap();
        }
    }

    #[test]
    fn test_write_instruction() {
        let mut chunk = Chunk::new();
        let instruction = TestInstruction { value: 1};

        chunk.write_instruction(instruction, 123);

        assert_eq!(chunk.code.len(), 1);
        assert_eq!(chunk.code[0], 1u8);
        let lines: Vec<&usize> = chunk.lines.into_iter().collect();
        assert_eq!(lines.len(), 1);
        assert_eq!(lines[0], &123);
    }

    #[test]
    fn test_write_multiple_instruction() {
        let mut chunk = Chunk::new();
        let instruction1 = TestInstruction { value: 1};
        let instruction2 = TestInstruction { value: 2};

        chunk.write_instruction(instruction1, 123);
        chunk.write_instruction(instruction2, 124);

        assert_eq!(chunk.code.len(), 2);
        assert_eq!(chunk.code[0], 1u8);
        assert_eq!(chunk.code[1], 2u8);
        let lines: Vec<&usize> = chunk.lines.into_iter().collect();
        assert_eq!(lines.len(), 2);
        assert_eq!(lines[0], &123);
        assert_eq!(lines[1], &124);
    }

    #[test]
    fn test_write_constant() {
        let mut chunk = Chunk::new();
        let value = Value::NumberValue(42.0);

        let result = chunk.write_constant(value, 123);
        assert!(result.is_ok(), "Expected ok result when writing constant");

        assert_eq!(chunk.constants.len(), 1);
        match chunk.constants[0] {
            Value::NumberValue(val) => {
                assert_eq!(val, 42.0);
            },
            _ => {
                assert!(false, "Expected NumberValue");
            }
        }

        assert_eq!(chunk.code.len(), 2);
        assert_eq!(chunk.code[0], OpCode::Constant.as_byte());
        assert_eq!(chunk.code[1], 0u8);  // Index into constant array

        let lines: Vec<&usize> = chunk.lines.into_iter().collect();
        assert_eq!(lines.len(), 2);
        assert_eq!(lines[0], &123);
        assert_eq!(lines[1], &123);
    }

    #[test]
    fn test_write_more_than_256_constants() {
        let mut chunk = Chunk::new();

        for i in 0..257 {
            let value = Value::NumberValue(i as f64);
            let result = chunk.write_constant(value, i);
            assert!(result.is_ok(), "Expected ok result when writing constant");
        }

        assert_eq!(chunk.constants.len(), 257);
        for i in 0..257 {
            match chunk.constants[i] {
                Value::NumberValue(val) => {
                    assert_eq!(val, i as f64);
                },
                _ => {
                    assert!(false, "Expected NumberValue");
                }
            }
        }

        // 2 bytes for first 256 values then 1 byte for op code + 4 for index
        assert_eq!(chunk.code.len(), 256 * 2 + 1 + 4);
        assert_eq!(chunk.code[256 * 2], OpCode::ConstantLong.as_byte());
        let mut cursor = Cursor::new(&chunk.code);
        cursor.set_position(256 * 2 + 1);
        let index = cursor.read_u32::<LittleEndian>().unwrap();
        assert_eq!(index, 256u32);  // Index into constant array
    }
}
