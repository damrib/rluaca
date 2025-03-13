use simple_stack::Stack;

use crate::structure::{function::Function, instruction::Instruction};
use crate::object::Value;

// TODO: implement Runtime Error

// TODO Modularity
struct CallFrame<'guard> {
    frame : Vec<Value<'guard>>,
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
        let register_b = self.frame[b];
        self.frame[a] = register_b;
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

fn get_rk<'frm>(func : &Function, frame : &CallFrame<'frm>, r : usize) -> Value<'frm> {
    if r > 256 {
        func.const_list[r].as_value()
    } else {
        frame.load(r)
    }
}

fn add(func : &Function, frame: &mut CallFrame<'_>, a : usize, b : usize, c : usize) {
    let rk_b = get_rk(func, frame, b);
    let rk_c = get_rk(func, frame, c);
    frame.store(a, Value::Number(rk_b.get_number() + rk_c.get_number()));
}

fn return_instruction(frame : &mut CallFrame, a : usize, b : usize) {


    // TODO Handle Upvalues
}

fn eval_instruction(func : &Function, instr : &Instruction, stack: &mut Stack<CallFrame<'_>>, frame : &mut CallFrame<'_>, pc : &mut usize) -> () {

    // TODO changer champs struct Instruction
    match instr {
        Instruction::Move(_, a, b, _) => { move_operation(frame, *a as usize, *b as usize) }
        Instruction::LoadK(_, a, b) => { load_k(func, frame, *a as usize, *b as usize) }
        Instruction::LoadBool(_, a, b, c) => { load_bool(frame, *a as usize, *b as usize, *c as usize, pc) }
        Instruction::LoadNil(_, a, b, _) => { load_nil(frame, *a as usize, *b as usize) }
        Instruction::Add(_, a, b, c) => { add(func, frame, *a as usize, *b as usize, *c as usize) }
        Instruction::Return(_, a, b, _) => { return_instruction(frame, *a as usize, *b as usize) }
        _ => { panic!("Not implemented {}", pc) }
    }

}

fn eval_sequence(main : Function, stack : &mut Stack<CallFrame<'_>>) -> () {

    let mut pc = 0;

    let mut frame : CallFrame<'_> = CallFrame::with_capacity(main.stack);

    while pc < main.instr_list.len() {
        // TODO make fields of Function private
        eval_instruction(&main, &main.instr_list[pc], stack, &mut frame, &mut pc);
        pc += 1;
    }

}

pub fn eval_program(main : Function) -> () {

    let mut stack: Stack<CallFrame<'_>> = Stack::new();
    stack.push(CallFrame::with_capacity(main.stack));

    eval_sequence(main, &mut stack);

}