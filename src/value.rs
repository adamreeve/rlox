use std::fmt;

#[derive(Debug,Clone,Copy)]
pub struct Value(f64);

impl Value {
    pub fn new(val: f64) -> Value {
        Value(val)
    }

    pub fn value(&self) -> f64 {
        self.0
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let &Value(val) = self;
        write!(f, "{}", val)
    }
}
