mod chunk;
mod debug;

use chunk::Chunk;
use chunk::OpCode::OpReturn;
use chunk::Const::Float;

fn main() {
    let mut chunk = Chunk::new();
    chunk.add_code(OpReturn);
    chunk.add_constant(Float(1.2));

    debug::disassemble_chunk(chunk, "test");

    println!("== Done! ==")
}
