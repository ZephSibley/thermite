use crate::chunk::Chunk;
use crate::chunk::OpCode;
use crate::chunk::Const

enum InterpretResult {
    InterpretOk,
    InterpretCompileError,
    InterpretRuntimeError,
}

struct VM {
    chunk: Chunk,
    stack: Vec<Const>
}

impl VM {
    pub fn new(chunk: Chunk) -> VM {
        VM {
            chunk,
            stack: Vec::new(),
        }
    }

    pub fn run(self) -> InterpretResult {
        for (i, code) in &self.chunk {
            match code {
                OpCode::OpConstant(ci) => {
                    println!("{:?}\n", self.chunk.constants[*ci])
                },
                OpCode::OpReturn => return InterpretResult::InterpretOk,
            }
        }
        return InterpretResult::InterpretOk;
    }
}
