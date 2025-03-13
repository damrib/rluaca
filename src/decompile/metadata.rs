use std::fmt;

#[derive(Debug)]
pub struct Metadata {
    pub version     : u8,
    pub format      : u8,
    pub bigendian   : bool,
    pub i_size      : u8,
    pub u_size      : u8,
    pub instr_size  : u8,
    pub number_size : u8,
    pub int_flag    : bool
}

impl fmt::Display for Metadata {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Metadata:\n")?;
        write!(f, "\tVersion         : {}\n", self.version)?;
        write!(f, "\tFormat          : {}\n", self.format)?;
        write!(f, "\tBigendian       : {}\n", self.bigendian)?;
        write!(f, "\tInteger Size    : {}\n", self.i_size)?;
        write!(f, "\tUnsigned Size   : {}\n", self.u_size)?;
        write!(f, "\tInstruction Size: {}\n", self.instr_size)?;
        write!(f, "\tNumber Size     : {}\n", self.number_size)?;
        write!(f, "\tInteger Flag    : {}\n", self.int_flag)
    }
}

