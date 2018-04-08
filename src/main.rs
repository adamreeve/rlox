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
use instructions::{OpCode, SimpleInstruction};
use virtual_machine::VirtualMachine;

fn main() {
    let mut chunk = Chunk::new();

    chunk.write_constant(value::Value::new(1.2), 123);
    chunk.write_constant(value::Value::new(3.4), 123);
    chunk.write_instruction(SimpleInstruction::new(OpCode::Add), 123);
    chunk.write_constant(value::Value::new(5.6), 123);
    chunk.write_instruction(SimpleInstruction::new(OpCode::Divide), 123);
    chunk.write_instruction(SimpleInstruction::new(OpCode::Negate), 123);
    chunk.write_instruction(SimpleInstruction::new(OpCode::Return), 123);

    debug::disassemble_chunk(&chunk, "test chunk");

    println!("Interpreting");

    let mut vm = VirtualMachine::new(&chunk);
    vm.interpret();
}
