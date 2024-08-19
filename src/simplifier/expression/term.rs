use super::factor::Factor;
use super::super::reform::poly::Poly;
use std::collections::HashMap;

pub struct Term<T> 
    where T: Factor 
{
    factors: Vec<T>,
    sign: i32,    
}

impl<T> Term<T>
    where T: Factor
{
    pub fn new(sign: i32) -> Term<T> {
        Term {
            factors: Vec::new(),
            sign,
        }
    }

    pub fn add_factor(&mut self, factor: T) {
        self.factors.push(factor);
    }
}

impl <T> Factor for Term<T> 
    where T: Factor
{
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