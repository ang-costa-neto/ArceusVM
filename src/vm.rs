use crate::common::OpCode;
use crate::value::Value;

/// Maximum stack size to avoid dynamic allocation during runtime.
/// 256 is enough for most nested expressions.
const STACK_MAX: usize = 256;

/// Possible outcomes of the VM execution.
#[derive(Debug)]
pub enum InterpretResult {
    Ok,
    CompileError,
    RuntimeError,
}

/// The Virtual Machine state.
pub struct VM {
    /// Instruction Pointer: points to the next byte to be executed.
    /// In a production VM, this would be a raw pointer for speed.
    ip: usize,
    /// The bytecode to execute.
    chunk: Vec<u8>,
    /// The constant pool for the current chunk.
    constants: Vec<Value>,
    /// The data stack for intermediate values.
    stack: [Value; STACK_MAX],
    /// Points to the next empty slot in the stack.
    stack_top: usize,
}

impl VM {
    pub fn new(chunk: Vec<u8>, constants: Vec<Value>) -> Self {
        VM {
            ip: 0,
            chunk,
            constants,
            stack: [Value::nil(); STACK_MAX], // Initialize stack with Nil
            stack_top: 0,
        }
    }

    /// The main execution loop (The Dispatcher).
    pub fn run(&mut self) -> InterpretResult {
        loop {
            // Fetch the next instruction
            let instruction = self.read_byte();
            
            // Decode and Execute
            match OpCode::from(instruction) {
                OpCode::Constant => {
                    let constant = self.read_constant();
                    self.push(constant);
                }
                OpCode::Add => {
                    let b = self.pop();
                    let a = self.pop();
                    
                    if a.is_number() && b.is_number() {
                        self.push(Value::number(a.as_number() + b.as_number()));
                    } else {
                        return self.runtime_error("Operands must be numbers.");
                    }
                }
                OpCode::Subtract => {
                    let b = self.pop();
                    let a = self.pop();
                    self.push(Value::number(a.as_number() - b.as_number()));
                }
                OpCode::Multiply => {
                    let b = self.pop();
                    let a = self.pop();
                    self.push(Value::number(a.as_number() * b.as_number()));
                }
                OpCode::Divide => {
                    let b = self.pop();
                    let a = self.pop();
                    self.push(Value::number(a.as_number() / b.as_number()));
                }
                OpCode::Negate => {
                    if !self.peek(0).is_number() {
                        return self.runtime_error("Operand must be a number.");
                    }
                    let val = self.pop();
                    self.push(Value::number(-val.as_number()));
                }
                OpCode::Return => {
                    // In a simple CLI, we print the result of the last expression.
                    println!("Result: {}", self.pop());
                    return InterpretResult::Ok;
                }
            }
        }
    }

    // --- Helper Methods (Internal) ---

    #[inline(always)]
    fn read_byte(&mut self) -> u8 {
        let byte = self.chunk[self.ip];
        self.ip += 1;
        byte
    }

    #[inline(always)]
    fn read_constant(&mut self) -> Value {
        // The index of the constant is stored in the next byte of the chunk.
        let index = self.read_byte() as usize;
        self.constants[index]
    }

    #[inline(always)]
    fn push(&mut self, value: Value) {
        if self.stack_top >= STACK_MAX {
            panic!("Stack Overflow!");
        }
        self.stack[self.stack_top] = value;
        self.stack_top += 1;
    }

    #[inline(always)]
    fn pop(&mut self) -> Value {
        self.stack_top -= 1;
        self.stack[self.stack_top]
    }

    #[inline(always)]
    fn peek(&self, distance: usize) -> Value {
        self.stack[self.stack_top - 1 - distance]
    }

    fn runtime_error(&self, message: &str) -> InterpretResult {
        eprintln!("Runtime Error: {}", message);
        // Here we could add stack trace information later.
        InterpretResult::RuntimeError
    }
}