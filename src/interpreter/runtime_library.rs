use crate::interpreter::{call_frame::CallFrame, object::Value};

pub type RuntimeFunction<'frm> = fn (CallFrame<'frm>, &mut Vec<Value<'frm>>);

pub fn print_lua<'frm>(frame : CallFrame<'frm>, _ : &mut Vec<Value<'frm>> ) {
    for i in 0..frame.len() {
        let val = frame.load(i);
        print!("{}\t", val);
    }
    println!();
}