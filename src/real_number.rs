#![allow(dead_code)]

use std::fmt::{Display, Formatter};
use std::ops::{Add, Sub};

#[derive(Debug, Copy, Clone)]
pub(super) struct Rational
{
    pub(super) num: i32,
    pub(super) den: i32
}

impl Add for Rational {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Rational {
            num: self.num * other.den + self.den * other.num,
            den: self.den * other.den
        }
    }
}

impl Rational {
    fn as_f64(&self) -> f64 {
        self.num as f64 / self.den as f64
    }
}

#[derive(Debug, Copy, Clone)]
pub(super) enum Real
{
    Rat(Rational),
    Irr(f64)
}

impl Real {
    fn as_f64(&self) -> f64 {
        match self {
            Real::Rat(x) => x.as_f64(),
            Real::Irr(x) => *x
        }
    }
}

impl Add for Real {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        match self {
            Real::Rat(x) => {
                match other {
                    Real::Rat(y) => Real::Rat(x + y),
                    Real::Irr(y) => Real::Irr(x.as_f64() + y)
                }
            },
            Real::Irr(x) => {
                match other {
                    Real::Rat(y) => Real::Irr(x + y.as_f64()),
                    Real::Irr(y) => Real::Irr(x + y)
                }
            }
        }
    }
}

impl Sub for Real {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        let inverse = match other {
            Real::Rat(mut x) => {
                x.num *= -1;
                Real::Rat(x)
            },
            Real::Irr(mut x) => {
                x *= -1.0;
                Real::Irr(x)
            }
        };

        self + inverse
    }
}

impl Display for Real {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Real::Rat(x) => write!(f, "[{}/{}(={})]", x.num, x.den, x.as_f64()),
            Real::Irr(x) => write!(f, "[{}]", x)
        }
    }
}
