mod chunk;
mod debug;
mod vm;

use chunk::Chunk;
use chunk::OpCode::OpReturn;
use chunk::Const::Float;

fn main() {
    let mut chunk = Chunk::new();
    chunk.add_code(OpReturn, 1);
    chunk.add_constant(Float(1.2), 2);

    debug::disassemble_chunk(chunk, "test");

    println!("== Done! ==")
}
