use criterion::{black_box, criterion_group, criterion_main, Criterion};
use arceus_vm::vm::VM;
use arceus_vm::value::Value;
use arceus_vm::common::OpCode;

fn benchmark_simple_addition(c: &mut Criterion) {
    // Setup a simple chunk: 5.0 + 10.5
    let mut code = Vec::new();
    let mut constants = Vec::new();

    constants.push(Value::number(5.0));
    constants.push(Value::number(10.5));

    code.push(OpCode::Constant as u8);
    code.push(0); // index of 5.0
    code.push(OpCode::Constant as u8);
    code.push(1); // index of 10.5
    code.push(OpCode::Add as u8);
    code.push(OpCode::Return as u8);

    c.bench_function("vm_add_5_10", |b| {
        b.iter(|| {
            // We use black_box to prevent the compiler from optimizing 
            // the entire execution away since the inputs are constant.
            let mut vm = VM::new(black_box(code.clone()), black_box(constants.clone()));
            vm.run()
        })
    });
}

criterion_group!(benches, benchmark_simple_addition);
criterion_main!(benches);