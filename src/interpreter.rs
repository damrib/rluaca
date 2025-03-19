use simple_stack::Stack;

use crate::alloc::TypeLua;
use crate::garbage_collection::{Collector, HeapError};
use crate::structure::{function::Function, instruction::Instruction};
use crate::object::Value;
use std::error::Error;

#[derive(thiserror::Error, Debug)]
pub enum InterpreterError {
    #[error("Heap Error: ")]
    GlobalError {
        #[from]
        heap_error : HeapError
    }
}

// TODO: implement Runtime Error

// TODO Modularity
pub struct CallFrame<'guard> {
    frame : Vec<Value<'guard>>
}

impl <'frm> CallFrame<'frm> {

    pub fn with_capacity(n : u8) -> Self {
        CallFrame {
            frame : vec![Value::Nil; n as usize]
        }
    }

    pub fn store(&mut self, index: usize, v : Value<'frm>) {
        self.frame[index] = v;
    }

    pub fn load(&self, index: usize) -> Value<'frm> {
        self.frame[index]
    }

    pub fn move_register(&mut self, a : usize, b : usize) {
        let register_b = self.load(b);
        self.frame[a] = register_b;
    }

    pub fn store_global<'gc>(&'frm mut self, index : usize, heap: &'gc Collector, gbl_name : String) -> Result<(), InterpreterError> 
    where 'gc : 'frm {
        let val = heap.extract_global(gbl_name).or_else(
            |err| {
                return Err(InterpreterError::GlobalError { heap_error: err })
            }
        )?;

        self.frame[index] = val;

        Ok(())
    }

}

fn load_k(main: &Function, frame: &mut CallFrame<'_>, a : usize, b : usize) {
    let constant = main.const_list[b].as_value();
    let v = constant;
    frame.store(a, v);
}

fn load_bool(frame : &mut CallFrame<'_>, a : usize, b : usize, c : usize, pc: &mut usize) {
    frame.move_register(a, b);
    if ! frame.load(c).is_nil() {
        *pc += 1;
    }
}

fn load_nil(frame : &mut CallFrame<'_>, a : usize, b : usize) {
    
    for i in a..=b {
        frame.store(i, Value::Nil);
    }

}

fn move_operation(frame: &mut CallFrame<'_>, a : usize, b : usize) {
    frame.move_register(a, b);
}

fn get_rk<'frm>(func : &Function, frame : &'frm CallFrame<'_>, r : usize) -> Value<'frm> {
    if r > 256 {
        func.const_list[r].as_value()
    } else {
        frame.load(r)
    }
}

fn add(func : &Function, frame: &mut CallFrame<'_>, a : usize, b : usize, c : usize) {
    let rk_b = get_rk(func, frame, b);
    let rk_c = get_rk(func, frame, c);
    frame.store(a, Value::Number(rk_b.get_number().unwrap() + rk_c.get_number().unwrap()));
}

fn get_global<'gc, 'frm>(func: &Function, frame: &'frm mut CallFrame<'_>, heap: &'gc Collector, a : usize, b: usize) -> Result<(), InterpreterError> 
where 'gc : 'frm {
    let gbl_name = func.const_list[b].get_string();

    Ok(())
}

fn set_global<'gc>(func: &Function, frame: &CallFrame<'_>, heap: &'gc mut Collector, a: usize, b: usize) -> Result<(), InterpreterError> {
    let register_a = frame.load(a);
    let gbl_name = func.const_list[b].get_string();

    let err = match register_a.get_type() {
        TypeLua::Number => { heap.add_global(&gbl_name, register_a.get_number().unwrap()) }
        TypeLua::Boolean => { heap.add_global(&gbl_name, register_a.get_boolean().unwrap()) }
        _ => { panic!("Not implemented yet") }
    };

    err.or_else(
        |err| {
            return Err(InterpreterError::GlobalError { heap_error: err })
        }
    )
}

// TODO Handle Upvalues
fn return_instruction<'stk, 'frm, 'val>(stack: &'stk mut Stack<CallFrame<'_>>, frame : &'frm mut CallFrame<'_>, a : usize, b : usize) -> Result<CallFrame<'frm>, InterpreterError> 
where 'stk : 'frm, 'frm : 'val {
    let previous_frame = stack.pop().unwrap(); 
    if b != 1 {

    }
    Ok(previous_frame)
}

fn eval_instruction<'stk, 'gc, 'frm>(func : &Function, instr : &Instruction, stack: &'stk mut Stack<CallFrame<'_>>, frame : &'frm mut CallFrame<'_>, heap: &'gc mut Collector, pc : &mut usize) -> Result<(), InterpreterError> 
where 'gc : 'frm, 'stk : 'frm {

    // TODO changer champs struct Instruction
    match instr {
        Instruction::Move(_, a, b, _) => { move_operation(frame, *a as usize, *b as usize); }
        Instruction::LoadK(_, a, b) => { load_k(func, frame, *a as usize, *b as usize); }
        Instruction::LoadBool(_, a, b, c) => { load_bool(frame, *a as usize, *b as usize, *c as usize, pc); }
        Instruction::LoadNil(_, a, b, _) => { load_nil(frame, *a as usize, *b as usize); }
        Instruction::Add(_, a, b, c) => { add(func, frame, *a as usize, *b as usize, *c as usize); }
        Instruction::Return(_, a, b, _) => { return_instruction(stack, frame, *a as usize, *b as usize)?; }
        Instruction::GetGlobal(_, a, b) => { get_global(func, frame, heap, *a as usize, *b as usize)? }
        Instruction::SetGlobal(_, a, b) => { set_global(func, frame, heap, *a as usize, *b as usize)? }
        _ => { panic!("Not implemented {}", pc) }
    }

    Ok(())
}

fn eval_sequence<'gc, 'frm, 'stk>(main : Function, stack : &mut Stack<CallFrame<'_>>, heap : &'gc mut Collector) -> Result<(), InterpreterError> 
where 'gc: 'frm, 'stk : 'frm {

    let mut pc = 0;

    let mut frame : CallFrame<'_> = CallFrame::with_capacity(main.stack);

    while pc < main.instr_list.len() {
        // TODO make fields of Function private
        eval_instruction(&main, &main.instr_list[pc], stack, &mut frame, heap, &mut pc)?;
        pc += 1;
    }

    Ok(())
}

pub fn eval_program(main : Function) -> Result<(), Box<dyn Error>> {

    let mut heap = Collector::new();
    let mut stack: Stack<CallFrame<'_>> = Stack::new();
    stack.push(CallFrame::with_capacity(main.stack));

    eval_sequence(main, &mut stack, &mut heap)?;

    Ok(())
}