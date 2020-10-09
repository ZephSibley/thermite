use crate::chunk::Chunk;
use crate::chunk::Const;
use crate::chunk::OpCode;

enum InterpretResult {
    InterpretOk,
    InterpretCompileError,
    InterpretRuntimeError,
}

struct VM {
    chunk: Chunk,
    stack: Vec<Const>,
}

impl VM {
    pub fn new(chunk: Chunk) -> VM {
        VM {
            chunk,
            stack: Vec::new(),
        }
    }

    pub fn run(&mut self) -> InterpretResult {
        for code in &self.chunk {
            match code {
                OpCode::OpConstant(ci) => {
                    println!("{:?}\n", self.chunk.constants[*ci]);
                    self.stack.push(
                        self.chunk.constants[*ci]
                    );
                    break;
                }
                OpCode::OpNegate => {
                    match self.stack.pop() {
                        None => return InterpretResult::InterpretCompileError,
                        Some(Const::Float(n)) => self.stack.push(Const::Float(-n)),
                    }
                    break;
                }
                OpCode::OpReturn => {
                    println!("{:?}", self.stack.pop());
                    return InterpretResult::InterpretOk;
                }
            }
        }
        return InterpretResult::InterpretOk;
    }
}
