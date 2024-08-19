use std::collections::HashMap;

use super::{factor::Factor, term::Term};
use super::super::reform::poly::Poly;

pub struct Expr<T>
    where T: Factor 
{
    terms: Vec<Term<T>>,
    pow: i32,
}

impl<T> Expr<T>
    where T: Factor
{
    pub fn new() -> Expr<T> {
        Expr {
            terms: Vec::new(),
            pow : 1,
        }
    }

    pub fn add_term(&mut self, term: Term<T>) {
        self.terms.push(term);
    }

    pub fn set_pow(&mut self, pow: i32) {
        self.pow = pow;
    }
    
}

impl<T> Factor for Expr<T> 
    where T: Factor
{
    fn to_polynomial(& self) -> Poly {
        let map = HashMap::new();
        let mut poly = Poly::new(map);
        for term in self.terms.iter() {
            poly = poly.add_poly(&mut term.to_polynomial());
        }
        poly.pow_poly(self.pow)
    }
}

