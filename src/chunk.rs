use ::value::Value;
use ::instructions::*;
use ::util::RunLengthEncoded;

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

    pub fn write_constant(&mut self, value: Value, line: usize) {
        self.constants.push(value);
        let constant_index = self.constants.len() - 1;
        // Once we have over 256 constants, we need to start
        // saving constants using a constant long instruction:
        if constant_index > 255  {
            self.write_instruction(ConstantLongInstruction::new(constant_index as u32), line);
        }
        else
        {
            self.write_instruction(ConstantInstruction::new(constant_index as u8), line);
        }
    }
}
