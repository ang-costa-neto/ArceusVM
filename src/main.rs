use arceus_vm::chunk::Chunk;
use arceus_vm::vm::{VM, InterpretResult};
use arceus_vm::compiler::Compiler;

fn main() {
    println!("--- ArceusVM: High-Performance Language Shell ---");

    // This source string will soon be read from a .arc file
    let source = "print 5.5 + 10.5;";

    // 1. Initialize the shared Bytecode Chunk
    let mut chunk = Chunk::new();

    // 2. Initialize the Compiler with the source code
    let mut compiler = Compiler::new(source);

    println!("Compiling source...");

    // 3. Run the Compilation phase
    // For now, it only scans tokens. In the next step, it will emit bytecode.
    if !compiler.compile(&mut chunk) {
        eprintln!("Compilation failed.");
        std::process::exit(65); // Standard exit code for data format error
    }

    println!("Executing...");

    // 4. Hand over the compiled chunk to the VM
    let mut vm = VM::new(chunk.code, chunk.constants);
    
    match vm.run() {
        InterpretResult::Ok => println!("\nExecution finished successfully."),
        InterpretResult::RuntimeError => {
            eprintln!("Runtime error occurred.");
            std::process::exit(70); // Standard exit code for internal software error
        }
        InterpretResult::CompileError => unreachable!(), // Handled in the compiler phase
    }
}