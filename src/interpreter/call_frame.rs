use crate::interpreter::object::Value;

// this struct is 
pub struct CallFrame<'guard> {
    frame : Vec<Value<'guard>>,
    // useful for call, when b == 0 the arguments called in a function 
    // are all the values between a given register and the top of the stack 
    pub top_stack : usize
}

impl <'frm> CallFrame<'frm> {

    // Allocating a new stack frame with capacity n
    pub fn with_capacity(n : u8) -> Self {
        CallFrame {
            frame : vec![Value::Nil; n as usize],
            top_stack : 0
        }
    }

    // Stores a new value in the register with the given index
    pub fn store(&mut self, index: usize, v : Value<'frm>) {
        if index >= self.top_stack {
            self.top_stack = index + 1;
        }
        self.frame[index] = v;
    }

    // returns the value store in the register with the given index
    pub fn load(&self, index: usize) -> Value<'frm> {
        self.frame[index]
    }

    // Copies the move register
    pub fn move_register(&mut self, a : usize, b : usize) {
        let register_b = self.load(b);
        self.frame[a] = register_b;
    }

    pub fn len(&self) -> usize {
        self.top_stack
    }

    pub fn set_length(&mut self, size: usize) {
        self.top_stack = size;
    }

}