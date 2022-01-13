use num::{ Zero};
use std::fmt;
use std::ops::{Add, Div, Mul, Sub};

#[derive(PartialEq, PartialOrd, Clone, Copy, Debug)]
pub struct Complexf64 {
    real: f64,
    imaginary: f64,
}

impl Complexf64 {
    pub fn new(a: f64, b:f64) -> Self {
        Self { real: a, imaginary: b}
    }
}

impl fmt::Display for Complexf64 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_zero()
        {
            write!(f, "0")
        }
        else if self.real.is_zero()
        {
            write!(f, "{}i", &self.imaginary)
        }
        else if self.imaginary.is_zero()
        {
            write!(f, "{}", &self.real)
        }
        else {
            write!(f, "{}+{}i", &self.real, &self.imaginary)
        }
    }
}

impl From<f64> for Complexf64  {
    fn from(d: f64) -> Self {
        Self { real : d, imaginary: f64::zero()}
    }
}

impl Zero for Complexf64 {
    fn zero() -> Complexf64 {
        Complexf64::new(f64::zero(), f64::zero())
    }

    fn is_zero(&self) -> bool {
        self.real.is_zero() && self.imaginary.is_zero()
    }
}

impl Add<Complexf64> for Complexf64 {
    type Output = Complexf64;
    fn add(self, _rhs: Complexf64) -> Complexf64 {
        Complexf64::new(self.real + _rhs.real, self.imaginary + _rhs.imaginary)
    }
}

impl<T> Add<T> for Complexf64 where T: Into<f64> {
    type Output = Complexf64;
    fn add(self, _rhs: T) -> Complexf64 {
        Complexf64::new(self.real + _rhs.into(), self.imaginary)
    }
}
impl Sub<Complexf64> for Complexf64  {
    type Output = Complexf64;
    fn sub(self, _rhs: Complexf64) -> Complexf64 {
        Complexf64::new(self.real - _rhs.real, self.imaginary - _rhs.imaginary)
    }
}

impl<T> Sub<T> for Complexf64 where T: Into<f64> {
    type Output = Complexf64;
    fn sub(self, _rhs: T) -> Complexf64 {
        Complexf64::new(self.real - _rhs.into(), self.imaginary)
    }
}

// https://wims.univ-cotedazur.fr/wims/wims.cgi?session=3P6507DCBB.1&+lang=fr&+module=H6%2Falgebra%2Fdocintrocomplexf64.fr&+cmd=reply&+job=read&+doc=1&+block=produit
impl Mul<Complexf64> for Complexf64  {
    type Output = Complexf64;
    fn mul(self, _rhs: Complexf64) -> Complexf64 {
        let a: f64 = self.real;
        let b: f64 = self.imaginary;
        let c: f64 = _rhs.real;
        let d: f64 = _rhs.imaginary;
        Complexf64::new((a * c) - (b * d), (a * d) + (b * c))
    }
}

impl<T> Mul<T> for Complexf64 where T: Into<f64> {
    type Output = Complexf64;
    fn mul(self, _rhs: T) -> Complexf64 {
        let num: f64 = _rhs.into();
        Complexf64::new(self.real * num, self.imaginary * num)
    }
}

// https://wims.univ-cotedazur.fr/wims/wims.cgi?session=3P6507DCBB.7&+lang=fr&+module=H6%2Falgebra%2Fdocintrocomplexf64.fr&+cmd=reply&+job=read&+doc=1&+block=quotient
impl Div<Complexf64> for Complexf64 where  {
    type Output = Complexf64;
    fn div(self, _rhs: Complexf64) -> Complexf64 {
        if _rhs.is_zero()
        {
            panic!("Can't divide by zero")
        }
        let a: f64 = self.real;
        let b: f64 = self.imaginary;
        let c: f64 = _rhs.real;
        let d: f64 = _rhs.imaginary;
        Complexf64::new(((a * c) + (b * d)) / ((c * c) + (d * d)), ((b * c) - (a * d)) / ((c * c) + (d * d)))
    }
}

impl<T> Div<T> for Complexf64 where T: Into<f64> {
    type Output = Complexf64;
    fn div(self, _rhs: T) -> Complexf64 {
        let num: f64 = _rhs.into();
        if num.is_zero()
        {
            panic!("Can't divide by zero")
        }

        Complexf64::new(self.real / num, self.imaginary / num)
    }
}

