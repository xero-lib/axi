use axi_core::{Lexer, Opcode, Parser, Precedence, VM, num::Tensor};
use std::io::{self, Write};

pub fn init() {
    println!("Type 'exit' to quit.\n");

    let stdin = io::stdin();
    let mut input = String::new();

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        input.clear();
        if stdin.read_line(&mut input).is_err() {
            break;
        }

        let source = input.trim();
        if source == "exit" || source == "quit" {
            break;
        }
        if source.is_empty() {
            continue;
        }

        let lexer = Lexer::new(source);
        let mut parser = Parser::new(lexer);

        parser.parse_expression(Precedence::None);
        parser.chunk.emit_byte(Opcode::Return as u8);

        let mut vm = VM::new(&parser.chunk);

        match vm.run() {
            Ok(result) => {
                if let Tensor::Scalar(num) = result {
                    println!("{}", num.real);
                }
            }
            Err(e) => println!("  VM Error: {}", e),
        }
    }
}
