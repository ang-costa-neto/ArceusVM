# ArceusVM
A high-performance, stack-based Virtual Machine and Bytecode Compiler built for safety and extreme predictability.

ArceusVM is not just another toy language. It is an exploration of low-level systems engineering, designed to demonstrate the implementation of a custom Instruction Set Architecture (ISA), a Virtual Machine with optimized dispatch, and a Data-Oriented approach to memory management.

## Architectural Overview

ArceusVM follows the classic "Two-Stage" execution model but is optimized for the modern CPU cache hierarchy.

1. Compiler Front-end: A hand-written recursive-descent parser that transforms source code into an Abstract Syntax Tree (AST), performing constant folding and basic semantic analysis.
2. Bytecode Emitter: Flattens the AST into a linear stream of compact 1-byte instructions (Opcodes).
3. VM Back-end: A stack-based execution engine written in safe, idiomatic Rust, utilizing a high-speed dispatch loop.

## Key Technical Features (The "Principal" Touch)

- Custom Bytecode ISA: A compact instruction set designed for high density and low fetch overhead.
- NaN-Boxing (Planned): Leveraging IEEE 754 floating-point representation to store pointers, booleans, and integers within a single 64-bit word, minimizing memory pressure.
- Zero-Copy Dispatch: The VM's instruction pointer (IP) interacts directly with the bytecode buffer, avoiding unnecessary allocations during the "Hot Path".
- Safety-First Memory: Built in Rust to guarantee no memory leaks or data races in the VM's core, without sacrificing the raw performance of a C implementation.

## The Instruction Set (Preview)

| OpCode | Hex | Description | Stack Effect |
|--|--|--|--|
|OP_CONSTANT|0x01|Loads a constant from the pool|`[ ] -> [v]`|
|OP_ADD|0x03|Binary addition|`"[a, b] -> [a+b]"`|
|OP_JUMP_IF_FALSE|0x08|Conditional branching|`[cond] -> [ ]`|
|OP_RETURN|0x0F|Exits the current frame|`[v] -> exit`|

## Design Decisions & Trade-offs

### Why a Stack-based VM?
While Register-based VMs (like LuaJIT or Dalvik) can be faster due to reduced instruction count, a Stack-based VM was chosen for ArceusVM to:

1. Reduce Compiler Complexity: Simplifying the translation from AST to Bytecode without the overhead of complex Register Allocation algorithms (like Graph Coloring).
2. Code Density: Bytecode instructions are smaller, which is crucial for staying within the L1 Instruction Cache.

### The Role of Rust
Rust was selected over C++ to leverage the Borrow Checker. By using Rust's ownership model, we ensure that the VM's internal stack and the Garbage Collector (when implemented) cannot suffer from "Use-After-Free" or "Double-Free" vulnerabilities by design.

## Getting Start
```bash
    # Clone the repository
    git clone https://github.com/seu-usuario/arceus-vm

    # Build in release mode for maximum performance
    cargo build --release

    # Run a sample script
    ./target/release/arceus run examples/fibonacci.arc
```