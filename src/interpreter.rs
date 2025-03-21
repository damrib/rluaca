use crate::garbage_collection::HeapError;
use crate::structure::{function::Function, instruction::Instruction};
use crate::object::Value;
use std::error::Error;

#[derive(thiserror::Error, Debug)]
pub enum InterpreterError {
    #[error("Heap Error: ")]
    GlobalError {
        #[from]
        heap_error : HeapError
    },
    #[error("Return not caught")]
    ReturnError
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

    pub fn len(&self) -> usize {
        self.frame.len()
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
    println!("ADD");
    if r >= 256 {
        func.const_list[r % 256].as_value()
    } else {
        frame.load(r)
    }
}

fn add(func : &Function, frame: &mut CallFrame<'_>, a : usize, b : usize, c : usize) {
    let rk_b = get_rk(func, frame, b);
    let rk_c = get_rk(func, frame, c);
    frame.store(a, Value::Number(rk_b.get_number().unwrap() + rk_c.get_number().unwrap()));
}

// TODO Handle Upvalues
fn return_instruction<'frm>(frame : &mut CallFrame<'frm>, a : usize, b : usize, return_values: &mut Vec<Value<'frm>>) -> Result<(), InterpreterError> {
    let max_index = if b == 0 {
        frame.len()
    } else {
        b - 1
    };

    for i in 0..max_index {
        return_values.push(frame.load(a + i));
    }
    Ok(())
}

fn jmp_instruction(pc: &mut usize, increment : usize) {
    *pc += increment;
}

fn closure_instruction<'cur>(func: &'cur Function, frame : &mut CallFrame<'cur>, a : usize, b : usize ) -> Result<(), InterpreterError> {
    let next_func = &func.func_list[b];
    frame.store(a, Value::LuaFunction(next_func));
    Ok(())
}

fn call_instruction(frame: &mut CallFrame<'_>, a : usize, b : usize, c : usize) -> Result<(), InterpreterError> {
    let func_val = frame.load(a);
    let next_func = func_val.get_function().unwrap();
    let mut new_frame = CallFrame::with_capacity(next_func.stack);
    let max_index = if b == 0 {
        frame.len()
    } else {
        b
    };
    for i in 1..max_index {
        new_frame.store(i - 1, frame.load(a + i));
    };
    let returned_values = eval_sequence(next_func, &mut new_frame)?;
    for i in 0..(c-1) {
        new_frame.store(i, returned_values[i]);
    }
    Ok(())
}

fn eval_instruction<'cur>(func : &'cur Function, instr : &Instruction, frame : &mut CallFrame<'cur>, pc : &mut usize, return_values : &mut Vec<Value<'cur>>) -> Result<(), InterpreterError> {

    // TODO changer champs struct Instruction
    match instr {
        Instruction::Move(_, a, b, _) => { move_operation(frame, *a as usize, *b as usize) }
        Instruction::LoadK(_, a, b) => { load_k(func, frame, *a as usize, *b as usize) }
        Instruction::LoadBool(_, a, b, c) => { load_bool(frame, *a as usize, *b as usize, *c as usize, pc) }
        Instruction::LoadNil(_, a, b, _) => { load_nil(frame, *a as usize, *b as usize) }
        Instruction::Add(_, a, b, c) => { add(func, frame, *a as usize, *b as usize, *c as usize) }
        Instruction::Jmp(_, _, b) => { jmp_instruction(pc, *b as usize) }
        Instruction::Call(_, a, b, c) => { call_instruction(frame, *a as usize, *b as usize, *c as usize)? } 
        Instruction::Closure(_, a, b) => { closure_instruction(func, frame, *a as usize, *b as usize)? }
        Instruction::Return(_, a, b, _) => { return_instruction(frame, *a as usize, *b as usize, return_values)? }
        _ => { panic!("Not implemented {}", pc) }
    }

    Ok(())
}

fn eval_sequence<'cur>(main : &'cur Function, frame : &mut CallFrame<'cur>) -> Result<Vec<Value<'cur>>, InterpreterError> {

    let mut pc = 0;

    let mut result: Vec<Value<'_>> = Vec::new();

    while pc < main.instr_list.len() {
        // TODO make fields of Function private
        eval_instruction(&main, &main.instr_list[pc], frame, &mut pc, &mut result)?;
        pc += 1;
    }

    Ok(result)
}

pub fn eval_program(main : Function) -> Result<(), Box<dyn Error>> {

    let mut frame : CallFrame<'_> = CallFrame::with_capacity(main.stack);

    eval_sequence(&main, &mut frame)?;

    Ok(())
}