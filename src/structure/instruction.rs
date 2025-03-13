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
  Move(u64, u64, u64, u64),
  LoadK(u64, u64, u64),
  LoadBool(u64, u64, u64, u64),
  LoadNil(u64, u64, u64, u64),
  GetUpVal(u64, u64, u64, u64),
  GetGlobal(u64, u64, u64),
  SetGlobal(u64, u64, u64),
  SetUpVal(u64, u64, u64, u64),
  GetTable(u64, u64, u64, u64),
  SetTable(u64, u64, u64, u64),
  NewTable(u64, u64, u64, u64),
  SelF(u64, u64, u64, u64),
  Add(u64, u64, u64, u64),
  Sub(u64, u64, u64, u64),
  Mul(u64, u64, u64, u64),
  Div(u64, u64, u64, u64),
  Mod(u64, u64, u64, u64), 
  Pow(u64, u64, u64, u64),
  Unm(u64, u64, u64, u64), 
  Not(u64, u64, u64, u64),
  Len(u64, u64, u64, u64),
  Concat(u64, u64, u64, u64),
  Jmp(u64, u64, i64),
  Eq(u64, u64, u64, u64),
  Lt(u64, u64, u64, u64),
  Le(u64, u64, u64, u64),
  Test(u64, u64, u64, u64),
  TestSet(u64, u64, u64, u64),
  Call(u64, u64, u64, u64),
  TailCall(u64, u64, u64, u64),
  Return(u64, u64, u64, u64),
  ForLoop(u64, u64, i64),
  ForPrep(u64, u64, i64),
  TForLoop(u64, u64, u64, u64),
  SetList(u64, u64, u64, u64),
  Close(u64, u64, u64, u64),
  Closure(u64, u64, u64),
  VarArg(u64, u64, u64, u64)
}

impl Instruction {

    pub fn build_abc(opcode : u64, a : u64, b : u64, c : u64) -> Result<Instruction, InstructionError> {

        match opcode {
            0  => { Ok(Instruction::Move(opcode, a, b, c)) }
            2  => { Ok(Instruction::LoadBool(opcode, a, b, c)) }
            3  => { Ok(Instruction::LoadNil(opcode, a, b, c)) }
            4  => { Ok(Instruction::GetUpVal(opcode, a, b, c)) }
            6  => { Ok(Instruction::GetTable(opcode, a, b, c)) }
            8  => { Ok(Instruction::SetUpVal(opcode, a, b, c)) }
            9  => { Ok(Instruction::SetTable(opcode, a, b, c)) }
            10 => { Ok(Instruction::NewTable(opcode, a, b, c)) }
            11 => { Ok(Instruction::SelF(opcode, a, b, c)) }
            12 => { Ok(Instruction::Add(opcode, a, b, c)) }
            13 => { Ok(Instruction::Sub(opcode, a, b, c)) }
            14 => { Ok(Instruction::Mul(opcode, a, b, c)) }
            15 => { Ok(Instruction::Div(opcode, a, b, c)) }
            16 => { Ok(Instruction::Mod(opcode, a, b, c)) }
            17 => { Ok(Instruction::Pow(opcode, a, b, c)) }
            18 => { Ok(Instruction::Unm(opcode, a, b, c)) }
            19 => { Ok(Instruction::Not(opcode, a, b, c)) }
            20 => { Ok(Instruction::Len(opcode, a, b, c)) }
            21 => { Ok(Instruction::Concat(opcode, a, b, c)) }
            23 => { Ok(Instruction::Eq(opcode, a, b, c)) }
            24 => { Ok(Instruction::Lt(opcode, a, b, c)) }
            25 => { Ok(Instruction::Le(opcode, a, b, c)) }
            26 => { Ok(Instruction::Test(opcode, a, b, c)) }
            27 => { Ok(Instruction::TestSet(opcode, a, b, c)) }
            28 => { Ok(Instruction::Call(opcode, a, b, c)) }
            29 => { Ok(Instruction::TailCall(opcode, a, b, c)) }
            30 => { Ok(Instruction::Return(opcode, a, b, c)) }
            33 => { Ok(Instruction::TForLoop(opcode, a, b, c)) }
            34 => { Ok(Instruction::SetList(opcode, a, b, c)) }
            35 => { Ok(Instruction::Close(opcode, a, b, c)) }
            37 => { Ok(Instruction::VarArg(opcode, a, b, c)) }
            _ => { return Err(InstructionError::NotABCError{ instr_code: opcode }) }
        
        }
    }

    pub fn build_abx(opcode : u64, a : u64, b : u64) -> Result<Instruction, InstructionError> {

        match opcode {
            1  => { Ok(Instruction::LoadK(opcode, a, b)) }
            5  => { Ok(Instruction::GetGlobal(opcode, a, b)) }
            7  => { Ok(Instruction::SetGlobal(opcode, a, b)) }
            36 => { Ok(Instruction::Closure(opcode, a, b)) }
            _  => { return Err(InstructionError::NotABxError{ instr_code: opcode }) }
        }

    }

    pub fn build_asb(opcode : u64, a : u64, b : i64) -> Result<Instruction, InstructionError> {

        match opcode {
            22 => { Ok(Instruction::Jmp(opcode, a, b)) }
            31 => { Ok(Instruction::ForLoop(opcode, a, b)) }
            32 => { Ok(Instruction::ForPrep(opcode, a, b)) }
            _  => { return Err(InstructionError::NotAsBError{ instr_code: opcode }) }
        } 

    }


}

impl fmt::Display for Instruction {

    fn fmt(&self, f : &mut fmt::Formatter<'_>) -> fmt::Result {
        
        match self {
            Instruction::Move(_, a, b, c)     => { write!(f, "Move      : [A: {}, B: {}, C: {}]", a, b, c) }
            Instruction::LoadK(_, a, b)             => { write!(f, "LoadK     : [A: {}, B: {}]", a, b) }
            Instruction::LoadBool(_, a, b, c) => { write!(f, "LoadBool  : [A: {}, B: {}, C: {}]", a, b, c) }
            Instruction::LoadNil(_, a, b, c)  => { write!(f, "LoadNil   : [A: {}, B: {}, C: {}]", a, b, c) }
            Instruction::GetUpVal(_, a, b, c) => { write!(f, "GetUpVal  : [A: {}, B: {}, C: {}]", a, b, c) }
            Instruction::GetGlobal(_, a, b)         => { write!(f, "GetGlobal : [A: {}, B: {}]", a, b) }
            Instruction::SetGlobal(_, a, b)         => { write!(f, "SetGlobal : [A: {}, B: {}]", a, b) }
            Instruction::SetUpVal(_, a, b, c) => { write!(f, "SetUpVal  : [A: {}, B: {}, C: {}]", a, b, c) }
            Instruction::GetTable(_, a, b, c) => { write!(f, "GetTable  : [A: {}, B: {}, C: {}]", a, b, c) }
            Instruction::SetTable(_, a, b, c) => { write!(f, "SetTable  : [A: {}, B: {}, C: {}]", a, b, c) }
            Instruction::NewTable(_, a, b, c) => { write!(f, "NewTable  : [A: {}, B: {}, C: {}]", a, b, c) }
            Instruction::SelF(_, a, b, c)     => { write!(f, "Self      : [A: {}, B: {}, C: {}]", a, b, c) }
            Instruction::Add(_, a, b, c)      => { write!(f, "Add       : [A: {}, B: {}, C: {}]", a, b, c) }
            Instruction::Sub(_, a, b, c)      => { write!(f, "Sub       : [A: {}, B: {}, C: {}]", a, b, c) }
            Instruction::Mul(_, a, b, c)      => { write!(f, "Mul       : [A: {}, B: {}, C: {}]", a, b, c) }
            Instruction::Div(_, a, b, c)      => { write!(f, "Div       : [A: {}, B: {}, C: {}]", a, b, c) }
            Instruction::Mod(_, a, b, c)      => { write!(f, "Mod       : [A: {}, B: {}, C: {}]", a, b, c) }
            Instruction::Pow(_, a, b, c)      => { write!(f, "Pow       : [A: {}, B: {}, C: {}]", a, b, c) }
            Instruction::Unm(_, a, b, c)      => { write!(f, "Unm       : [A: {}, B: {}, C: {}]", a, b, c) }
            Instruction::Not(_, a, b, c)      => { write!(f, "Not       : [A: {}, B: {}, C: {}]", a, b, c) }
            Instruction::Len(_, a, b, c)      => { write!(f, "Len       : [A: {}, B: {}, C: {}]", a, b, c) }
            Instruction::Concat(_, a, b, c)   => { write!(f, "Concat    : [A: {}, B: {}, C: {}]", a, b, c) }
            Instruction::Jmp(_, a, b)               => { write!(f, "Jmp       : [A: {}, B: {}]", a, b) }
            Instruction::Eq(_, a, b, c)       => { write!(f, "Eq        : [A: {}, B: {}, C: {}]", a, b, c) }
            Instruction::Lt(_, a, b, c)       => { write!(f, "Lt        : [A: {}, B: {}, C: {}]", a, b, c) }
            Instruction::Le(_, a, b, c)       => { write!(f, "Le        : [A: {}, B: {}, C: {}]", a, b, c) }
            Instruction::Test(_, a, b, c)     => { write!(f, "Test      : [A: {}, B: {}, C: {}]", a, b, c) }
            Instruction::TestSet(_, a, b, c)  => { write!(f, "TestSet   : [A: {}, B: {}, C: {}]", a, b, c) }
            Instruction::Call(_, a, b, c)     => { write!(f, "Call      : [A: {}, B: {}, C: {}]", a, b, c) }
            Instruction::TailCall(_, a, b, c) => { write!(f, "TailCall  : [A: {}, B: {}, C: {}]", a, b, c) }
            Instruction::Return(_, a, b, c)   => { write!(f, "Return    : [A: {}, B: {}, C: {}]", a, b, c) }
            Instruction::ForLoop(_, a, b)           => { write!(f, "ForLoop   : [A: {}, B: {}]", a, b) }
            Instruction::ForPrep(_, a, b)           => { write!(f, "ForPrep   : [A: {}, B: {}]", a, b) }
            Instruction::TForLoop(_, a, b, c) => { write!(f, "TForLoop  : [A: {}, B: {}, C: {}]", a, b, c) }
            Instruction::SetList(_, a, b, c)  => { write!(f, "SetList   : [A: {}, B: {}, C: {}]", a, b, c) }
            Instruction::Close(_, a, b, c)    => { write!(f, "Close     : [A: {}, B: {}, C: {}]", a, b, c) }
            Instruction::Closure(_, a, b)           => { write!(f, "Closure   : [A: {}, B: {}]", a, b) }
            Instruction::VarArg(_, a, b, c)   => { write!(f, "VarArg    : [A: {}, B: {}, C: {}]", a, b, c) }
        }

    }


}