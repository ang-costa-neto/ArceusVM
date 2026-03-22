pub mod common;
pub mod value;
pub mod chunk;
pub mod vm;
pub mod compiler;

// Re-export common types for easier access across the crate.
pub use common::OpCode;
pub use value::Value;
pub use vm::VM;