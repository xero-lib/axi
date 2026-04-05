use axi_num::Number;

use crate::{
    lexer::{Lexer, Token},
    vm::Opcode,
};

#[derive(PartialEq, PartialOrd)]
pub enum Precedence {
    None,
    Assignment,
    Term,     
    Factor, 
    Unary,
    Power, 
    Primary,
}

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    current: Token<'a>,
    peek: Token<'a>,

    pub chunk: [u8; 512],
    pub chunk_len: usize,

    pub constants: [Number; 64],
    pub constants_len: usize,
}

impl<'a> Parser<'a> {
    pub fn new(mut lexer: Lexer<'a>) -> Self {
        let current = lexer.next_token();
        let peek = lexer.next_token();

        Parser {
            lexer,
            current,
            peek,
            chunk: [0; 512],
            chunk_len: 0,
            constants: [Number::from(0.0); 64],
            constants_len: 0,
        }
    }
    pub fn peek(&self) -> &Token<'a> {
        &self.peek
    }

    pub fn next(&mut self) {
        self.current = *self.peek();
        self.peek = self.lexer.next_token();
    }

    pub fn emit_byte(&mut self, byte: u8) {
        self.chunk[self.chunk_len] = byte;
        self.chunk_len += 1;
    }

    fn add_constant(&mut self, val: Number) -> usize {
        for i in 0..self.constants_len {
            if self.constants[i] == val {
                return i;
            }
        }

        let index = self.constants_len;
        self.constants[index] = val;
        self.constants_len += 1;
        index
    }

    fn emit_number(&mut self) {
        if let Token::Number(val) = self.current {
            let num = Number {
                real: val,
                imag: 0.0,
            };

            let index = self.add_constant(num);

            self.emit_byte(Opcode::Constant as u8);
            self.emit_byte(index as u8);
        }
    }

    fn parse_grouping(&mut self) {
        self.next();

        self.parse_expression(Precedence::None);

        if self.peek != Token::RightParen {
            panic!("Expected ')' after expression");
        }

        self.next();
    }

    fn parse_unary(&mut self) {
        let operator_type = self.current;

        self.parse_expression(Precedence::Unary);

        match operator_type {
            Token::Subtract => self.emit_byte(Opcode::Negate as u8),
            _ => return,
        }
    }

    fn parse_binary(&mut self, precedence: Precedence) {
        let operator_type = self.current;

        self.next();

        self.parse_expression(precedence);

        match operator_type {
            Token::Add => self.emit_byte(Opcode::Add as u8),
            Token::Subtract => self.emit_byte(Opcode::Subtract as u8),
            Token::Multiply => self.emit_byte(Opcode::Multiply as u8),
            Token::Divide => self.emit_byte(Opcode::Divide as u8),
            _ => return,
        }
    }

    fn get_precedence(&self, token: Token) -> Precedence {
        match token {
            Token::Add | Token::Subtract => Precedence::Term,
            Token::Multiply | Token::Divide => Precedence::Factor,
            Token::Power => Precedence::Power,
            _ => Precedence::None,
        }
    }

    fn prefix_dispatch(&mut self) {
        match self.current {
            Token::Number(_) => self.emit_number(),
            Token::Subtract => self.parse_unary(),
            Token::LeftParen => self.parse_grouping(),
            _ => panic!("Parser Error: Expected a number or prefix operator, but found {:?}", self.current),
        }
    }

    pub fn parse_expression(&mut self, precedence: Precedence) {
        self.prefix_dispatch();

        while precedence < self.get_precedence(self.peek) {
            self.next(); // move to the operator 
            self.infix_dispatch();
        }
    }

    fn infix_dispatch(&mut self) {
        let prec = self.get_precedence(self.current);
        match self.current {
            Token::Add | Token::Subtract | Token::Multiply | Token::Divide => {
                self.parse_binary(prec);
            }
            _ => panic!("Unknown infix operator"),
        }
    }
}
