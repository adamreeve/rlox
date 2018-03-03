extern crate byteorder;
#[macro_use]
extern crate enum_primitive_derive;
extern crate num_traits;

mod chunk;
mod debug;
mod util;
mod value;

use chunk::Chunk;
use chunk::OpCode;

fn main() {
    let mut chunk = Chunk::new();

    for i in 0..260 {
        chunk.write_constant(value::Value::new((i * 2) as f64), 123);
    }

    chunk.write_chunk(OpCode::Return.as_byte(), 123);

    debug::disassemble_chunk(&chunk, "test chunk");
}
