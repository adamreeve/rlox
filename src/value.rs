use std::fmt;

#[derive(Debug,Clone,Copy)]
pub enum Value {
    NilValue,
    BoolValue(bool),
    NumberValue(f64),
}

impl Value {
    pub fn nil() -> Value {
        Value::NilValue
    }

    pub fn bool(val: bool) -> Value {
        Value::BoolValue(val)
    }

    pub fn number(val: f64) -> Value {
        Value::NumberValue(val)
    }

    pub fn as_bool(&self) -> bool {
        match &self {
            &Value::BoolValue(val) => val.clone(),
            _ => panic!("Value is not a BoolValue"),
        }
    }

    pub fn as_number(&self) -> f64 {
        match &self {
            &Value::NumberValue(val) => val.clone(),
            _ => panic!("Value is not a NumberValue"),
        }
    }

    pub fn is_bool(&self) -> bool {
        match &self {
            &Value::BoolValue(_) => true,
            _ => false,
        }
    }

    pub fn is_number(&self) -> bool {
        match &self {
            &Value::NumberValue(_) => true,
            _ => false,
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            &Value::NilValue => write!(f, "Nil"),
            &Value::BoolValue(val) => write!(f, "Bool({})", val),
            &Value::NumberValue(val) => write!(f, "Number({})", val),
        }
    }
}
