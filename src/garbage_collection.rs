use simple_stack::Stack;
use std::{collections::HashMap, ops::Deref};

use crate::{alloc::{HasHeader, ObjectCell, ObjectHeader, ScopedPtr}, interpreter::CallFrame, object::Value};

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

pub trait Mutate{
    fn mutate(self, obj_coll : &ObjectCollector) -> Result<(), HeapError>;
    fn build_collectable(self) -> Result<ObjectCollector, HeapError>;
}

impl Mutate for f64 {
    fn mutate(self, obj_coll : &ObjectCollector) -> Result<(), HeapError> {
        match obj_coll {
            ObjectCollector::Number(float_cell) => { 
                float_cell.set(self).or_else(
                    |_|{
                        return Err(HeapError::HeapAllocationError)
                    })?;
            }
            _ => { return Err(HeapError::AffectationError) }
        }
        Ok(())
    }

    fn build_collectable(self) -> Result<ObjectCollector, HeapError> {
        let obj_cell = ObjectCell::build(self).or_else(
            |err|{
                return Err(HeapError::HeapAllocationError)
            }
        )?;
        Ok(ObjectCollector::Number(obj_cell))
    }
}

impl Mutate for bool {
    fn mutate(self, obj_coll : &ObjectCollector) -> Result<(), HeapError> {
        match obj_coll {
            ObjectCollector::Boolean(bool_cell) => { 
                bool_cell.set(self).or_else(
                    |_|{
                        return Err(HeapError::HeapAllocationError)
                    })?;
            }
            _ => { return Err(HeapError::AffectationError) }
        }
        Ok(())
    }

    fn build_collectable(self) -> Result<ObjectCollector, HeapError> {
        let obj_cell = ObjectCell::build(self).or_else(
            |err|{
                return Err(HeapError::HeapAllocationError)
            }
        )?;
        Ok(ObjectCollector::Boolean(obj_cell))
    }
}

pub enum ObjectCollector {
    Number(ObjectCell<f64>),
    Boolean(ObjectCell<bool>)
}

impl ObjectCollector {

    fn build<T>(value : T) -> Result<Self, HeapError> 
    where T : HasHeader + Copy + Mutate{
        value.build_collectable()
    }

    pub fn as_value<'gc>(&mut self, collector : &'gc Collector ) -> Value<'gc> {
        match self {
            ObjectCollector::Number(c) => { Value::ObjectNumber(ScopedPtr::new(collector, c)) }
            ObjectCollector::Boolean(c) => { Value::ObjectBoolean(ScopedPtr::new(collector, c)) }
        }
    }

    pub fn get_header(&self) -> ObjectHeader {
        match self {
            ObjectCollector::Number(f) =>  { f.get_header() },
            ObjectCollector::Boolean(b) => { b.get_header() }
        }
    }

    pub fn mutate<T>(&mut self, value : T) -> Result<(), HeapError> 
        where T : HasHeader + Copy + Mutate
    {
        value.mutate(self)
    }

}

pub struct Collector {

    roots : Stack<ObjectCollector>,
    global_table : HashMap<String, ObjectCollector>

}

impl Collector {
    
    pub fn new() -> Self {
        
        Self {
            roots : Stack::new(),
            global_table : HashMap::new()
        }

    }

    pub fn extract_global<'gc, 'frm>(&'gc self, key : String) -> Result<Value<'gc>, HeapError> 
    where 'gc : 'frm {
        let res = self.global_table.get(&key);
        let res= match res {
            None => { return Err(HeapError::GlobalNotFound { sym: String::from(key) }) }
            Some(ObjectCollector::Number(c)) => { Value::ObjectNumber(ScopedPtr::new(self, c)) }
            Some(ObjectCollector::Boolean(c)) => { Value::ObjectBoolean(ScopedPtr::new(self, c)) }
        };

        Ok(res)
    }

    pub fn add_global<'gc, T : HasHeader + Copy + Mutate>(&'gc mut self, key : &String, val : T) -> Result<(), HeapError> {
        
        let former_value = self.global_table.get(key);

        match former_value {
            None => {
                self.global_table.insert(String::from(key), ObjectCollector::build(val)?);
            }
            Some(obj_coll) => {
                let obj_header = obj_coll.get_header();
                let header = val.new_header();
                if header.obj_type == obj_header.obj_type {
                    val.mutate(obj_coll)?;
                } else {
                    self.global_table.insert(String::from(key), ObjectCollector::build(val)?);
                }
            }
        }

        Ok(())
    }

}