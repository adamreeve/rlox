extern crate byteorder;
#[macro_use]
extern crate enum_primitive_derive;
extern crate num_traits;

mod chunk;
mod instructions;
mod debug;
mod util;
mod value;
mod virtual_machine;

use chunk::Chunk;
use instructions::ReturnInstruction;
use virtual_machine::VirtualMachine;

fn main() {
    let mut chunk = Chunk::new();

    chunk.write_constant(value::Value::new(0.123), 123);
    chunk.write_instruction(ReturnInstruction::new(), 123);

    debug::disassemble_chunk(&chunk, "test chunk");

    println!("Interpreting");

    let mut vm = VirtualMachine::new(&chunk);
    vm.interpret();
}
