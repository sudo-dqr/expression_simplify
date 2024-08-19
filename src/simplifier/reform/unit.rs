pub struct Unit {
    coe: i32,
    pow: i32,
}

impl Unit {
    pub fn new(coe: i32, pow: i32) -> Unit {
        Unit {
            coe,
            pow,
        }
    }

    pub fn to_string(&self) -> String {
        return String::from(format!("{}x^{}", self.coe, self.pow));
    }
}