use arceus_vm::common::OpCode;
use arceus_vm::value::Value;
use arceus_vm::chunk::Chunk;
use arceus_vm::vm::VM;

fn main() {
    println!("--- ArceusVM: High-Performance Execution Test ---");

    // 1. Create a new chunk of code
    let mut chunk = Chunk::new();

    // 2. Add constants to the pool (5.0 and 10.5)
    let constant_a = chunk.add_constant(Value::number(5.0));
    let constant_b = chunk.add_constant(Value::number(10.5));

    // 3. Write instructions to the chunk
    // Load 5.0
    chunk.write(OpCode::Constant as u8);
    chunk.write(constant_a);

    // Load 10.5
    chunk.write(OpCode::Constant as u8);
    chunk.write(constant_b);

    // Add them together (5.0 + 10.5)
    chunk.write(OpCode::Add as u8);

    // Return and print result
    chunk.write(OpCode::Return as u8);

    // 4. Initialize the VM with our bytecode and constants
    let mut vm = VM::new(chunk.code, chunk.constants);

    // 5. Run the engine!
    println!("Executing bytecode...");
    vm.run();
}