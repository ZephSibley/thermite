#[derive(Debug)]
pub enum OpCode {
    OpReturn,
    OpConstant(usize),
}

#[derive(Debug)]
pub enum Const {
    Float(f64),
}

#[derive(Debug)]
pub struct Chunk {
    pub codes: Vec<OpCode>,
    pub constants: Vec<Const>,
    //lines: u64,
}


impl Chunk {
    pub fn new() -> Chunk {
        Chunk {
            codes: Vec::new(),
            constants: Vec::new(),
        }
    }

    pub fn add_code(&mut self, code: OpCode) {
        self.codes.push(code);
    }

    pub fn add_constant(&mut self, value: Const) {
        let value_index = self.constants.len();
        self.constants.push(value);
        self.codes.push(OpCode::OpConstant(value_index));
    }
}
