use core::fmt;
use std::ops::{BitAnd, BitOr};

use crate::{interpreter::runtime_library::RuntimeFunction, structure::function::Function};

#[derive(PartialEq, Debug)]
pub enum TypeLua{
    Number,
    Boolean,
    String,
    Nil,
    Table,
    Function
}

#[derive(Clone, Copy, Debug, Default)]
pub enum Value<'gc> {
    Number(f64),
    Boolean(bool),
    LuaFunction(&'gc Function),
    LuaString(&'gc String),
    RuntimeFunction(RuntimeFunction<'gc>),
    #[default]
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
        match *self {
            Self::Boolean(b) => { Some(b) }
            _ => { None }
        }
    }

    pub fn to_boolean(&self) -> bool {
        match *self {
            Self::Boolean(b) => b,
            Self::Number(_) | Self::LuaString(_) | Self::LuaFunction(_) | Self::RuntimeFunction(_) => true,
            Self::Nil => false
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

impl <'gc> PartialEq<Value<'gc>> for Value<'gc> {
    fn eq(&self, other: &Value<'gc>) -> bool {
        match (*self, *other) {
            (Self::Number(l0), Self::Number(r0)) => l0 == r0,
            (Self::Boolean(l0), Self::Boolean(r0)) => l0 == r0,
            (Self::LuaFunction(l0), Self::LuaFunction(r0)) => l0 as *const _ == r0 as *const _,
            (Self::LuaString(l0), Self::LuaString(r0)) => l0 == r0,
            (Self::RuntimeFunction(l0), Self::RuntimeFunction(r0)) => l0 == r0,
            (Self::Nil, Self::Nil) => true,
            _ => false
        }
    }
}

impl <'gc> BitAnd<Value<'gc>> for Value<'gc> {
    type Output = Value<'gc>;

    fn bitand(self, rhs: Value<'gc>) -> Self::Output {
        match (self, rhs) {
            (Self::Nil, _) | (_, Self::Nil) => Self::Nil,
            (Self::Boolean(b1), Self::Boolean(b2)) => Self::Boolean(b1 && b2),
            (Self::Boolean(false), _) => Self::Boolean(false),
            (a, Self::Boolean(false)) => a,
            (Self::Number(n1), Self::Number(n2)) => {
                let bits1 = f64::to_bits(n1);
                let bits2 = f64::to_bits(n2);
                Self::Number(f64::from_bits(bits1 & bits2))
            } (_, b) => b
        }
    }
}

impl <'gc> BitOr<Value<'gc>> for Value<'gc> {
    type Output = Value<'gc>;

    fn bitor(self, rhs: Value<'gc>) -> Self::Output {
        match (self, rhs) {
            (Self::Nil, v) | (v, Self::Nil) => v,
            (Self::Boolean(b1), Self::Boolean(b2)) => Self::Boolean(b1 || b2),
            (Self::Number(n1), Self::Number(n2)) => {
                let bits1 = f64::to_bits(n1);
                let bits2 = f64::to_bits(n2);
                Self::Number(f64::from_bits(bits1 | bits2))
            } (a, _) => a
        }        
    }
}

impl <'gc> fmt::Display for Value<'gc>{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        
        match self {
            Self::Boolean(b) => { write!(f, "{}", *b) }
            Self::Number(n) => { write!(f, "{}", *n) }
            Self::Nil => { write!(f, "nil") }
            Self::LuaString(s) => { write!(f, "{}", (*s).as_str()) } 
            // We display the location of functions in memory
            Self::LuaFunction(adr) => { write!(f, "function: {:p}", *adr) }
            Self::RuntimeFunction(adr) => { write!(f, "function: {}", *adr as usize) }
        }
    }
}