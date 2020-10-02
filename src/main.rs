mod chunk;
mod debug;

use chunk::Chunk;
use chunk::OpCode::OpReturn;

fn main() {
    let chunk = Chunk {
        code: vec![OpReturn]
    };

    debug::disassemble_chunk(&chunk, "test");
    
    println!("len: {}", chunk.code.len());
    println!("== Done! ==")
}
