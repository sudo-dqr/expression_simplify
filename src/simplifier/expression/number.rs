use super::factor::Factor;
use super::super::reform::poly::Poly;

pub struct Number {
    i: i32,
}

impl Number {
    pub fn new(i: i32) -> Number {
        Number {
            i,
        }
    }
}

impl Factor for Number {
    fn to_polynomial(&self) -> Poly {
        
    }
}