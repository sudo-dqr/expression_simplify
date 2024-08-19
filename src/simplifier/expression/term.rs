use super::factor::Factor;
use super::super::reform::poly::Poly;
use std::collections::HashMap;

pub struct Term {
    factors: Vec<Box<dyn Factor>>,
    sign: i32,    
}

impl Term {
    pub fn new(sign: i32) -> Term {
        Term {
            factors: Vec::new(),
            sign,
        }
    }

    pub fn add_factor(&mut self, factor: Box<dyn Factor>) {
        self.factors.push(factor);
    }
}

impl Factor for Term {
    fn to_polynomial(&self) -> Poly {
        let mut poly = Poly::new(HashMap::new());
        for factor in self.factors.iter() {
            poly = poly.multi_poly(&mut factor.to_polynomial());
        }
        if self.sign == -1 {
            poly.negate_poly();
        }
        poly
    }
    
}