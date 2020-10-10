use std::ops::Add;
use std::ops::Sub;
use std::ops::Div;
use std::ops::Mul;

#[derive(Debug, Copy, Clone)]
pub enum Const {
    Float(f64),
}

impl Add for Const {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        match self {
            Const::Float(l) => match other {
                Const::Float(r) => return Const::Float(l+r)
            },
        }
    }
}

impl Sub for Const {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        match self {
            Const::Float(l) => match other {
                Const::Float(r) => return Const::Float(l-r)
            }
        }
    }
}

impl Div for Const {
    type Output = Self;
    
    fn div(self, other: Self) -> Self {
        match self {
            Const::Float(l) => match other {
                Const::Float(r) => return Const::Float(l/r)
            }
        }
    }
}

impl Mul for Const {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        match self {
            Const::Float(l) => match other {
                Const::Float(r) => return Const::Float(l*r)
            }
        }
    }
}
