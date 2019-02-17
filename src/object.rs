use std::fmt;

#[derive(Debug)]
pub enum LoxObject {
    String(String)
}

impl fmt::Display for LoxObject {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            &LoxObject::String(s) => write!(f, "\"{}\"", s),
        }
    }
}

pub fn objects_equal(left: &LoxObject, right: &LoxObject) -> bool {
    match (left, right) {
        (LoxObject::String(left), LoxObject::String(right)) => left == right,
        // (_, _) => false,
    }
}
