/// Instruction Set Architecture (ISA) for ArceusVM.
/// Each OpCode is represented as a single byte (u8).
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OpCode {
    /// Loads a constant from the constant pool onto the stack.
    Constant, 
    /// Pop two values, add them, and push the result.
    Add,      
    /// Pop two values, subtract them, and push the result.
    Subtract,
    /// Pop two values, multiply them, and push the result.
    Multiply,
    /// Pop two values, divide them, and push the result.
    Divide,
    /// Negate the value at the top of the stack (unary minus).
    Negate,   
    /// Finish execution and return from the current frame.
    Return,   
}

impl From<u8> for OpCode {
    /// Converts a raw byte back into an OpCode. 
    /// This is used during the VM's fetch-decode cycle.
    fn from(value: u8) -> Self {
        // Safe because our OpCode is #[repr(u8)] and we control the range.
        unsafe { std::mem::transmute(value) }
    }
}