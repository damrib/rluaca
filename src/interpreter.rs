use crate::garbage_collection::{EnvironmentError, GlobalEnvironment};
use crate::structure::{function::Function, instruction::Instruction};
use crate::object::Value;
use std::error::Error;

#[derive(thiserror::Error, Debug)]
pub enum InterpreterError {
    #[error("Heap Error: ")]
    GlobalError {
        #[from]
        heap_error : EnvironmentError
    },
    #[error("Return not caught")]
    ReturnError,
    #[error("Dividing by zero")]
    ZeroDivisionError,
    #[error("Object is not callable")]
    NotCallableError
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

fn load_k<'frm>(main: &'frm Function, frame: &mut CallFrame<'frm>, a : usize, b : usize) {
    let constant = main.const_list[b].as_value();
    frame.store(a, constant);
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

fn get_rk<'frm>(func : &'frm Function, frame : &CallFrame<'frm>, r : usize) -> Value<'frm> {
    if r >= 256 {
        func.const_list[r % 256].as_value()
    } else {
        frame.load(r)
    }
}

fn arithmetic_operation<'frm>(func : &'frm Function, instr : &Instruction, frame : &mut CallFrame<'frm>, a : usize, b : usize, c : usize) -> Result<(), InterpreterError> {
    let rk_b = get_rk(func, frame, b);
    let rk_c = get_rk(func, frame, c);
    let number_b = rk_b.get_number().unwrap();
    let number_c = rk_c.get_number().unwrap();
    match instr {
        Instruction::Add(_, _, _, _) => { frame.store(a, Value::Number( number_b + number_c )) }
        Instruction::Sub(_, _, _, _) => { frame.store(a, Value::Number( number_b - number_c )) }
        Instruction::Mul(_, _, _, _) => { frame.store(a, Value::Number( number_b * number_c )) }
        Instruction::Pow(_, _, _, _) => { frame.store(a, Value::Number( number_b.powf(number_c) )) }
        Instruction::Div(_, _, _, _) => {
            if number_c == 0. {
                return Err(InterpreterError::ZeroDivisionError)
            }
            frame.store(a, Value::Number( number_b / number_c )) 
        }
        Instruction::Mod(_, _, _, _) => {
            if number_c == 0. {
                return Err(InterpreterError::ZeroDivisionError)
            }
            frame.store(a, Value::Number(number_b % number_c))
        }
        _ => { panic!("Should not call arithmetic operation for non arithmetic instruction") }
    }

    Ok(())
}

fn comparison_operator<'frm>(func : &'frm Function, instr: &Instruction, frame: &mut CallFrame<'frm>, a : usize, b : usize, c : usize, pc : &mut usize) {
    let register_a = frame.load(a);
    let rk_b = get_rk(func, frame, b);
    let rk_c = get_rk(func, frame, c);
    let boolean_a = register_a.get_boolean().unwrap();
    let number_b = rk_b.get_number().unwrap();
    let number_c = rk_c.get_number().unwrap();
    let order = number_b.total_cmp(&number_c);
    match instr {
        Instruction::Le(_, _, _, _) => { if order.is_le() != boolean_a { *pc += 1; } }
        Instruction::Lt(_, _, _, _) => { if order.is_lt() != boolean_a { *pc += 1; } }
        _ => panic!("Should not call comparison operator")
    }
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

fn get_global<'frm>(func : &'frm Function, frame: &mut CallFrame<'frm>, env : &GlobalEnvironment<'frm>, a : usize, b : usize) -> Result<(), InterpreterError> {
    let rk_b = func.const_list[b].as_value();
    let val = env.get_global(rk_b.get_string().unwrap()).or_else(
        |err| {
            return Err(InterpreterError::GlobalError { heap_error: err })
        }
    );
    frame.store(a, val?);
    Ok(())
}

fn set_global<'frm>(func : &'frm Function, frame: &mut CallFrame<'frm>, env : &mut GlobalEnvironment<'frm>, a : usize, b : usize) -> Result<(), InterpreterError> {
    let register_a = frame.load(a);
    let rk_b = func.const_list[b].as_value();
    env.insert_global(rk_b.get_string().unwrap(), register_a);

    Ok(())
}

fn call_instruction<'frm>(frame: &mut CallFrame<'frm>, env : &mut GlobalEnvironment<'frm>, a : usize, b : usize, c : usize) -> Result<(), InterpreterError> {
    let mut returned_values;
    let func_val = frame.load(a);
    match func_val {
        Value::LuaFunction(next_func) => {
            let mut new_frame = CallFrame::with_capacity(next_func.stack);
            let max_index = if b == 0 {
                frame.len()
            } else {
                    b
            };
                for i in 1..max_index {
                    new_frame.store(i - 1, frame.load(a + i));
                };
                returned_values = eval_sequence(next_func, new_frame, env)?;
        } 
        Value::RuntimeFunction(next_func) => {
            let stack_size: u8 = if b > 0 { (b-1) as u8 } else { b as u8 };  
            let mut new_frame = CallFrame::with_capacity(stack_size);
            for i in 1..=stack_size {
                new_frame.store((i-1) as usize, frame.load(a + (i as usize)));
            };
            returned_values = Vec::new();
            (next_func)(new_frame, &mut returned_values);
        }
        _ => { return Err(InterpreterError::NotCallableError) }
    }
    let max_index = if c == 0 { returned_values.len() } else { c - 1 };
    for i in 0..max_index {
        frame.store(i, returned_values[i]);
    }
    Ok(())
}

fn eval_instruction<'cur>(
    func : &'cur Function, 
    instr : &Instruction, 
    frame : &mut CallFrame<'cur>, 
    env : &mut GlobalEnvironment<'cur>,
    pc : &mut usize, 
    return_values : &mut Vec<Value<'cur>>) 
   -> Result<(), InterpreterError> {

    // TODO changer champs struct Instruction
    match instr {
        Instruction::Move(_, a, b, _) => { move_operation(frame, *a as usize, *b as usize) }
        Instruction::LoadK(_, a, b) => { load_k(func, frame, *a as usize, *b as usize) }
        Instruction::LoadBool(_, a, b, c) => { load_bool(frame, *a as usize, *b as usize, *c as usize, pc) }
        Instruction::LoadNil(_, a, b, _) => { load_nil(frame, *a as usize, *b as usize) }
        Instruction::Add(_, a, b, c) => { arithmetic_operation(func, instr, frame, *a as usize, *b as usize, *c as usize)? }
        Instruction::Sub(_, a, b, c) => { arithmetic_operation(func, instr, frame, *a as usize, *b as usize, *c as usize)? }
        Instruction::Mul(_, a, b, c) => { arithmetic_operation(func, instr, frame, *a as usize, *b as usize, *c as usize)? }
        Instruction::Div(_, a, b, c) => { arithmetic_operation(func, instr,frame, *a as usize, *b as usize, *c as usize)? }
        Instruction::Mod(_, a, b, c) => { arithmetic_operation(func, instr, frame, *a as usize, *b as usize, *c as usize)? }
        Instruction::Pow(_, a, b, c) => { arithmetic_operation(func, instr, frame, *a as usize, *b as usize, *c as usize)? }
        Instruction::Le(_, a, b, c) => { comparison_operator(func, instr, frame, *a as usize, *b as usize, *c as usize, pc); }
        Instruction::Lt(_, a, b, c) => { comparison_operator(func, instr, frame, *a as usize, *b as usize, *c as usize, pc); }
        Instruction::Jmp(_, _, b) => { jmp_instruction(pc, *b as usize) }
        Instruction::GetGlobal(_, a, b) => { get_global(func, frame, env, *a as usize, *b as usize)? }
        Instruction::SetGlobal(_, a, b) => { set_global(func, frame, env, *a as usize, *b as usize)? }
        Instruction::Call(_, a, b, c) => { call_instruction(frame, env, *a as usize, *b as usize, *c as usize)? } 
        Instruction::Closure(_, a, b) => { closure_instruction(func, frame, *a as usize, *b as usize)? }
        Instruction::Return(_, a, b, _) => { return_instruction(frame, *a as usize, *b as usize, return_values)? }
        _ => { panic!("Not implemented {}", pc) }
    }

    Ok(())
}

fn eval_sequence<'cur>(main : &'cur Function, mut frame : CallFrame<'cur>, env : &mut GlobalEnvironment<'cur>) -> Result<Vec<Value<'cur>>, InterpreterError> {

    let mut pc = 0;

    let mut result: Vec<Value<'_>> = Vec::new();

    while pc < main.instr_list.len() {
        // TODO make fields of Function private
        eval_instruction(&main, &main.instr_list[pc], &mut frame, env, &mut pc, &mut result)?;
        pc += 1;
    }

    Ok(result)
}

pub fn eval_program(main : Function) -> Result<(), Box<dyn Error>> {

    let frame : CallFrame<'_> = CallFrame::with_capacity(main.stack);
    let mut global_environement = GlobalEnvironment::new();

    eval_sequence(&main, frame, &mut global_environement)?;

    Ok(())
}