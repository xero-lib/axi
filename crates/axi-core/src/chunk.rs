use core::ops::{Index, IndexMut, RangeTo};

use axi_num::Number;

pub struct Chunk {
    pub bytes: [u8; 512],
    pub len: usize,
    pub constants: [Number; 64],
    pub constants_len: usize,
}

impl Index<usize> for Chunk {
    type Output = u8;
    fn index(&self, index: usize) -> &Self::Output {
        &self.bytes[index]
    }
}

impl Index<RangeTo<usize>> for Chunk {
    type Output = [u8];
    fn index(&self, index: RangeTo<usize>) -> &Self::Output {
        &self.bytes[index]
    }
}

impl IndexMut<RangeTo<usize>> for Chunk {
    fn index_mut(&mut self, index: RangeTo<usize>) -> &mut Self::Output {
        &mut self.bytes[index]
    }
}

impl IndexMut<usize> for Chunk {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.bytes[index]
    }
}

impl Chunk {
    pub fn get_bytes(&self) -> &[u8] {
        &self.bytes[..self.len]
    }

    pub fn get_constants(&self) -> &[Number] {
        &self.constants[..self.constants_len]
    }

    pub fn emit_byte(&mut self, byte: u8) {
        self.bytes[self.len] = byte;
        self.len += 1;
    }

    pub fn add_constant(&mut self, val: Number) -> usize {
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
}
