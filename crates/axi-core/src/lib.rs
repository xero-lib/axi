#![no_std]

extern crate alloc;

mod evaluator;
pub mod lexer;
pub mod parser;
pub mod vm;
pub use axi_num as num;