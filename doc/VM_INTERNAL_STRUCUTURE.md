# ArceusVM: Internal Architecture & Design
This document details the internal memory layout, execution model, and state management of the ArceusVM. Our goal is to achieve high throughput with minimal overhead by staying as close to the hardware as possible.

## 1. The VM State (Virtual CPU)
In Rust, the VM is encapsulated in a struct that acts as the CPU's register set and memory controller. We avoid global state to allow for thread-safe isolates (multiple VMs running in the same process).

### Core Components
|Component|Type|Description|
|--|--|--|
|`IP` (Instruction Pointer)|`*const u8`|A raw pointer to the next bytecode instruction to be executed.|
|`Stack`|`[Value; STACK_MAX]`|A contiguous array of `Values`. The primary storage for intermediate computations.|
|`Stack Top`|`*mut Value`|A pointer to the next empty slot in the stack. Faster than index-based access.|
|`Constants`|`Vec<Value>`|A pool of literal values (numbers, strings) loaded at compile time.|

*Principal Insight*: Using raw pointers for `IP` and `stack_top` instead of array indices (`usize`) eliminates bounds checking in the hot path, effectively mimicking how a physical CPU's registers work.

## 2. Memory Layout: The Stack
ArceusVM is a Stack Machine. Unlike Register Machines, which use named registers, we push and pop values from a LIFO (Last-In, First-Out) structure.

### Cache Locality
The stack is pre-allocated as a contiguous block of memory. This ensures that most operations happen within the *L1 Data Cache*, as the `stack_top` pointer usually moves within a small memory range.

### Stack Safety
To prevent *Stack Overflow* vulnerabilities (common in low-level VMs), the `push()` operation includes an assertion in debug builds, while release builds rely on the compiler's height analysis to ensure the stack depth never exceeds `STACK_MAX`.

## 3. The Execution Loop (The Dispatcher)
The heart of the VM is the `run()` function. It implements a Linear Dispatch model.
```rust
fn run(&mut self) -> InterpretResult {
    loop {
        let instruction = unsafe { self.read_byte() };
        match instruction {
            OP_CONSTANT => {
                let constant = self.read_constant();
                self.push(constant);
            }
            OP_ADD => {
                let b = self.pop();
                let a = self.pop();
                self.push(a + b);
            }
            OP_RETURN => return InterpretResult::Ok,
            _ => return InterpretResult::RuntimeError,
        }
    }
}
```

### Dispatch Optimizations
- *Match-to-Lookup*: Rust's `match` on an `enum` with sequential values is typically compiled into a *Jump Table*, which is highly efficient
- *Inlining*: Critical operations like `push` and `pop` are marked with `#[inline(always)]` to remove function call overhead.

## 4. Value Representation (The "Cell")
The `Value` type is the most frequently accessed structure in the VM.
```rust
pub enum Value {
    Number(f64),
    Boolean(bool),
    Nil,
    // Future: Obj(Box<ObjHeap>)
}
```
_Current Implementation_: We use a *Tagged Union* (Rust's `enum`).
_Future Evolution_: To reach Google-level performance, we will transition to *NaN-Boxing*, representing all types within a single 64-bit `u64`, reducing the `Value` size and improving cache density.

## 5. Error Handling & Safety
ArceusVM differentiates between two types of errors:
1. *Compile-time Errors*: Caught by the Parser/Compiler (Syntax, invalid tokens).
2. *Runtime Errors*: Caught by the VM (Stack underflow, type mismatch, division by zero).

When a Runtime Error occurs, the VM provides a *StackTrace* by calculating the offset of the `IP` relative to the start of the bytecode chunk.