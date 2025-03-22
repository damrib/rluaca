use crate::structure::local_variable::LocalVariable;
use crate::structure::constant::Constant;
use crate::structure::instruction::Instruction;

use std::fmt;
#[derive(Debug)]
pub struct Function{
    pub name          : String,
    pub first_line    : u64,
    pub last_line     : u64,
    pub up_values     : u8,
    pub args          : u8,
    pub vargs         : u8,
    pub stack         : u8,
    pub instr_list    : Vec<Instruction>,
    pub const_list    : Vec<Constant>,
    pub func_list     : Vec<Function>,
    pub lines_list    : Vec<u64>,
    pub local_list    : Vec<LocalVariable>,
    pub upvalues_list : Vec<String>,
    // To manage the upvalues of each function We need a way to identify
    // each function uniquely, the name is not enough as compilers
    // don't always 
    pub identifier    : usize
}

impl fmt::Display for Function {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.display_function(f, 0)
    }

}

impl Function {

    fn display_function(&self, f: &mut fmt::Formatter<'_>, tabulation : usize) -> fmt::Result {

        let tabs = "\t".repeat(tabulation);

        write!(f, "{tabs}Function {}: {{\n", self.name)?;
        write!(f, "{tabs}\tFirst Line: {}\n", self.first_line)?;
        write!(f, "{tabs}\tLast Line : {}\n", self.last_line)?;
        write!(f, "{tabs}\tUpValues  : {}\n", self.up_values)?;
        write!(f, "{tabs}\tArgs      : {}\n", self.args)?;
        write!(f, "{tabs}\tVargs     : {}\n", self.vargs)?;
        write!(f, "{tabs}\tStack     : {}\n", self.stack)?;

        write!(f, "{tabs}\tInstructions:\n")?;
        for instr in &self.instr_list {
            write!(f, "{tabs}\t\t{}\n", instr)?;
        }

        write!(f, "{tabs}\tConstants:\n")?;
        for cst in &self.const_list {
            write!(f, "{tabs}\t\t{}\n", cst)?;
        }    

        for func in &self.func_list {
            func.display_function(f, tabulation + 1)?;
        }

        write!(f, "{tabs}\tLines : [")?;
        for line in &self.lines_list {
            write!(f, "{}, ", line)?;
        }
        write!(f, "]\n")?;

        write!(f, "{tabs}\tLocal Variables:\n")?;
        for vars in &self.local_list {
            write!(f, "{tabs}\t\t{}\n", vars)?;
        }

        write!(f, "{tabs}\tUpvalues:\n")?;
        for val in &self.upvalues_list {
            write!(f, "{tabs}\t\t{}\n", val)?;
        }

        write!(f, "{tabs}}}\n")
    }

    fn assign_id(&mut self, mut id : usize) -> usize {

        self.identifier = id;
        for func in &mut self.func_list {
            id = func.assign_id(id + 1)
        }

        id
    }

    pub fn assign_upval_id(&mut self) -> usize {
        self.assign_id(0)
    }
}