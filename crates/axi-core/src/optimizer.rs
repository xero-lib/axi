use crate::{Chunk, vm::Opcode};
use axi_num::Number;

/// Optimizes Chunk bytecode in-place 
pub struct Optimizer<'a> {
    chunk: &'a mut Chunk,
}

impl<'a> Optimizer<'a> {
    pub fn new(chunk: &'a mut Chunk) -> Self {
        Self { chunk }
    }

    pub fn optimize(&mut self) {
        loop {
            let prev_len = self.chunk.len;
            self.fold_constants();
            if self.chunk.len == prev_len {
                break;
            }
        }
    }

    fn fold_constants(&mut self) {
        let mut i = 0;
        let mut out = [0u8; 512];
        let mut out_len = 0;

        while i < self.chunk.len {
            let op = self.chunk[i];

            if op == Opcode::Constant as u8
                && i + 4 < self.chunk.len
                && self.chunk[i + 2] == Opcode::Constant as u8
            {
                let binary_op = self.chunk[i + 4];
                let a = self.chunk.constants[self.chunk[i + 1] as usize];
                let b = self.chunk.constants[self.chunk[i + 3] as usize];

                let result = if binary_op == Opcode::Add as u8 {
                    Some(a + b)
                } else if binary_op == Opcode::Subtract as u8 {
                    Some(a - b)
                } else if binary_op == Opcode::Multiply as u8 {
                    Some(a * b)
                } else if binary_op == Opcode::Divide as u8 {
                    if b == Number::from(0.0) {
                        None
                    } else {
                        Some(a / b)
                    }
                } else {
                    None
                };

                if let Some(val) = result {
                    let index = self.chunk.add_constant(val);
                    out[out_len] = Opcode::Constant as u8;
                    out[out_len + 1] = index as u8;
                    out_len += 2;
                    i += 5;
                    continue;
                }
            }

            out[out_len] = self.chunk[i];
            out_len += 1;
            i += 1;
        }

        self.chunk.bytes[..out_len].copy_from_slice(&out[..out_len]);
        self.chunk.len = out_len;
    }
}
