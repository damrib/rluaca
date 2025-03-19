use std::thread::Scope;

use crate::alloc::{HasHeader, ScopedPtr, TypeLua};

// I used a similar representation as the one described here: https://

#[derive(Copy, Clone)]
pub enum Value<'gc> {
    Number(f64),
    Boolean(bool),
    Nil,
    ObjectNumber(ScopedPtr<'gc, f64>),
    ObjectBoolean(ScopedPtr<'gc, bool>),
}

impl <'gc> Value<'gc> {

    pub fn is_nil(&self) -> bool {
        match *self {
            Self::Nil => { true }
            _         => { false }
        }
    }

    pub fn get_number(&self) -> Option<f64> {
        match self {
            Self::Number(res) => { Some(*res) }
            Self::ObjectNumber(ptr) => { Some(ptr.get()) }
            _ => { None }
        }
    }

    pub fn get_boolean(&self) -> Option<bool> {
        match self {
            Self::Boolean(b) => { Some(*b) }
            Self::ObjectBoolean(ptr) => { Some(ptr.get()) }
            _ => { None }
        }
    }

    pub fn get_type(&self) -> TypeLua {

        match self {
            Self::Boolean(_) => { TypeLua::Boolean }
            Self::ObjectBoolean(_) => { TypeLua::Boolean }
            Self::Nil => { TypeLua::Nil }
            Self::Number(_) => { TypeLua::Number }
            Self::ObjectNumber(_) => { TypeLua::Number }
        }

    }

}