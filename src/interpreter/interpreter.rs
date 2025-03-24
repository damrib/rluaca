use crate::interpreter::{global_environment::{EnvironmentError, GlobalEnvironment}, call_frame::CallFrame};
use crate::structure::{function::Function, instruction::Instruction};
use crate::interpreter::object::Value;
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
    NotCallableError,
    #[error("Object can't be tail called")]
    TailCallError,
    #[error("Error while making closure")]
    ClosureError
}

/// store the bth constant in the constant list of the current function in the ath register of the current frame
fn load_k<'frm>(func: &'frm Function, frame: &mut CallFrame<'frm>, a : usize, b : usize) {
    let constant = func.const_list[b].as_value();
    frame.store(a, constant);
}

/// store a boolean in the current stack frame, skips the next instruction if c is superior to 0
fn load_bool(frame : &mut CallFrame<'_>, a : usize, b : usize, c : usize, pc: &mut usize) {
    frame.store(a, Value::Boolean(b > 0));
    if c > 0 {
        *pc += 1;
    }
}

/// fils the frame with Nil between index a and b
fn load_nil(frame : &mut CallFrame<'_>, a : usize, b : usize) {
    
    for i in a..=b {
        frame.store(i, Value::Nil);
    }

}

/// The value in register b is copied in register a
fn move_operation(frame: &mut CallFrame<'_>, a : usize, b : usize) {
    frame.move_register(a, b);
}

/// returns the value stored in register r of the current stack frame if r < 256
/// returns a constant from the constant list of the current function
fn get_rk<'frm>(func : &'frm Function, frame : &CallFrame<'frm>, r : usize) -> Value<'frm> {
    if r >= 256 {
        func.const_list[r % 256].as_value()
    } else {
        frame.load(r)
    }
}

/* evaluates an arithmetic instruction and store the result in register a of the current frame
 * func : Function to access the constant list
 * instr : instruction to evaluate
 * frame : current stack frame
*/
fn arithmetic_operation<'frm>(
    func : &'frm Function, 
    instr : &Instruction, 
    frame : &mut CallFrame<'frm>, 
    a : usize, 
    b : usize, 
    c : usize) 
    -> Result<(), InterpreterError> {
    let rk_b = get_rk(func, frame, b);
    let rk_c = get_rk(func, frame, c);
    let number_b = rk_b.get_number().unwrap();
    let number_c = rk_c.get_number().unwrap();
    match instr {
        Instruction::Add(_, _, _) => { frame.store(a, Value::Number( number_b + number_c )) }
        Instruction::Sub(_, _, _) => { frame.store(a, Value::Number( number_b - number_c )) }
        Instruction::Mul(_, _, _) => { frame.store(a, Value::Number( number_b * number_c )) }
        Instruction::Pow(_, _, _) => { frame.store(a, Value::Number( number_b.powf(number_c) )) }
        Instruction::Div(_, _, _) => {
            if number_c == 0. {
                return Err(InterpreterError::ZeroDivisionError)
            }
            frame.store(a, Value::Number( number_b / number_c ))
        }
        Instruction::Mod(_, _, _) => {
            if number_c == 0. {
                return Err(InterpreterError::ZeroDivisionError)
            }
            frame.store(a, Value::Number(number_b % number_c))
        }
        _ => { panic!("Should not call arithmetic operation for non arithmetic instruction") }
    }

    Ok(())
}

/// store in register a the opposite value found in register b (LUANUMBER)
fn minus_operator(frame: &mut CallFrame<'_>, a: usize, b: usize) {
    let number = frame.load(b).get_number().unwrap();
    frame.store(a, Value::Number(-number));
} 

/// store in register a the opposite value found in register b (BOOLEAN)
fn not_operator(frame: &mut CallFrame<'_>, a: usize, b: usize) {
    let boolean = frame.load(b).get_boolean().unwrap();
    frame.store(a, Value::Boolean(!boolean));
}

/// store in register a the length of string found in register b
fn len_operator(frame: &mut CallFrame<'_>, a: usize, b: usize) {
    let lua_string = frame.load(b).get_string().unwrap();
    frame.store(a, Value::Number(lua_string.len() as f64));
}

fn test_operator(frame: &CallFrame<'_>, pc: &mut usize, a: usize, c: usize) {
    let register_a = frame.load(a);
    if (c > 0) != register_a.to_boolean() {
        *pc += 1;
    }
}

fn testset_operator(frame: &mut CallFrame<'_>, pc: &mut usize, a: usize, b: usize, c: usize) {
    let register_b = frame.load(b);
    if (c != 0) == register_b.to_boolean() {
        frame.store(a, register_b);
    } else {
        *pc += 1;
    }
}

/// store in register a the bth upvalue of the upvalue list passed in argument
fn get_upvalue<'frm>(frame: &mut CallFrame<'frm>, upvalues: &Vec<Value<'frm>>, a : usize, b: usize) {
    let upval = upvalues[b];
    frame.store(a, upval);
}

/// changes the bth value stored in the upvalue list by the value in register a 
fn set_upvalue<'frm>(frame: &CallFrame<'frm>, upvalues: &mut Vec<Value<'frm>>, a: usize, b: usize) {
    upvalues[b] = frame.load(a);
}

/* evaluates an arithmetic comparison instruction
 * skips the next instruction if the result is equal to the boolean in register a
 * func : Function to access the constant list
 * instr : instruction to evaluate
 * frame : current stack frame
 * pc : program counter pointing to the next instruction
*/
fn comparison_operator<'frm>(func : &'frm Function, instr: &Instruction, frame: &mut CallFrame<'frm>, a : usize, b : usize, c : usize, pc : &mut usize) {
    let rk_b = get_rk(func, frame, b);
    let rk_c = get_rk(func, frame, c);
    let boolean_a = a > 0;
    let number_b = rk_b.get_number().unwrap();
    let number_c = rk_c.get_number().unwrap();
    let order = number_b.total_cmp(&number_c);
    match instr {
        Instruction::Le(_, _, _) => { if order.is_le() != boolean_a { *pc += 1; } }
        Instruction::Lt(_, _, _) => { if order.is_lt() != boolean_a { *pc += 1; } }
        _ => panic!("Should not call comparison operator")
    }
}

/// test if the values in register b and c are equal 
/// and skips the next instruction if the result is equal to the boolean in register a
fn equality<'frm>(func: &'frm Function, frame: &mut CallFrame<'frm>, pc: &mut usize, a: usize, b: usize, c: usize) {
    let rk_b = get_rk(func, frame, b);
    let rk_c = get_rk(func, frame, c);
    let boolean_a = a > 0;
    if (rk_b == rk_c) != boolean_a { *pc += 1; }
}

/* Adds the resulting values of the current function in the return_values vector
 * a : register number of the first value to return
 * b : if 0 then return all values from a to the top of the stack else return b-1 values
*/ 
fn return_instruction<'frm>(frame : &mut CallFrame<'frm>, a : usize, b : usize, return_values: &mut Vec<Value<'frm>>) -> Result<(), InterpreterError> {
    
    let max_index = if b == 0 {
        frame.len() - a
    } else {
        b - 1
    };

    for i in 0..max_index {
        return_values.push(frame.load(a + i));
    }
    Ok(())
}

/* modify to program counter to skips instruction or loop */
fn jmp_instruction(pc: &mut usize, increment : isize) {
    if increment < 0 {
        *pc -= (-increment) as usize;
    } else {
        *pc += increment as usize;
    }
}

/// Instantiate a closure by putting the bth function of the function list of the current function in register a 
/// and instantiating the upvalue list of the closure
fn closure_instruction<'cur>(
    func: &'cur Function, 
    frame : &mut CallFrame<'cur>,
    upvalues : &mut Vec<Vec<Value<'cur>>>,
    pc : &mut usize,
    a : usize, 
    b : usize ) 
    -> Result<(), InterpreterError> {
    let next_func = &func.func_list[b];
    frame.store(a, Value::LuaFunction(next_func));

    // There is one Move or GetUPVal instruction following the closure per upvalue
    // We evaluate these instruction here because they behave differently than they normally do
    upvalues[next_func.identifier] = Vec::with_capacity(next_func.up_values as usize);
    for _ in 0..next_func.up_values {
        *pc += 1;
        match func.instr_list[*pc - 1] {
            Instruction::Move(_, reg_b, _) => { 
                upvalues[next_func.identifier].push(frame.load(reg_b));
            }
            Instruction::GetUpVal(_, reg_b, _) => { 
                let upval = upvalues[func.identifier][reg_b];
                upvalues[next_func.identifier].push(upval);
            }
            _ => { return Err(InterpreterError::ClosureError) }
        }

    }

    Ok(())
}

fn get_global<'frm>(
    func : &'frm Function, 
    frame: &mut CallFrame<'frm>, 
    env : &GlobalEnvironment<'frm>, 
    a : usize, 
    b : usize) 
    -> Result<(), InterpreterError> {
    let rk_b = func.const_list[b].as_value();
    let val = env.get_global(rk_b.get_string().unwrap()).map_err(
        |err| {
            InterpreterError::GlobalError { heap_error: err }
        }
    );
    frame.store(a, val?);
    Ok(())
}

fn set_global<'frm>(
    func : &'frm Function, 
    frame: &mut CallFrame<'frm>, 
    env : &mut GlobalEnvironment<'frm>, 
    a : usize, 
    b : usize) 
    -> Result<(), InterpreterError> {
    let register_a = frame.load(a);
    let rk_b = func.const_list[b].as_value();
    env.insert_global(rk_b.get_string().unwrap(), register_a);

    Ok(())
}

fn for_prep(frame: &mut CallFrame<'_>, pc: &mut usize, a: usize, b: isize) {
    let register_a = frame.load(a).get_number().unwrap();
    let stepping_value = frame.load(a + 2).get_number().unwrap();
    frame.store(a, Value::Number(register_a - stepping_value));
    jmp_instruction(pc, b);
}

fn for_loop(frame: &mut CallFrame<'_>, pc: &mut usize, a: usize, b: isize) {
    let number_a = frame.load(a).get_number().unwrap();
    let limit = frame.load(a + 1).get_number().unwrap();
    let stepping_value = frame.load(a + 2).get_number().unwrap();
    frame.store(a, Value::Number(number_a + stepping_value));
    
    let cmp_operator = if stepping_value < 0. { f64::gt } else { f64::lt };

    if cmp_operator(&number_a, &limit) {
        jmp_instruction(pc, b);
        frame.store(a + 3, frame.load(a));
    }
}

fn call_instruction<'frm>(
    frame: &mut CallFrame<'frm>, 
    env : &mut GlobalEnvironment<'frm>, 
    upvalues : &mut Vec<Vec<Value<'frm>>>,
    a : usize, 
    b : usize, 
    c : usize) 
    -> Result<(), InterpreterError> {

    let mut returned_values = Vec::new();
    let func_val = frame.load(a);

    match func_val {
        // evaluating function implemented in the program
        Value::LuaFunction(next_func) => {
            let mut new_frame = CallFrame::with_capacity(next_func.stack);
            // Adding the arguments of the called function in the stack frame of the next function
            // if b == 0 we add of the values from register a + 1 to the top of stack frame 
            let max_index = if b == 0 { frame.len() - a } else { b };
            for i in 1..max_index {
                new_frame.store(i - 1, frame.load(a + i));
            };

            eval_sequence(next_func, new_frame, env, upvalues, &mut returned_values)?;
        } 
        // evaluating function from the runtime library
        Value::RuntimeFunction(next_func) => {
            let stack_size= if b > 0 { b-1 } else { frame.len() - a - 1 };  
            let mut new_frame = CallFrame::with_capacity(stack_size as u8);
            // Adding the arguments of the called function in the stack frame of the next function
            for i in 0..stack_size {
                new_frame.store(i, frame.load(a + i + 1));
            };
            (next_func)(new_frame, &mut returned_values);
        }
        _ => return Err(InterpreterError::NotCallableError)
    }
    // Call instruction manipulates the top of the stack frame
    // Basically Call is supposed to pop the argument of the called function
    // from the stack frame.
    frame.set_length(a + 1);

    // Adding the returned values in the current stack frame
    let max_index = if c == 0 { returned_values.len() } else { c - 1 };
    for i in 0..max_index {
        frame.store(i + a, returned_values[i]);
    }

    Ok(())
}

// used to similate tail recursion
// Tail recursion optimization does not work with this implementation
fn tailcall_instruction<'frm>(
    frame: &mut CallFrame<'frm>,
    env : &mut GlobalEnvironment<'frm>,
    upvalues : &mut Vec<Vec<Value<'frm>>>,
    return_values : &mut Vec<Value<'frm>>,
    pc : &mut usize,
    a : usize,
    b : usize
) -> Result<(), InterpreterError> {
    let func_val = frame.load(a);
    match func_val {
        Value::LuaFunction(next_func) => {
            let mut new_frame = CallFrame::with_capacity(next_func.stack);
            // Adding the arguments of the called function in its stack frame
            // if b == 0, we add all values from regiter a + 1 to the top of the stack       
            let max_index = if b == 0 { frame.len() - a } else { b };
            for i in 1..max_index {
                new_frame.store(i - 1, frame.load(a + i));
            };
            // Call instruction manipulates the top of the stack frame
            // Basically Call is supposed to pop the argument of the called function
            // from the stack frame. 
            frame.set_length(a + 1);
            // A tail Call is always followed by two return instruction
            // We skip the first one as it tries to add values from the register
            // into return_values even though we do not put the returned_values 
            // in the stack frame
            *pc = usize::MAX;
            eval_sequence(next_func, new_frame, env, upvalues, return_values)
        }
        _ => { return Err(InterpreterError::TailCallError) }
    }
}

/* function treating one instruction of the function passed in argument and returning the results of the function through a vector
 * func : reference to the function we are evaluating
 * frame : stack frame of the current function
 * env : Table containing the global variables
 * upvalues : Vector of the upvalues of each function
 * pc : program counter
 * result : Vector through which we return the result of the function
 */
fn eval_instruction<'frm>(
    func : &'frm Function, 
    instr : &Instruction, 
    frame : &mut CallFrame<'frm>, 
    env : &mut GlobalEnvironment<'frm>,
    upvalues : &mut Vec<Vec<Value<'frm>>>,
    pc : &mut usize, 
    return_values : &mut Vec<Value<'frm>>) 
   -> Result<(), InterpreterError> {

    match *instr {
        Instruction::Move(a, b, _) => { move_operation(frame, a, b) }
        Instruction::LoadK(a, b) => { load_k(func, frame, a, b) }
        Instruction::LoadBool(a, b, c) => { load_bool(frame, a, b, c, pc) }
        Instruction::LoadNil(a, b, _) => { load_nil(frame, a, b) }
        Instruction::Add(a, b, c) => { arithmetic_operation(func, instr, frame, a, b, c)? }
        Instruction::Sub(a, b, c) => { arithmetic_operation(func, instr, frame, a, b, c)? }
        Instruction::Mul(a, b, c) => { arithmetic_operation(func, instr, frame, a, b, c)? }
        Instruction::Div(a, b, c) => { arithmetic_operation(func, instr, frame, a, b, c)? }
        Instruction::Mod(a, b, c) => { arithmetic_operation(func, instr, frame, a, b, c)? }
        Instruction::Pow(a, b, c) => { arithmetic_operation(func, instr, frame, a, b, c)? }
        Instruction::Unm(a, b, _) => { minus_operator(frame, a, b); }
        Instruction::Not(a, b, _) => { not_operator(frame, a, b); }
        Instruction::Len(a, b, _) => { len_operator(frame, a, b); }
        Instruction::Eq(a, b, c) => { equality(func, frame, pc, a, b, c); }
        Instruction::Le(a, b, c) => { comparison_operator(func, instr, frame, a, b, c, pc); }
        Instruction::Lt(a, b, c) => { comparison_operator(func, instr, frame, a, b, c, pc); }
        Instruction::Jmp(_, b) => { jmp_instruction(pc, b as isize) }
        Instruction::GetGlobal(a, b) => { get_global(func, frame, env, a, b)? }
        Instruction::SetGlobal(a, b) => { set_global(func, frame, env, a, b)? }
        Instruction::GetUpVal(a, b, _) => { get_upvalue(frame, &upvalues[func.identifier], a, b) }
        Instruction::SetUpVal(a, b, _) => { set_upvalue(frame, &mut upvalues[func.identifier], a, b); }
        Instruction::Test(a, _, c) => { test_operator(frame, pc, a, c); }
        Instruction::TestSet(a, b, c) => { testset_operator(frame, pc, a, b, c); }
        Instruction::Call(a, b, c) => { call_instruction(frame, env, upvalues, a, b, c)? } 
        Instruction::TailCall(a, b, _) => { tailcall_instruction(frame, env, upvalues, return_values, pc, a, b)? }
        Instruction::Closure(a, b) => { closure_instruction(func, frame, upvalues, pc, a, b)? }
        Instruction::Return(a, b, _) => { return_instruction(frame, a, b, return_values)?; *pc = usize::MAX; }
        Instruction::ForLoop(a, b) => { for_loop(frame, pc, a, b); }
        Instruction::ForPrep(a, b) => { for_prep(frame, pc, a, b); }
        _ => { panic!("Not implemented {}", pc) }
    }

    Ok(())
}

/* function evaluating the function passed in argument and returning the results of the function through a vector
 * func : reference to the function we are evaluating
 * frame : stack frame of the current function
 * env : Table containing the global variables
 * upvalues : Vector of the upvalues of each function
 * result : Vector through which we return the result of the function
 */
fn eval_sequence<'cur>(
    func : &'cur Function, 
    mut frame : CallFrame<'cur>, 
    env : &mut GlobalEnvironment<'cur>,
    upvalues : &mut Vec<Vec<Value<'cur>>>,
    result : &mut Vec<Value<'cur>>) 
    -> Result<(), InterpreterError> {

    let mut pc = 0;

    while pc < func.instr_list.len() {
        pc += 1;
        eval_instruction(&func, &func.instr_list[pc - 1], &mut frame, env, upvalues, &mut pc, result)?;
    }

    Ok(())
}

/// Instantiates the global environment and the local environment for the first function
/// and evaluates the function passed in argument
pub fn eval_program(mut main : Function) -> Result<(), Box<dyn Error>> {

    let frame : CallFrame<'_> = CallFrame::with_capacity(main.stack);
    let mut global_environement = GlobalEnvironment::new();
    let mut result: Vec<Value<'_>> = Vec::new();

    // We assign an id to each function that we use the find the upvalue list of each function during the interpretation
    let nb_upvalues = main.assign_upval_id();
    let mut upvalues_lists = vec![Vec::new(); nb_upvalues + 1];

    eval_sequence(&main, frame, &mut global_environement, &mut upvalues_lists, &mut result)?;

    Ok(())
}