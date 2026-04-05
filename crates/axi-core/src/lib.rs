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
