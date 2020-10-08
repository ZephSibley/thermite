use crate::chunk::Chunk;
use crate::chunk::OpCode;

enum InterpretResult {
    InterpretOk,
    InterpretCompileError,
    InterpretRuntimeError,
}

struct VM {
    chunk: Chunk,
}

impl VM {
    pub fn new(chunk: Chunk) -> VM {
        VM {
            chunk,
        }
    }

    pub fn run(self) -> InterpretResult {
        for (i, code) in &self.chunk {
            match code {
                OpCode::OpReturn => return InterpretResult::InterpretOk,
                OpCode::OpConstant(c) => return InterpretResult::InterpretOk,
            }
        }
        return InterpretResult::InterpretOk;
    }
}
