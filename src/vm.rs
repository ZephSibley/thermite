use crate::chunk::Chunk;

enum InterpretResult {
    InterpretOk,
    InterpretCompileError,
    InterpretRuntimeError,
}

struct VM {
    chunk: Chunk,
    ip: usize,
}

impl VM {
    pub fn new(chunk: Chunk) -> VM {
        VM {
            chunk,
            ip: 0,
        }
    }
}
