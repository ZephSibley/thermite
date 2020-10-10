
use crate::chunk::Chunk;
use crate::chunk::OpCode;

pub fn disassemble_chunk(chunk: &Chunk, name: &str ) {
    println!("== {} ==", name);

    for code in chunk {
        match code {
            OpCode::OpReturn => println!("OpReturn"),
            OpCode::OpConstant(i) => println!(
                "OpConstant {:?}", chunk.constants[*i]
            ),
            _ => println!("{:?}", code)
        }
    }
}
