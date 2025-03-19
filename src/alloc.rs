use std::{vec, alloc::{self, Layout}, cell::Cell};

use crate::{garbage_collection::Collector, object::Value};

#[derive(PartialEq)]
pub enum TypeLua{
    Number,
    Boolean,
    String,
    Nil,
    Table,
    Function
}

pub struct ObjectHeader {
    pub obj_type : TypeLua,
    size : usize
}

pub trait HasHeader {
    fn sizeof(&self) -> usize;
    fn new_header(&self) -> ObjectHeader; 
}


#[derive(Copy, Clone)]
// TODO : if possible change visibility of this struct
struct RawPtr<T> where T : HasHeader + Copy {
    ptr : *mut T
}

#[derive(Clone)]
pub struct ObjectCell<T : HasHeader + Copy> {
    ptr : Cell<RawPtr<T>>
}

#[derive(Copy, Clone)]
pub struct ScopedPtr<'guard, T : HasHeader + Copy> {
    obj_ref : &'guard T
}

impl HasHeader for f64 {
    fn sizeof(&self) -> usize {
        8
    }

    fn new_header(&self) -> ObjectHeader {
        ObjectHeader {
            obj_type : TypeLua::Number,
            size : self.sizeof()
        }
    }
}

impl HasHeader for bool {
    fn sizeof(&self) -> usize {
        1
    }

    fn new_header(&self) -> ObjectHeader {
        ObjectHeader {
            obj_type : TypeLua::Boolean,
            size : self.sizeof()
        }
    }
}

impl <T> RawPtr<T> where T : HasHeader + Copy {

    pub fn build(val : T) -> Result<RawPtr<T>, alloc::LayoutError> {
        let size = val.sizeof();
        assert!(size > 0 && size.is_power_of_two());
        
        let new_layout = alloc::Layout::from_size_align(size, size)?;

        let new_ptr = unsafe { alloc::alloc(new_layout) };

        if new_ptr.is_null() {
            alloc::handle_alloc_error(new_layout);
        }

        let raw = RawPtr::<T> {
            ptr : new_ptr.cast()
        };

        unsafe { raw.ptr.write(val); }

        Ok(raw)
    }

    pub fn dealloc(&mut self, size: usize) -> Result<(), alloc::LayoutError> {
        assert!(size > 0 && size.is_power_of_two());

        let layout = alloc::Layout::from_size_align(size, size)?;
        unsafe { alloc::dealloc(self.ptr as *mut u8, layout); }

        Ok(())
    }

    pub fn get_header(&self) -> ObjectHeader {
        unsafe { (*self.ptr).new_header() }
    }

    pub fn get_size(&self) -> usize {
        unsafe { (*self.ptr).sizeof() }
    }

}

impl <T> Drop for ObjectCell<T> where T : HasHeader + Copy {
    fn drop(&mut self) {
        println!("drop");
        let raw = self.ptr.get_mut();

        let s = raw.get_size();

        raw.dealloc(s).unwrap()
    }
}

impl <T> ObjectCell<T> where T : HasHeader + Copy {
    pub fn build(value: T) -> Result<Self, alloc::LayoutError> {
        let raw : RawPtr<T> = RawPtr::build(value)?;

        let ptr = Self {
            ptr : Cell::new(raw)
        };

        Ok(ptr)
    }

    pub fn get_ref<'gc>(&self, _ : &'gc Collector) -> &'gc T {
       unsafe { &*self.ptr.get().ptr }
    }

    pub fn get_header(&self) -> ObjectHeader {
        self.ptr.get().get_header()
    }

    pub fn set(&self, value : T) -> Result<(), alloc::LayoutError> {

        let mut obj = self.ptr.replace(RawPtr::build(value)?);

        obj.dealloc(obj.get_size())?;

        Ok (())
    }

}

impl <'guard, T> ScopedPtr<'guard, T> where T : HasHeader + Copy {

    pub fn new(collector : &'guard Collector, obj_cell : &ObjectCell<T>) -> Self {
        Self {
            obj_ref : obj_cell.get_ref(collector)
        }
    } 

}

impl <'guard, T> ScopedPtr<'guard, T> where T : HasHeader + Copy {

    pub fn get(&self) -> T {
        *self.obj_ref
    }

}




#[test]
fn build_test() -> Result<(), alloc::LayoutError> {
    let mask = 7;
    let mut raw = RawPtr::<f64>::build(0.0).unwrap();
        assert!((raw.ptr as usize & mask) ^ mask == mask);

        if let Some(val) = unsafe { raw.ptr.as_ref() } {
            println!("{}", val);
            assert!(0.0 == *val);
        }

        let val = unsafe { raw.ptr.as_ref() };
        match val {
            Some(v) => { raw.dealloc(v.sizeof()).unwrap() }
            None => { panic!("error") }
        }

    let mut br = RawPtr::<bool>::build(true).unwrap();
    assert!((br.ptr as usize & mask) ^ mask == mask);

    if let Some(val) = unsafe { br.ptr.as_ref() } {
        println!("{}", val);
        assert!(true == *val);
    }

    let val = unsafe { br.ptr.as_ref() };
    match val {
        Some(v) => { br.dealloc(v.sizeof()).unwrap() }
        None => { panic!("error") }
    }

    //let mut v = Vec::new();
    let mut obj_cell = ObjectCell::build(0.0).unwrap();
    {
        let collector = Collector::new();
        {            
            let v = obj_cell.get_ref(&collector);

            let _ = &mut obj_cell;

            println!("{}", v);
        }
    }
    println!("Dropped?");

    let mut obj_cell = ObjectCell::build(0.0)?;
    obj_cell.set(1.0)?;

    Ok(())
}

