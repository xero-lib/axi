use core::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Clone, Copy, PartialEq)]
pub struct Number {
    pub real: f64,
    pub imag: f64,
}

impl Neg for Number {
    type Output = Number;
    fn neg(self) -> Self::Output {
        Number {
            real: -self.real,
            imag: self.imag,
        }
    }
}

impl From<f64> for Number {
    fn from(value: f64) -> Self {
        Self {
            real: value,
            imag: 0.0,
        }
    }
}

impl From<(f64, f64)> for Number {
    fn from(value: (f64, f64)) -> Self {
        Self {
            real: value.0,
            imag: value.1,
        }
    }
}

impl Number {
    pub fn is_real(&self) -> bool {
        self.imag.abs() < 1e-12 // epsilon check
    }

    pub fn is_imag(&self) -> bool {
        !self.is_real()
    }
}

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
            (Tensor::Scalar(a), Tensor::Scalar(b)) => {
                let denominator = b.real * b.real + b.imag * b.imag;
                Tensor::Scalar(Number {
                    real: (a.real * b.real + a.imag * b.imag) / denominator,
                    imag: (a.imag * b.real + a.real * b.imag) / denominator,
                })
            }
            (_, _) => unimplemented!(),
        }
    }
}

impl<'a> Mul<Tensor<'_>> for Tensor<'a> {
    type Output = Tensor<'a>;
    fn mul(self, rhs: Tensor<'_>) -> Self::Output {
        match (self, rhs) {
            (Tensor::Scalar(a), Tensor::Scalar(b)) => {
                Tensor::Scalar(Number {
                    real: a.real * b.real - a.imag * b.imag,
                    imag: a.real * b.imag + a.imag * b.real
                })
            }
            (_, _) => unimplemented!(),
        }
    }
}
