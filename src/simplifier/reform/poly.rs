use std::collections::HashMap;

pub struct Poly {
    units: HashMap<i32, i32>, // <pow, coe>
}

impl Poly {
    pub fn new (units: HashMap<i32, i32>) -> Poly{
        Poly {
            units,
        }
    }

    pub fn add_poly(&mut self, another: &mut Poly) -> Poly {
        for (pow, coe) in self.units.iter_mut() {
            if another.units.contains_key(pow) {
                let coe1 = another.units.get(pow).unwrap();
                let coe1 = *coe1 + *coe;
                let pow1 = *pow;
                another.units.insert(pow1, coe1);
            } else {
                another.units.insert(*pow, *coe);
            }
        }
        Poly::new(another.units.clone())
    }

    pub fn multi_poly(&mut self, another: &mut Poly) -> Poly{
        if self.units.is_empty() {
            return Poly::new(another.units.clone());
        } else {
            let mut new_map = HashMap::new();
            for (pow, coe) in self.units.iter_mut() {
                for (pow1, coe1) in another.units.iter_mut() {
                    let new_pow = *pow + *pow1;
                    let new_coe = *coe * *coe1;
                    if new_map.contains_key(&new_pow) {
                        let coe2 = new_map.get(&new_pow).unwrap();
                        let coe2 = *coe2 + new_coe;
                        new_map.insert(new_pow, coe2);
                    } else {
                        new_map.insert(new_pow, new_coe);
                    }
                }
            }
            Poly::new(new_map)
        }
    }

    pub fn pow_poly(&mut self, pow: i32) -> Poly {
        if pow == 0 {
            let mut new_map = HashMap::new();
            new_map.insert(0, 1);
            Poly::new(new_map)
        } else {
            let mut clone_poly = Poly::new(self.units.clone());
            for _ in 1..pow {
                clone_poly = clone_poly.multi_poly(&mut Poly::new(self.units.clone()));
            }
            clone_poly
        }
    }

    pub fn negate_poly(&mut self) {
        let mut new_map = HashMap::new();
        for (pow, coe) in self.units.iter_mut() {
            new_map.insert(*pow, -*coe);
        }
    }
}