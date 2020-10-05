
use crate::chunk::Chunk;
use crate::chunk::OpCode;

pub fn disassemble_chunk(chunk: Chunk, name: &str ) {
    println!("== {} ==", name);

    for code in chunk.codes.iter() {
        match code {
            OpCode::OpReturn => println!("OpReturn"),
            // Todo: Fix the hardcoded index here or figure out the builtin debugger.
            OpCode::OpConstant(i) => println!(
                "OpConstant {:?}", chunk.constants[0]
            ),
        }
    }
}
