use core::mem::MaybeUninit;

use axi_num::{Number, Tensor};

use crate::Chunk;

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum Opcode {
    /// End of execution
    Return = 0,
    /// Load constant from pool
    Constant = 1,
    /// Add two numbers
    Add = 2,
    /// Subtract two numbers
    Subtract = 3,
    /// Multiply two numbers
    Multiply = 4,
    /// Divide two numbers
    Divide = 5,
    /// Negate a number
    Negate = 6,
    // Exponent = 8
}

impl From<u8> for Opcode {
    fn from(byte: u8) -> Self {
        match byte {
            0 => Opcode::Return,
            1 => Opcode::Constant,
            2 => Opcode::Add,
            3 => Opcode::Subtract,
            4 => Opcode::Multiply,
            5 => Opcode::Divide,
            6 => Opcode::Negate,
            // 7 => Opcode::Exponent,
            _ => panic!("Encountered invalid operation."),
        }
    }
}

// make configurable later?
const STACK_SIZE: usize = 256;

pub struct VM<'a> {
    bytes: &'a [u8],
    constants: &'a [Number],

    ip: usize,
    stack: [MaybeUninit<Tensor<'a>>; STACK_SIZE],
    sp: usize,
}

impl<'a> VM<'a> {
    pub fn new(chunk: &'a Chunk) -> Self {
        VM {
            bytes: chunk.get_bytes(),
            constants: chunk.get_constants(),
            ip: 0,
            stack: [MaybeUninit::uninit(); STACK_SIZE],
            sp: 0,
        }
    }

    pub fn reset(&mut self, chunk: &'a Chunk) {
        self.bytes = chunk.get_bytes();
        self.constants = chunk.get_constants();
        self.ip = 0;
        self.sp = 0;
    }

    pub fn push(&mut self, value: Tensor<'a>) -> Result<(), &'static str> {
        if self.sp == STACK_SIZE {
            return Err("VM Runtime Error: Stack Overflow");
        }

        self.stack[self.sp].write(value);
        self.sp += 1;
        Ok(())
    }

    pub fn pop(&mut self) -> Result<Tensor<'a>, &'static str> {
        if self.sp == 0 {
            return Err("VM Runtime Error: Stack Underflow");
        }

        self.sp -= 1;
        Ok(unsafe { self.stack[self.sp].assume_init() })
    }

    fn read_byte(&mut self) -> u8 {
        let byte = self.bytes[self.ip];
        self.ip += 1;
        byte
    }

    pub fn run(&mut self) -> Result<Tensor<'a>, &'static str> {
        loop {
            debug_assert!(
                self.ip < self.bytes.len(),
                "ip out of bounds, missing Return?"
            );
            // if self.ip >= self.bytes.len() {
            //     return Err("VM Runtime Error: Unexpected end of bytecode");
            // }

            match Opcode::from(self.read_byte()) {
                Opcode::Return => return self.pop(),
                Opcode::Constant => {
                    let i = self.read_byte() as usize; // next byte should tell us which constant to load
                    let val = self.constants[i];
                    self.push(Tensor::Scalar(val))?;
                }
                Opcode::Negate => {
                    let val = self.pop()?;
                    self.push(-val)?;
                }
                Opcode::Add => {
                    let b = self.pop()?;
                    let a = self.pop()?;
                    self.push(a + b)?;
                }
                Opcode::Subtract => {
                    let b = self.pop()?;
                    let a = self.pop()?;
                    self.push(a - b)?;
                }
                Opcode::Divide => {
                    let b = self.pop()?;
                    let a = self.pop()?;
                    self.push(a / b)?;
                }
                Opcode::Multiply => {
                    let b = self.pop()?;
                    let a = self.pop()?;
                    self.push(a * b)?;
                }
            }
        }
    }
}
