extern crate byteorder;
#[macro_use]
extern crate enum_primitive_derive;
extern crate num_traits;

mod chunk;
mod instructions;
mod debug;
mod util;
mod value;

use chunk::Chunk;
use instructions::ReturnInstruction;

fn main() {
    let mut chunk = Chunk::new();

    for i in 0..30 {
        for j in 0..10 {
            chunk.write_constant(value::Value::new(((i * 10 + j) * 2) as f64), 123 + i);
        }
    }

    chunk.write_instruction(ReturnInstruction::new(), 123);

    debug::disassemble_chunk(&chunk, "test chunk");
}
