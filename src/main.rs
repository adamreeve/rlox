#[macro_use]
extern crate enum_primitive_derive;
extern crate num_traits;

mod chunk;
mod debug;
mod value;

use chunk::Chunk;
use chunk::OpCode;

fn main() {
    let mut chunk = Chunk::new();
    chunk.write_chunk(OpCode::Return.as_byte());

    let constant = chunk.add_constant(value::Value::new(1.2));
    chunk.write_chunk(OpCode::Constant.as_byte());
    chunk.write_chunk(constant);

    debug::disassemble_chunk(&chunk, "test chunk");
}
