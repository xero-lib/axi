#![no_std]

extern crate alloc;

mod evaluator;
mod lexer;
mod parser;
mod vm;

pub use axi_num as num;
pub use vm::{ VM, Opcode };
pub use lexer::{ Lexer, Token };
pub use parser::{ Parser, Precedence };