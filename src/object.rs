use core::fmt;

use crate::{runtime_library::RuntimeFunction, structure::function::Function};

#[derive(PartialEq, Debug)]
pub enum TypeLua{
    Number,
    Boolean,
    String,
    Nil,
    Table,
    Function
}

#[derive(Clone, Copy, Debug)]
pub enum Value<'gc> {
    Number(f64),
    Boolean(bool),
    LuaFunction(&'gc Function),
    LuaString(&'gc String),
    RuntimeFunction(RuntimeFunction<'gc>),
    Nil
}

impl <'gc> Value<'gc> {

    pub fn is_nil(&self) -> bool {
        match *self {
            Self::Nil => { true }
            _         => { false }
        }
    }

    pub fn get_number(&self) -> Option<f64> {
        // TODO handle string arithmetic op
        match self {
            Self::Number(res) => { Some(*res) }
            _ => { None }
        }
    }

    pub fn get_boolean(&self) -> Option<bool> {
        match self {
            Self::Boolean(b) => { Some(*b) }
            _ => { None }
        }
    }

    pub fn get_function(&self) -> Option<&'gc Function> {
        match self {
            Self::LuaFunction(f) => { Some(*f) }
            _ => { None }
        }
    }

    pub fn get_string(&self) -> Option<&'gc String> {
        match self {
            Self::LuaString(s) => { Some(*s) }
            _ => { None }
        }
    }

    pub fn get_type(&self) -> TypeLua {

        match self {
            Self::Boolean(_) => { TypeLua::Boolean }
            Self::Nil => { TypeLua::Nil }
            Self::Number(_) => { TypeLua::Number }
            Self::LuaFunction(_) => { TypeLua::Function }
            Self::LuaString(_) => { TypeLua::String }
            Self::RuntimeFunction(_) => { TypeLua::Function }
        }

    }

}

impl <'gc> fmt::Display for Value<'gc>{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        
        match *self {
            Self::Boolean(b) => { write!(f, "{}", b) }
            Self::Number(n) => { write!(f, "{}", n) }
            Self::Nil => { write!(f, "nil") }
            Self::LuaString(s) => { write!(f, "{}", s.as_str()) } 
            // We display the location of functions in memory
            Self::LuaFunction(adr) => { write!(f, "function: {}", adr) }
            Self::RuntimeFunction(adr) => { write!(f, "function: {}", adr as usize) }
        }
    }
}