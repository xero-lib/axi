/* Super preliminary and not at all what final will look like, just getting something hacked together to start. */
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Token<'a> {
    Add,
    Subtract,
    Multiply,
    Divide,
    Power,

    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,

    Number(f64),
    Identifier(&'a str),

    EOF,

    Error(&'static str),
}

pub struct Lexer<'a> {
    source: &'a [u8],
    cursor: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            source: input.as_bytes(),
            cursor: 0,
        }
    }

    #[inline]
    fn is_at_end(&self) -> bool {
        self.cursor >= self.source.len()
    }

    // pub fn peek(&self) -> u8 {
    //     if self.is_at_end() {
    //         return 0;
    //     }
    //     self.source[self.cursor]
    // }

    pub fn next(&mut self) -> u8 {
        if self.is_at_end() {
            return 0;
        }

        let byte = self.source[self.cursor];
        self.cursor += 1;
        byte
    }

    fn skip_whitespace(&mut self) {
        while !self.is_at_end() {
            match self.source[self.cursor] {
                b' ' | b'\r' | b'\t' | b'\n' => self.cursor += 1,
                _ => break,
            }
        }
    }

    fn parse_number(&mut self) -> Token<'a> {
        let start = self.cursor - 1;

        while !self.is_at_end() {
            match self.source[self.cursor] {
                b'0'..=b'9' | b'.' => self.cursor += 1,
                _ => break,
            }
        }

        let utf_str = core::str::from_utf8(&self.source[start..self.cursor])
            .expect("Failed to parse verified utf-8: should be unreachable");

        match utf_str.parse::<f64>() {
            Ok(val) => Token::Number(val),
            Err(_) => Token::Error("Invalid number format"),
        }
    }

    fn parse_identifier(&mut self) -> Token<'a> {
        let start = self.cursor - 1;

        while !self.is_at_end() {
            match self.source[self.cursor] {
                b'a'..=b'z' | b'A'..=b'Z' | b'0'..=b'9' | b'_' => self.cursor += 1,
                _ => break,
            }
        }

        Token::Identifier(
            core::str::from_utf8(&self.source[start..self.cursor])
                .expect("Failed to parse verified utf-8: should be unreachable"),
        )
    }

    pub fn next_token(&mut self) -> Token<'a> {
        self.skip_whitespace();

        if self.is_at_end() {
            return Token::EOF;
        }

        let byte = self.next();

        match byte {
            b'+' => Token::Add,
            b'-' => Token::Subtract,
            b'*' => Token::Multiply,
            b'/' => Token::Divide,
            b'^' => Token::Power,
            b'(' => Token::LeftParen,
            b')' => Token::RightParen,
            b'{' => Token::LeftBrace,
            b'}' => Token::RightBrace,
            b'[' => Token::LeftBracket,
            b']' => Token::RightBracket,
            b'0'..=b'9' => self.parse_number(),
            b'a'..=b'z' | b'A'..=b'Z' => self.parse_identifier(),
            _ => Token::Error("Unexpected character"),
        }
    }
}
