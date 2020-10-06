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
    lines: Vec<usize>,
}


impl Chunk {
    pub fn new() -> Chunk {
        Chunk {
            codes: Vec::new(),
            constants: Vec::new(),
            lines: Vec::new(),
        }
    }

    pub fn add_code(&mut self, code: OpCode, line: usize) {
        self.codes.push(code);
        self.lines.push(line);
    }

    pub fn add_constant(&mut self, value: Const, line: usize) {
        let value_index = self.constants.len();
        self.constants.push(value);
        self.codes.push(OpCode::OpConstant(value_index));
        self.lines.push(line);
    }

    pub fn get_line(self, index: usize) -> usize {
        return self.lines[index]
    }
}
