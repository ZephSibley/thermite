use crate::constants::Const;

#[derive(Debug)]
pub enum OpCode {
    OpConstant(usize),
    OpAdd,
    OpSubtract,
    OpMultiply,
    OpDivide,
    OpNegate,
    OpReturn,
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

impl<'a> IntoIterator for &'a Chunk {
    type Item = &'a OpCode;
    type IntoIter = ChunkIntoIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        ChunkIntoIterator {
            chunk: self,
            index: 0,
        }
    }
}

pub struct ChunkIntoIterator<'a> {
    chunk: &'a Chunk,
    index: usize,
}

// TODO: Test to see if this is more efficient when using Copy instead
// of passing by reference
impl<'a> Iterator for ChunkIntoIterator<'a> {
    
    type Item = &'a OpCode;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index == self.chunk.codes.len() {
            return None
        }
        let result = &self.chunk.codes[self.index];
        self.index += 1;
        return Some(result);
    }
}
