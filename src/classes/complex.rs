use num::Zero;
use std::fmt;
use std::ops::{Add, Div, Mul, Sub};

#[derive(PartialEq, PartialOrd, Clone, Copy, Debug)]
pub struct Complexf32 {
    pub real: f32,
    pub imaginary: f32,
}

impl Complexf32 {
    pub fn new(a: f32, b: f32) -> Self {
        Self {
            real: a,
            imaginary: b,
        }
    }
}

impl fmt::Display for Complexf32 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_zero() {
            write!(f, "0")
        } else if self.real.is_zero() {
            write!(f, "{}i", &self.imaginary)
        } else if self.imaginary.is_zero() {
            write!(f, "{}", &self.real)
        } else {
            write!(f, "{}{:+}i", &self.real, &self.imaginary)
        }
    }
}

impl From<f32> for Complexf32 {
    fn from(d: f32) -> Self {
        Self {
            real: d,
            imaginary: f32::zero(),
        }
    }
}

impl Zero for Complexf32 {
    fn zero() -> Complexf32 {
        Complexf32::new(f32::zero(), f32::zero())
    }

    fn is_zero(&self) -> bool {
        self.real.is_zero() && self.imaginary.is_zero()
    }
}

impl Add<Complexf32> for Complexf32 {
    type Output = Complexf32;
    fn add(self, _rhs: Complexf32) -> Complexf32 {
        Complexf32::new(self.real + _rhs.real, self.imaginary + _rhs.imaginary)
    }
}

impl<T> Add<T> for Complexf32
where
    T: Into<f32>,
{
    type Output = Complexf32;
    fn add(self, _rhs: T) -> Complexf32 {
        Complexf32::new(self.real + _rhs.into(), self.imaginary)
    }
}
impl Sub<Complexf32> for Complexf32 {
    type Output = Complexf32;
    fn sub(self, _rhs: Complexf32) -> Complexf32 {
        Complexf32::new(self.real - _rhs.real, self.imaginary - _rhs.imaginary)
    }
}

impl<T> Sub<T> for Complexf32
where
    T: Into<f32>,
{
    type Output = Complexf32;
    fn sub(self, _rhs: T) -> Complexf32 {
        Complexf32::new(self.real - _rhs.into(), self.imaginary)
    }
}

// https://wims.univ-cotedazur.fr/wims/wims.cgi?session=3P6507DCBB.1&+lang=fr&+module=H6%2Falgebra%2Fdocintrocomplexf32.fr&+cmd=reply&+job=read&+doc=1&+block=produit
impl Mul<Complexf32> for Complexf32 {
    type Output = Complexf32;
    fn mul(self, _rhs: Complexf32) -> Complexf32 {
        let a: f32 = self.real;
        let b: f32 = self.imaginary;
        let c: f32 = _rhs.real;
        let d: f32 = _rhs.imaginary;
        Complexf32::new((a * c) - (b * d), (a * d) + (b * c))
    }
}

impl<T> Mul<T> for Complexf32
where
    T: Into<f32>,
{
    type Output = Complexf32;
    fn mul(self, _rhs: T) -> Complexf32 {
        let num: f32 = _rhs.into();
        Complexf32::new(self.real * num, self.imaginary * num)
    }
}

// https://wims.univ-cotedazur.fr/wims/wims.cgi?session=3P6507DCBB.7&+lang=fr&+module=H6%2Falgebra%2Fdocintrocomplexf32.fr&+cmd=reply&+job=read&+doc=1&+block=quotient
impl Div<Complexf32> for Complexf32 {
    type Output = Complexf32;
    fn div(self, _rhs: Complexf32) -> Complexf32 {
        if _rhs.is_zero() {
            panic!("Can't divide by zero")
        }
        let a: f32 = self.real;
        let b: f32 = self.imaginary;
        let c: f32 = _rhs.real;
        let d: f32 = _rhs.imaginary;
        Complexf32::new(
            ((a * c) + (b * d)) / ((c * c) + (d * d)),
            ((b * c) - (a * d)) / ((c * c) + (d * d)),
        )
    }
}

impl<T> Div<T> for Complexf32
where
    T: Into<f32>,
{
    type Output = Complexf32;
    fn div(self, _rhs: T) -> Complexf32 {
        let num: f32 = _rhs.into();
        if num.is_zero() {
            panic!("Can't divide by zero")
        }

        Complexf32::new(self.real / num, self.imaginary / num)
    }
}
