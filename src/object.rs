use std::cell::Cell;
use crate::structure::function::Function;

// I used a similar representation as the one described here: https://


// Pointeur pointant soit sur un objet dans le tas de la vm
// Soit pointant vers la table des constantes
// Representation safe d'un pointeur
#[derive(Copy, Clone)]
pub struct ScopedPtr<'guard, T> {
    ptr : &'guard T 
}

// representation unsafe d'un pointeur
pub struct RawPtr<T> {
    ptr : *const T
}

enum TypeLua{
    Number,
    Boolean,
    String,
    Nil,
    Table,
    Function
}

#[derive(Copy, Clone)]
pub enum Value<'guard> {
    Number(f64),
    Boolean(bool),
    Nil,
    Object(ScopedPtr<'guard, ()>)
}

// Enum utilisé comme interface pour utiliser les TaggedPtr 
pub enum FatPtr {
    Number(f64),
    Boolean(bool),
    Nil
}

// representation unsafe d'un tagged ptr
pub union TaggedPtr {
    tag: usize,
    number: u64
}

const TAG_SIZE: usize = 2;
const TAG_MASK: usize = 0x3;
const PTR_MASK: usize = !0x3;
const TAG_BOOLEAN: usize = 0x2;
const TAG_PTR: usize = 0x0;
const TAG_NUMBER: usize = 0x1;

impl From<bool> for TaggedPtr {
    fn from(b : bool) -> Self {
        if b {
            TaggedPtr { number : (1 << TAG_SIZE) + (TAG_BOOLEAN as u64) }
        } else {
            TaggedPtr { number : TAG_BOOLEAN as u64 }
        } 
    }
}

impl From<f64> for TaggedPtr {
    fn from(lua_number: f64) -> Self {
        TaggedPtr { number: (lua_number.to_bits() << TAG_SIZE) + (TAG_NUMBER as u64) }
    }
}

impl From<FatPtr> for TaggedPtr {
    fn from(raw: FatPtr) -> Self {
        match raw {
            FatPtr::Nil => { TaggedPtr::nil() }
            FatPtr::Number(f64) => { TaggedPtr::from(f64) }
            FatPtr::Boolean(b) => { TaggedPtr::from(b) } 
        }
    }
}

impl TaggedPtr {

    pub fn nil() -> Self {
        TaggedPtr { tag : 0 } 
    }

    fn get_ptr(&self) -> usize {
        unsafe {
            self.tag & PTR_MASK
        }
    }

    pub fn get_number(&self) -> f64 {
        unsafe {
            f64::from_bits(self.number >> TAG_SIZE)
        }
    }

    pub fn get_boolean(&self) -> bool {
        unsafe {
            (self.number >> TAG_SIZE) == 1
        }
    }

    pub fn get_type(self) -> FatPtr {
        unsafe {
            match self.tag {
                t if t == 0 => { FatPtr::Nil }
                t if (t & TAG_MASK) == TAG_PTR => { panic!("not implemented yet") }
                t if (t & TAG_MASK) == TAG_NUMBER => { FatPtr::Number(self.get_number()) }
                t if (t & TAG_MASK) == TAG_BOOLEAN => { FatPtr::Boolean(self.get_boolean()) }
                _ => panic!("impossible")
            }
        }
    }

}

impl From<TaggedPtr> for FatPtr {

    fn from(tag: TaggedPtr) -> Self {
        tag.get_type()
    }

}

impl FatPtr {

    fn into_values<'guard>(&self) -> Value<'guard> {
        match *self {
            Self::Nil => { Value::Nil }
            Self::Number(f) => { Value::Number(f) }
            Self::Boolean(b) => { Value::Boolean(b) }
        }
    }

}

impl  <'guard> From<FatPtr> for Value<'guard> {

    fn from(ptr : FatPtr) -> Value<'guard> {
        ptr.into_values()
    }

}

impl <'guard> Value<'guard> {

    pub fn is_nil(&self) -> bool {
        match *self {
            Self::Nil => { true }
            _         => { false }
        }
    } 

    pub fn get_number(self) -> f64 {
        match self {
            Self::Number(res) => { res }
            _ => { panic!("Should not have been called") }
        }
    }

}