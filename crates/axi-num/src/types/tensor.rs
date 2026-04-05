use core::ops::{Add, Div, Mul, Neg, Sub};

use crate::Number;

#[derive(Clone, Copy)]
pub enum Tensor<'a> {
    Scalar(Number),
    Vector {
        data: &'a [Number],
    },
    Matrix {
        data: &'a [Number],
        rows: u32,
        cols: u32,
    },
}

impl<'a> Neg for Tensor<'a> {
    type Output = Tensor<'a>;
    fn neg(self) -> Self::Output {
        match self {
            Self::Scalar(n) => Self::Scalar(-n),
            _ => unimplemented!(),
        }
    }
}

impl<'a> Add<Tensor<'_>> for Tensor<'a> {
    type Output = Tensor<'a>;
    fn add(self, rhs: Tensor<'_>) -> Self::Output {
        match (self, rhs) {
            (Tensor::Scalar(a), Tensor::Scalar(b)) => Tensor::Scalar(Number {
                real: a.real + b.real,
                imag: a.imag + b.imag,
            }),
            (_, _) => unimplemented!(),
        }
    }
}

impl<'a> Sub<Tensor<'_>> for Tensor<'a> {
    type Output = Tensor<'a>;
    fn sub(self, rhs: Tensor<'_>) -> Self::Output {
        match (self, rhs) {
            (Tensor::Scalar(a), Tensor::Scalar(b)) => Tensor::Scalar(Number {
                real: a.real - b.real,
                imag: a.imag - b.imag,
            }),
            (_, _) => unimplemented!(),
        }
    }
}

impl<'a> Div<Tensor<'_>> for Tensor<'a> {
    type Output = Tensor<'a>;
    fn div(self, rhs: Tensor<'_>) -> Self::Output {
        match (self, rhs) {
            (Tensor::Scalar(a), Tensor::Scalar(b)) => Tensor::Scalar(a / b),
            (_, _) => unimplemented!(),
        }
    }
}

impl<'a> Mul<Tensor<'_>> for Tensor<'a> {
    type Output = Tensor<'a>;
    fn mul(self, rhs: Tensor<'_>) -> Self::Output {
        match (self, rhs) {
            (Tensor::Scalar(a), Tensor::Scalar(b)) => Tensor::Scalar(a * b),
            (_, _) => unimplemented!(),
        }
    }
}
