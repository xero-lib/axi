use std::time::Instant;
// Make sure these match your actual imports
use axi_core::{Lexer, Parser, VM, Precedence, Opcode, num::Tensor };

fn main() {
    let source = "1+2+3+4+5+6+7+8+9";
    let iterations: usize = 1 << 26; 

    println!("Expression: {}", source);
    println!("Iterations: {}", iterations);

    let lexer = Lexer::new(source);
    let mut parser = Parser::new(lexer);
    
    parser.parse_expression(Precedence::None);
    parser.emit_byte(Opcode::Return as u8);

    let chunk = &parser.chunk[..parser.chunk_len];
    let constants = &parser.constants;

    let mut test_vm = VM::new(chunk, constants);
    if let Ok(Tensor::Scalar(res)) = test_vm.run() {
        println!("Verification Pass: Result = {}", res.real);
    }

    let start = Instant::now();

    let mut vm = VM::new(chunk, constants);
    for _ in 0..iterations {
        vm.reset(chunk, constants);
        let _ = vm.run(); 
    }

    let duration = start.elapsed();
    let ops_per_sec = (iterations as f64) / duration.as_secs_f64();

    println!("Total Time: {:.2?}", duration);
    println!("Speed: {:.2} million executions/second", ops_per_sec / 1_000_000.0);
}