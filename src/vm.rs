use crate::chunk::Chunk;
use crate::chunk::Const;
use crate::chunk::OpCode;

pub enum InterpretResult {
    InterpretOk,
    InterpretCompileError,
    //InterpretRuntimeError,
}

pub struct VM {
    pub chunk: Chunk,
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
                    println!("{:?}", self.chunk.constants[*ci]);
                    self.stack.push(
                        self.chunk.constants[*ci]
                    );
                },
                OpCode::OpAdd => {
                    let (left, right) = self.unroll_operands();
                    self.stack.push(
                        left + right
                    );
                },
                OpCode::OpNegate => {
                    match self.stack.pop() {
                        None => return InterpretResult::InterpretCompileError,
                        Some(Const::Float(n)) => self.stack.push(Const::Float(-n)),
                    }
                }
                OpCode::OpReturn => {
                    println!("{:?}", self.stack.pop());
                    return InterpretResult::InterpretOk;
                }
            }
        }
        println!("End Run");
        return InterpretResult::InterpretOk;
    }

    fn unroll_operands(self) -> (Const, Const) {
        let right = self.stack.pop().unwrap();
        let left = self.stack.pop().unwrap();
        return (left, right);
    }
}
