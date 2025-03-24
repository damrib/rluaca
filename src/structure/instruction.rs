use std::fmt;

#[derive(thiserror::Error, Debug)]
pub enum InstructionError {
    #[error("Instruction with opcode: {instr_code:?} is not an ABC instruction")]
    NotABCError{
        instr_code : u64
    },
    #[error("Instruction with opcode: {instr_code:?} is not an AsB instruction")]
    NotABxError{
        instr_code : u64
    },
    #[error("Instruction with opcode: {instr_code:?} is not an AsB instruction")]
    NotAsBError{
        instr_code : u64
    }
}

// TODO minimiser espace memoire utiliser ici
#[derive(Debug)]
pub enum Instruction {
  Move(usize, usize, usize),
  LoadK(usize, usize),
  LoadBool(usize, usize, usize),
  LoadNil(usize, usize, usize),
  GetUpVal(usize, usize, usize),
  GetGlobal(usize, usize),
  SetGlobal(usize, usize),
  SetUpVal(usize, usize, usize),
  GetTable(usize, usize, usize),
  SetTable(usize, usize, usize),
  NewTable(usize, usize, usize),
  SelF(usize, usize, usize),
  Add(usize, usize, usize),
  Sub(usize, usize, usize),
  Mul(usize, usize, usize),
  Div(usize, usize, usize),
  Mod(usize, usize, usize), 
  Pow(usize, usize, usize),
  Unm(usize, usize, usize), 
  Not(usize, usize, usize),
  Len(usize, usize, usize),
  Concat(usize, usize, usize),
  Jmp(usize, isize),
  Eq(usize, usize, usize),
  Lt(usize, usize, usize),
  Le(usize, usize, usize),
  Test(usize, usize, usize),
  TestSet(usize, usize, usize),
  Call(usize, usize, usize),
  TailCall(usize, usize, usize),
  Return(usize, usize, usize),
  ForLoop(usize, isize),
  ForPrep(usize, isize),
  TForLoop(usize, usize, usize),
  SetList(usize, usize, usize),
  Close(usize, usize, usize),
  Closure(usize, usize),
  VarArg(usize, usize, usize)
}

impl Instruction {

    pub fn build_abc(opcode : u64, a : u64, b : u64, c : u64) -> Result<Instruction, InstructionError> {
        let a = a as usize;
        let b = b as usize;
        let c = c as usize;
        match opcode {
            0  => { Ok(Instruction::Move(a, b, c)) }
            2  => { Ok(Instruction::LoadBool(a, b, c)) }
            3  => { Ok(Instruction::LoadNil(a, b, c)) }
            4  => { Ok(Instruction::GetUpVal(a, b, c)) }
            6  => { Ok(Instruction::GetTable(a, b, c)) }
            8  => { Ok(Instruction::SetUpVal(a, b, c)) }
            9  => { Ok(Instruction::SetTable(a, b, c)) }
            10 => { Ok(Instruction::NewTable(a, b, c)) }
            11 => { Ok(Instruction::SelF(a, b, c)) }
            12 => { Ok(Instruction::Add(a, b, c)) }
            13 => { Ok(Instruction::Sub(a, b, c)) }
            14 => { Ok(Instruction::Mul(a, b, c)) }
            15 => { Ok(Instruction::Div(a, b, c)) }
            16 => { Ok(Instruction::Mod(a, b, c)) }
            17 => { Ok(Instruction::Pow(a, b, c)) }
            18 => { Ok(Instruction::Unm(a, b, c)) }
            19 => { Ok(Instruction::Not(a, b, c)) }
            20 => { Ok(Instruction::Len(a, b, c)) }
            21 => { Ok(Instruction::Concat(a, b, c)) }
            23 => { Ok(Instruction::Eq(a, b, c)) }
            24 => { Ok(Instruction::Lt(a, b, c)) }
            25 => { Ok(Instruction::Le(a, b, c)) }
            26 => { Ok(Instruction::Test(a, b, c)) }
            27 => { Ok(Instruction::TestSet(a, b, c)) }
            28 => { Ok(Instruction::Call(a, b, c)) }
            29 => { Ok(Instruction::TailCall(a, b, c)) }
            30 => { Ok(Instruction::Return(a, b, c)) }
            33 => { Ok(Instruction::TForLoop(a, b, c)) }
            34 => { Ok(Instruction::SetList(a, b, c)) }
            35 => { Ok(Instruction::Close(a, b, c)) }
            37 => { Ok(Instruction::VarArg(a, b, c)) }
            _ => { return Err(InstructionError::NotABCError{ instr_code: opcode }) }
        
        }
    }

    pub fn build_abx(opcode : u64, a : u64, b : u64) -> Result<Instruction, InstructionError> {

        let a = a as usize;
        let b = b as usize;

        match opcode {
            1  => { Ok(Instruction::LoadK(a, b)) }
            5  => { Ok(Instruction::GetGlobal(a, b)) }
            7  => { Ok(Instruction::SetGlobal(a, b)) }
            36 => { Ok(Instruction::Closure(a, b)) }
            _  => { return Err(InstructionError::NotABxError{ instr_code: opcode }) }
        }

    }

    pub fn build_asb(opcode : u64, a : u64, b : i64) -> Result<Instruction, InstructionError> {

        let a = a as usize;
        let b = b as isize;
        match opcode {
            22 => { Ok(Instruction::Jmp(a, b)) }
            31 => { Ok(Instruction::ForLoop(a, b)) }
            32 => { Ok(Instruction::ForPrep(a, b)) }
            _  => { return Err(InstructionError::NotAsBError{ instr_code: opcode }) }
        } 

    }


}

impl fmt::Display for Instruction {

    fn fmt(&self, f : &mut fmt::Formatter<'_>) -> fmt::Result {
        
        match self {
            Instruction::Move(a, b, c)     => { write!(f, "Move      : [A: {}, B: {}, C: {}]", a, b, c) }
            Instruction::LoadK(a, b)             => { write!(f, "LoadK     : [A: {}, B: {}]", a, b) }
            Instruction::LoadBool(a, b, c) => { write!(f, "LoadBool  : [A: {}, B: {}, C: {}]", a, b, c) }
            Instruction::LoadNil(a, b, c)  => { write!(f, "LoadNil   : [A: {}, B: {}, C: {}]", a, b, c) }
            Instruction::GetUpVal(a, b, c) => { write!(f, "GetUpVal  : [A: {}, B: {}, C: {}]", a, b, c) }
            Instruction::GetGlobal(a, b)         => { write!(f, "GetGlobal : [A: {}, B: {}]", a, b) }
            Instruction::SetGlobal(a, b)         => { write!(f, "SetGlobal : [A: {}, B: {}]", a, b) }
            Instruction::SetUpVal(a, b, c) => { write!(f, "SetUpVal  : [A: {}, B: {}, C: {}]", a, b, c) }
            Instruction::GetTable(a, b, c) => { write!(f, "GetTable  : [A: {}, B: {}, C: {}]", a, b, c) }
            Instruction::SetTable(a, b, c) => { write!(f, "SetTable  : [A: {}, B: {}, C: {}]", a, b, c) }
            Instruction::NewTable(a, b, c) => { write!(f, "NewTable  : [A: {}, B: {}, C: {}]", a, b, c) }
            Instruction::SelF(a, b, c)     => { write!(f, "Self      : [A: {}, B: {}, C: {}]", a, b, c) }
            Instruction::Add(a, b, c)      => { write!(f, "Add       : [A: {}, B: {}, C: {}]", a, b, c) }
            Instruction::Sub(a, b, c)      => { write!(f, "Sub       : [A: {}, B: {}, C: {}]", a, b, c) }
            Instruction::Mul(a, b, c)      => { write!(f, "Mul       : [A: {}, B: {}, C: {}]", a, b, c) }
            Instruction::Div(a, b, c)      => { write!(f, "Div       : [A: {}, B: {}, C: {}]", a, b, c) }
            Instruction::Mod(a, b, c)      => { write!(f, "Mod       : [A: {}, B: {}, C: {}]", a, b, c) }
            Instruction::Pow(a, b, c)      => { write!(f, "Pow       : [A: {}, B: {}, C: {}]", a, b, c) }
            Instruction::Unm(a, b, c)      => { write!(f, "Unm       : [A: {}, B: {}, C: {}]", a, b, c) }
            Instruction::Not(a, b, c)      => { write!(f, "Not       : [A: {}, B: {}, C: {}]", a, b, c) }
            Instruction::Len(a, b, c)      => { write!(f, "Len       : [A: {}, B: {}, C: {}]", a, b, c) }
            Instruction::Concat(a, b, c)   => { write!(f, "Concat    : [A: {}, B: {}, C: {}]", a, b, c) }
            Instruction::Jmp(a, b)               => { write!(f, "Jmp       : [A: {}, B: {}]", a, b) }
            Instruction::Eq(a, b, c)       => { write!(f, "Eq        : [A: {}, B: {}, C: {}]", a, b, c) }
            Instruction::Lt(a, b, c)       => { write!(f, "Lt        : [A: {}, B: {}, C: {}]", a, b, c) }
            Instruction::Le(a, b, c)       => { write!(f, "Le        : [A: {}, B: {}, C: {}]", a, b, c) }
            Instruction::Test(a, b, c)     => { write!(f, "Test      : [A: {}, B: {}, C: {}]", a, b, c) }
            Instruction::TestSet(a, b, c)  => { write!(f, "TestSet   : [A: {}, B: {}, C: {}]", a, b, c) }
            Instruction::Call(a, b, c)     => { write!(f, "Call      : [A: {}, B: {}, C: {}]", a, b, c) }
            Instruction::TailCall(a, b, c) => { write!(f, "TailCall  : [A: {}, B: {}, C: {}]", a, b, c) }
            Instruction::Return(a, b, c)   => { write!(f, "Return    : [A: {}, B: {}, C: {}]", a, b, c) }
            Instruction::ForLoop(a, b)           => { write!(f, "ForLoop   : [A: {}, B: {}]", a, b) }
            Instruction::ForPrep(a, b)           => { write!(f, "ForPrep   : [A: {}, B: {}]", a, b) }
            Instruction::TForLoop(a, b, c) => { write!(f, "TForLoop  : [A: {}, B: {}, C: {}]", a, b, c) }
            Instruction::SetList(a, b, c)  => { write!(f, "SetList   : [A: {}, B: {}, C: {}]", a, b, c) }
            Instruction::Close(a, b, c)    => { write!(f, "Close     : [A: {}, B: {}, C: {}]", a, b, c) }
            Instruction::Closure(a, b)           => { write!(f, "Closure   : [A: {}, B: {}]", a, b) }
            Instruction::VarArg(a, b, c)   => { write!(f, "VarArg    : [A: {}, B: {}, C: {}]", a, b, c) }
        }

    }


}