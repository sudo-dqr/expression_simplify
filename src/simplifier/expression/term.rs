use super::factor::Factor;

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