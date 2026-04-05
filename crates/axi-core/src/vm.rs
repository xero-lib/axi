use axi_num::{Number, Tensor};

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
    /// Add a constant
    AddConstant = 7
    // /// Exponentiate a number by another
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
            7 => Opcode::AddConstant,
            // 8 => Opcode::Exponent,
            _ => panic!("Encountered invalid operation."),
        }
    }
}

// make configurable later?
const STACK_SIZE: usize = 256;

pub struct VM<'a> {
    chunk: &'a [u8],
    constants: &'a [Number],

    ip: usize,

    stack: [Tensor<'a>; STACK_SIZE],
    sp: usize,
}

impl<'a> VM<'a> {
    pub fn new(chunk: &'a [u8], constants: &'a [Number]) -> Self {
        VM {
            chunk,
            constants,
            ip: 0,
            stack: [Tensor::Scalar(Number::from(0.0)); 256],
            sp: 0,
        }
    }

    pub fn reset(&mut self, chunk: &'a [u8], constants: &'a [Number]) {
        self.chunk = chunk;
        self.constants = constants;
        self.ip = 0;
        self.sp = 0;
    }

    pub fn push(&mut self, value: Tensor<'a>) -> Result<(), &'static str> {
        if self.sp == STACK_SIZE - 1 {
            return Err("VM Runtime Error: Stack Overflow");
        }

        self.stack[self.sp] = value;
        self.sp += 1;
        Ok(())
    }

    pub fn pop(&mut self) -> Result<Tensor<'a>, &'static str> {
        if self.sp == 0 {
            return Err("VM Runtime Error: Stack Undeflow");
        }

        self.sp -= 1;
        Ok(self.stack[self.sp])
    }

    fn read_byte(&mut self) -> u8 {
        let byte = self.chunk[self.ip];
        self.ip += 1;
        byte
    }

    pub fn run(&mut self) -> Result<Tensor<'a>, &'static str> {
        loop {
            match Opcode::from(self.read_byte()) {
                Opcode::Return => return self.pop(),
                Opcode::Constant => {
                    let i = self.read_byte() as usize; // next byte should tell us which constant to load
                    let val = self.constants[i];
                    self.push(Tensor::Scalar(val))?;
                }
                Opcode::AddConstant => {
                    let index = self.read_byte() as usize;
                    let val = self.constants[index];
                    
                    let a = self.pop()?;
                    self.push(a + Tensor::Scalar(val))?;
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
