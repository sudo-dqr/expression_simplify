use super::super::reform::poly::Poly;
use super::factor::Factor;
use std::collections::HashMap;

pub struct Pow {
    pow: i32,
}

impl Pow {
    pub fn new(pow: i32) -> Pow {
        Pow {
            pow,
        }
    }
}

impl Factor for Pow {
    fn to_polynomial(&self) -> Poly {
        let mut map = HashMap::new();
        map.insert(self.pow, 1);
        Poly::new(map)
    }
    
}