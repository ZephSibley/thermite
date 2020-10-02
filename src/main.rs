mod chunk;
use chunk::Chunk;
use chunk::OpCode::OpReturn;

fn main() {
    let chunk = Chunk {
        code: vec![OpReturn]
    };
    println!("{}", chunk.code.len());
    println!("Done!");
}
