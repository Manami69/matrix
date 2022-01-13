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
        write!(f, "{}+{}i", &self.real, &self.imaginary)
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
impl Sub<Complexf64> for Complexf64  {
    type Output = Complexf64;
    fn sub(self, _rhs: Complexf64) -> Complexf64 {
        Complexf64::new(self.real - _rhs.real, self.imaginary - _rhs.imaginary)
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

// https://wims.univ-cotedazur.fr/wims/wims.cgi?session=3P6507DCBB.7&+lang=fr&+module=H6%2Falgebra%2Fdocintrocomplexf64.fr&+cmd=reply&+job=read&+doc=1&+block=quotient
impl Div<Complexf64> for Complexf64 where  {
    type Output = Complexf64;
    fn div(self, _rhs: Complexf64) -> Complexf64 {
        if _rhs.imaginary == f64::zero() && _rhs.real == f64::zero()
        {
            panic!("Can't divide by null number")
        }
        let a: f64 = self.real;
        let b: f64 = self.imaginary;
        let c: f64 = _rhs.real;
        let d: f64 = _rhs.imaginary;
        Complexf64::new(((a * c) + (b * d)) / ((c * c) + (d * d)), ((b * c) - (a * d)) / ((c * c) + (d * d)))
    }
}
