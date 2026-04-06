use axi_core::{Lexer, Opcode, Optimizer, Parser, Precedence, VM, num::Tensor};

fn eval(source: &str) -> f64 {
    let lexer = Lexer::new(source);
    let mut parser = Parser::new(lexer);
    parser.parse_expression(Precedence::None);
    parser.chunk.emit_byte(Opcode::Return as u8);
    let mut optimizer = Optimizer::new(&mut parser.chunk);
    optimizer.optimize();
    let mut vm = VM::new(&parser.chunk);
    match vm.run() {
        Ok(Tensor::Scalar(n)) => n.real,
        _ => panic!("evaluation failed"),
    }
}

#[test]
fn test_basic_arithmetic() {
    assert_eq!(eval("1+2"), 3.0);
    assert_eq!(eval("10-3"), 7.0);
    assert_eq!(eval("2*3"), 6.0);
    assert_eq!(eval("10/2"), 5.0);
}

#[test]
fn test_precedence() {
    assert_eq!(eval("2+3*4"), 14.0);
    assert_eq!(eval("2*3+4"), 10.0);
}

#[test]
fn test_grouping() {
    assert_eq!(eval("(2+3)*4"), 20.0);
}

#[test]
fn test_implicit_multiply() {
    assert_eq!(eval("2(3+2)"), 10.0);
    assert_eq!(eval("1+2+3+4+5+6+7+8+9(3+2)"), 81.0);
}

#[test]
fn test_unary_negate() {
    assert_eq!(eval("-3"), -3.0);
    assert_eq!(eval("-(2+3)"), -5.0);
}

#[test]
fn test_constant_folding() {
    // result should be folded to a single constant
    assert_eq!(eval("1+2+3+4+5+6+7+8+9"), 45.0);
}

#[test]
fn test_nested_grouping() {
    assert_eq!(eval("((2+3)*4)+1"), 21.0);
}
