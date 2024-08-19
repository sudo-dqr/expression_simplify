use std::collections::HashMap;

use super::{factor::Factor, term::Term};
use super::super::reform::poly::Poly;

pub struct Expr {
    terms: Vec<Term>,
    pow: i32,
}

impl Expr {
    pub fn new() -> Expr {
        Expr {
            terms: Vec::new(),
            pow : 1,
        }
    }

    pub fn add_term(&mut self, term: Term) {
        self.terms.push(term);
    }

    pub fn set_pow(&mut self, pow: i32) {
        self.pow = pow;
    }
    
}

impl Factor for Expr {
    fn to_polynomial(& self) -> Poly {
        let map = HashMap::new();
        let mut poly = Poly::new(map);
        for term in self.terms.iter() {
            poly = poly.add_poly(&mut term.to_polynomial());
        }
        poly.pow_poly(self.pow)
    }
}

