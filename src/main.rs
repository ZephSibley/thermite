mod chunk;
mod constants;
mod debug;
mod vm;

use chunk::Chunk;
use chunk::OpCode;
use constants::Const::Float;
use vm::VM;

fn main() {
    let chunk = Chunk::new();
    let mut vm = VM::new(chunk);
    
    vm.chunk.add_constant(Float(1.2), 1);
    vm.chunk.add_constant(Float(1.2), 1);
    vm.chunk.add_code(OpCode::OpAdd, 2);
    vm.chunk.add_constant(Float(2.5), 2);
    vm.chunk.add_code(OpCode::OpMultiply, 3);
    vm.chunk.add_code(OpCode::OpNegate, 3);
    vm.chunk.add_code(OpCode::OpReturn, 4);

    vm.run();

    debug::disassemble_chunk(&vm.chunk, "test");

    println!("== Done! ==")
}
