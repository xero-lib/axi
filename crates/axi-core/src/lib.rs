#![no_std]

extern crate alloc;

mod chunk;
mod evaluator;
mod lexer;
mod optimizer;
mod parser;
mod vm;

pub use axi_num as num;
pub use chunk::Chunk;
pub use lexer::{Lexer, Token};
pub use optimizer::Optimizer;
pub use parser::{Parser, Precedence};
pub use vm::{Opcode, VM};

use axi_num::{Number, Tensor};

pub fn eval(input: &str) -> Result<Number, &'static str> {
    let lexer = Lexer::new(input);
    let mut parser = Parser::new(lexer);
    parser.parse_expression(Precedence::None);
    parser.chunk.emit_byte(Opcode::Return as u8);
    let mut optimizer = Optimizer::new(&mut parser.chunk);
    optimizer.optimize();
    let mut vm = VM::new(&parser.chunk);
    match vm.run() {
        Ok(Tensor::Scalar(n)) => Ok(n),
        _ => Err("Evaluation failed"),
    }
}