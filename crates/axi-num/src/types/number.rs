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

impl Add<Self> for Number {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            real: self.real + rhs.real,
            imag: self.imag + rhs.imag,
        }
    }
}

impl Sub<Self> for Number {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            real: self.real - rhs.real,
            imag: self.imag - rhs.imag,
        }
    }
}

impl Mul<Self> for Number {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            real: self.real * rhs.real - self.imag * rhs.imag,
            imag: self.real * rhs.imag + self.imag * rhs.real,
        }
    }
}

impl Div<Self> for Number {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        let denominator = rhs.real * rhs.real + rhs.imag * rhs.imag;
        Self {
            real: (self.real * rhs.real + self.imag * rhs.imag) / denominator,
            imag: (self.imag * rhs.real + self.real * rhs.imag) / denominator,
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
