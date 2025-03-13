use std::fmt;
use crate::object::Value;

// TODO Retirer cette enum directement utilise value
#[derive(Debug, Clone)]
pub enum Constant {
    Null,
    Boolean(bool),
    Number(f64),
    String(String)
}

impl fmt::Display for Constant {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Constant::Null               => { Ok(()) }
            Constant::Boolean(b)  => { write!(f, "Boolean: {}", b) }
            Constant::Number(n)    => { write!(f, "Number: {}", n) }
            Constant::String(s) => { write!(f, "String: {}", s) }
        }
    }

}


impl Constant {

    pub fn as_value <'guard>(&self) -> Value<'guard> {
        match *self {
            Constant::Null => { Value::Nil },
            Constant::Boolean(b) => { Value::Boolean(b) },
            Constant::Number(n) => { Value::Number(n) },
            Constant::String(_) => panic!("pas encore implemente")
        }
    }  

}