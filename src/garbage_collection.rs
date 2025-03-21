use std::collections::HashMap;

use crate::{object::Value, runtime_library};

#[derive(thiserror::Error, Debug)]
pub enum EnvironmentError {
    #[error("Symbol {sym} not found")]
    GlobalNotFound {
        sym : String
    }
}

pub struct GlobalEnvironment<'ge> {
    global_map: HashMap<String, Value<'ge>>
}

impl <'ge> GlobalEnvironment<'ge> {

    pub fn new() -> Self {
        let mut res = GlobalEnvironment { 
            global_map : HashMap::new() 
        };

        res.global_map.insert(String::from("print"), Value::RuntimeFunction(runtime_library::print_lua));

        res
    }

    pub fn insert_global(&mut self, key : &String, val : Value<'ge>) {
        self.global_map.insert(String::from(key), val);
    }

    pub fn get_global(&self, key : &String) -> Result<Value<'ge>, EnvironmentError> {
        let val = self.global_map.get(key);
        
        match val {
            Some(v) => { Ok(*v) }
            None => { return Err(EnvironmentError::GlobalNotFound { sym: String::from(key) }) }
        }
    }

}
