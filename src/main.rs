#[macro_use]
extern crate enum_primitive_derive;
extern crate num_traits;

mod chunk;
mod debug;

use chunk::Chunk;
use chunk::OpCode;

fn main() {
    let mut chunk = Chunk::new();
    chunk.write_chunk(OpCode::Return.as_byte());

    debug::disassemble_chunk(&chunk, "test chunk");
}
