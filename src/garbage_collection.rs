use simple_stack::Stack;
use std::{collections::HashMap, ops::Deref};

use crate::{interpreter::CallFrame, object::Value};

#[derive(thiserror::Error, Debug)]
pub enum HeapError {
    #[error("Symbol {sym} not found")]
    GlobalNotFound {
        sym : String
    },
    #[error("Error when allocation on the heap")]
    HeapAllocationError,
    #[error("Wrong type affectation")]
    AffectationError
}

pub struct GlobalEnvironment<'ge> {
    global_map: HashMap<String, Value<'ge>>
}

impl <'ge> GlobalEnvironment<'ge> {

    pub fn new() -> Self {
        GlobalEnvironment { global_map : HashMap::new() }
    }

    pub fn add_function(&mut self, val : Value<'ge>) {

    }

}
