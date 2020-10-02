pub enum OpCode {
    OpReturn,
}

pub struct Chunk {
    //lines: u64,
    pub code: Vec<OpCode>,
    //constants: Vec<f64>,
}
