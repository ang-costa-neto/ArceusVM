use crate::value::Value;

/// A 'Chunk' is a contiguous block of bytecode instructions 
/// and its associated constant pool.
pub struct Chunk {
    pub code: Vec<u8>,
    pub constants: Vec<Value>,
}

impl Chunk {
    pub fn new() -> Self {
        Chunk {
            code: Vec::new(),
            constants: Vec::new(),
        }
    }

    /// Adds a byte (opcode or operand) to the chunk.
    pub fn write(&mut self, byte: u8) {
        self.code.push(byte);
    }

    /// Adds a constant to the pool and returns its index.
    /// This index is later used by OP_CONSTANT.
    pub fn add_constant(&mut self, value: Value) -> u8 {
        self.constants.push(value);
        (self.constants.len() - 1) as u8
    }
}