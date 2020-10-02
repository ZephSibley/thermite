
use crate::chunk::Chunk;
use crate::chunk::OpCode;

pub fn disassemble_chunk(chunk: &Chunk, name: &str ) {
    println!("== {} ==", name);

    for code in chunk.code.iter() {
        disassemble_instruction(code);
    }
}

fn disassemble_instruction(code: &OpCode) {
    match code {
        OpCode::OpReturn => println!("OpReturn"),
    }
}
