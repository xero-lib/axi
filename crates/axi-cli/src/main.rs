use axi_core::{Lexer, Opcode, Optimizer, Parser, Precedence, VM, num::Tensor};
use std::hint::black_box;
use std::time::Instant;

fn main() {
    let source = "1+2+3+4+5+6+7+8+9";
    let iterations: usize = 1 << 26;
    println!("Expression: {}", source);
    println!("Iterations: {}", iterations);

    let lexer = Lexer::new(source);
    let mut parser = Parser::new(lexer);
    parser.parse_expression(Precedence::None);
    parser.chunk.emit_byte(Opcode::Return as u8);

    let mut optimizer = Optimizer::new(&mut parser.chunk);
    optimizer.optimize();

    let mut vm = VM::new(&parser.chunk);
    if let Ok(Tensor::Scalar(res)) = vm.run() {
        println!("Verification Pass: Result = {}", res.real);
    }

    let start = Instant::now();
    for _ in 0..iterations {
        vm.reset(&parser.chunk);
        let _ = black_box(vm.run());
    }

    let duration = start.elapsed();
    let ops_per_sec = (iterations as f64) / duration.as_secs_f64();
    println!("Total Time: {:.2?}", duration);
    println!(
        "Speed: {:.2} million executions/second",
        ops_per_sec / 1_000_000.0
    );
}
