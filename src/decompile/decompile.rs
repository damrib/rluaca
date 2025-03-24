// TODO modularity

use crate::config::Vmconfig;
use crate::structure::{constant::Constant, local_variable::LocalVariable, instruction::{Instruction, InstructionError}, function::Function};
use crate::decompile::metadata::Metadata;

use std::{error::Error, fs};
use std::vec::IntoIter;

#[derive(thiserror::Error, Debug)]
enum DecompileError{
    #[error("Signature is invalid")]
    SignatureError,
    #[error("Missing Metadata")]
    MetadataError,
    #[error("Early end of file")]
    FileFormatError,
    #[error("Version of compiler is not the one specified in command line")]
    VersionDataError,
    #[error("byte does not specify constant type")]
    ConstantTypeError,
    #[error("Instruction opcode: {instr_code:?} is not recognized")]
    InstrEncodingError{
        instr_code : u64
    },
    #[error("Instruction Error: ")]
    InstrABError{
        #[from]
        instr_error : InstructionError 
    }
}

/// reads the next byte or return the error passed in argument if there are no more bytes to read 
fn next_byte_or_error(iter: &mut IntoIter<u8>, err : DecompileError) -> Result<u8, DecompileError> {
    match iter.next() {
        Some(b) => { Ok(b) }
        None        => { return Err(err) }
    }
}

/// reads size bytes interpreting them as bigendian interger
fn decode_bigendian(iter: &mut IntoIter<u8>, size: u8) -> Result<u64, DecompileError> {
    let mut res = 0;
    
    for _ in 0..size {
        let byte = next_byte_or_error(iter, DecompileError::FileFormatError)?;
        res = res * 256 + u64::from(byte);
    }

    Ok (res)
}

/// reads size bytes interpreting them as littleendian integer
fn decode_litendian(iter: &mut IntoIter<u8>, size: u8) -> Result<u64, DecompileError> {
    let mut res = 0;
    let mut acc = 1;

    for _ in 0..size {
        let byte = next_byte_or_error(iter, DecompileError::FileFormatError)?;
        
        res = res + u64::from(byte) * acc;
        acc = acc << 8;
    }    

    Ok(res)
}

/// reads size bytes and return an integer according to the endianness passed in argument
fn decode_int(iter: &mut IntoIter<u8>, size: u8, bigendian : bool) -> Result<u64, DecompileError> {

    let res = if bigendian {
        decode_bigendian(iter, size)?
    } else {
        decode_litendian(iter, size)?
    };

    Ok(res)
}

/// reads the signature in the header of the file, returns an error if it is incorrect
fn decode_signature(iter: &mut IntoIter<u8>) -> Result<(), DecompileError> {

    const SIGNATURE: u64 = 0x1B4C7561;
    const SIGNATURE_SIZE : u8 = 4;

    let res = decode_bigendian(iter, SIGNATURE_SIZE)?;

    if res != SIGNATURE {
        return Err(DecompileError::SignatureError)
    }

    Ok(())
}

/// parse the metadata of the given file
fn decode_metadata(iter: &mut IntoIter<u8>) -> Result<Metadata, DecompileError> {

    let mut next_meta_or_error = || {
        next_byte_or_error(iter, DecompileError::MetadataError)
    };

    let metadata = Metadata
    {
        version : next_meta_or_error()?,
        format : next_meta_or_error()?,
        bigendian : next_meta_or_error()? == 0,
        i_size : next_meta_or_error()?,
        u_size : next_meta_or_error()?,
        instr_size : next_meta_or_error()?,
        number_size : next_meta_or_error()?,
        int_flag : next_meta_or_error()? == 1
    };

    Ok (metadata)
}

/// Checks that the metadata of the file are correct (mainly that the version of lua is correct)
fn verify_metadata(metadata: &Metadata, config: Vmconfig) -> Result<(), DecompileError> {

    if u32::from(metadata.version) != config.get_ver() {
        return Err(DecompileError::VersionDataError);
    }

    Ok (())
}

/// parse a double number from the file
fn decode_double(iter: &mut IntoIter<u8>, size: u8, bigendian : bool) -> Result<f64, DecompileError>{

    let integer = decode_int(iter, size, bigendian)?;

    let res = f64::from_bits(integer);

    Ok (res)
}


/// parse a string from the file
fn decode_str(iter: &mut IntoIter<u8>, size: u64) -> Result<String, DecompileError>{

    let mut s = String::new();

    if size != 0 {

        for _ in 0..(size-1) {
            let c = char::from(next_byte_or_error(iter, DecompileError::FileFormatError)?); 
            s.push(c);
        }
    
        // We ignore the next byte as it represents the null character of the string
        next_byte_or_error(iter, DecompileError::FileFormatError)?;
    
    }

    Ok(s)
}

/// parse a constant from the file
fn decode_constant(iter: &mut IntoIter<u8>, metadata: &Metadata) -> Result<Constant, DecompileError>{

    let typ = next_byte_or_error(iter, DecompileError::FileFormatError)?;

    let cst = match typ {
        0 => { Constant::Null }
        1 => { 
            let byte = next_byte_or_error(iter, DecompileError::FileFormatError)?;
            Constant::Boolean(byte != 0)
        }
        3 => {
            let n = decode_double(iter, metadata.number_size, metadata.bigendian)?;
            Constant::Number(n)
        }
        4 => {
            let string_size = decode_int(iter, metadata.u_size, metadata.bigendian)?;
            let str = decode_str(iter, string_size)?;
            Constant::String(str)
        }
        _ => { return Err(DecompileError::ConstantTypeError) }
    };

    Ok (cst)
}

/// extract s bytes from n at position p
fn get_bits(n : u64, p: u8, s: u8) -> u64 {
    (n >> p) & (!((!0)<<s))
}

/// makes an instruction ABC from the bytes provided
fn get_register_abc(instruction_bytes: u64, opcode: u64) -> Result<Instruction, DecompileError> {

    let a = get_bits(instruction_bytes, 6, 8);
    let b = get_bits(instruction_bytes, 23, 9);
    let c = get_bits(instruction_bytes, 14, 9);

    Instruction::build_abc(opcode, a, b, c).map_err(
        |err| {
            DecompileError::InstrABError{instr_error: err}
        }
    )
} 

/// makes an instruction ABx from the bytes provided
fn get_register_abx(instruction_bytes: u64, opcode: u64) -> Result<Instruction, DecompileError> {

    let a = get_bits(instruction_bytes, 6, 8);
    let b = get_bits(instruction_bytes, 14, 18);

    Instruction::build_abx(opcode, a, b).map_err(
        |err| {
            DecompileError::InstrABError { instr_error: err }
        }
    )

}

/// makes an instruction AsB from the bytes provided
fn get_register_asb(instruction_bytes: u64, opcode: u64) -> Result<Instruction, DecompileError> {
    
    let a = get_bits(instruction_bytes, 6, 8);
    let b: i64 = get_bits(instruction_bytes, 14, 18) as i64 - 131071;


    Instruction::build_asb(opcode, a, b).map_err(
        |err| {
            DecompileError::InstrABError { instr_error: err }
        }
    )
    
} 

/// parse an instruction from the fiel
fn decode_instruction(iter: &mut IntoIter<u8>, metadata: &Metadata) -> Result<Instruction, DecompileError> {

    let instruction_bytes = decode_int(iter, metadata.instr_size, metadata.bigendian)?;

    let opcode = get_bits(instruction_bytes, 0, 6);

    match opcode {
        0 | 2..=4 | 6 | 8..=21 | 23..=30 | 33 | 34 | 35 | 37 => { Ok(get_register_abc(instruction_bytes, opcode)?) }
        1 | 5 | 7 | 36 => { Ok(get_register_abx(instruction_bytes, opcode)?) }
        22 | 31 | 32 => { Ok(get_register_asb(instruction_bytes, opcode)?) }
        _  => { return Err(DecompileError::InstrEncodingError{instr_code : opcode} ) }
    }
}

/// parse a list of element (Constant, Function, ...) according to the decode function provided
fn decode_list<T>(iter: &mut IntoIter<u8>, metadata: &Metadata, decoder : fn(&mut IntoIter<u8>, &Metadata) -> Result<T, DecompileError>) -> Result<Vec<T>, DecompileError> {

    let capacity = decode_int(iter, metadata.i_size, metadata.bigendian)? as usize;

    let mut res: Vec<T> = Vec::with_capacity(capacity);

    for _ in 0..capacity {
        res.push(decoder(iter, metadata)?);
    }

    Ok(res)
}

/// parse the list of lines
fn decode_lines_list(iter: &mut IntoIter<u8>, metadata: &Metadata) -> Result<Vec<u64>, DecompileError> {

    let number_lines = decode_int(iter, metadata.i_size, metadata.bigendian)?;

    let capacity = number_lines as usize;
    let mut res: Vec<u64> = Vec::with_capacity(capacity);

    for _ in 0..capacity {
        res.push(decode_int(iter, metadata.i_size, metadata.bigendian)?);
    }

    Ok(res)
}

/// parse the list of upvalues
fn decode_upvalues_list(iter: &mut IntoIter<u8>, metadata: &Metadata) -> Result<Vec<String>, DecompileError> {

    let number_lines = decode_int(iter, metadata.i_size, metadata.bigendian)?;

    let capacity = number_lines as usize;
    let mut res: Vec<String> = Vec::with_capacity(capacity);

    for _ in 0..capacity {
        let string_size = decode_int(iter, metadata.u_size, metadata.bigendian)?;
        res.push(decode_str(iter, string_size)?);
    }

    Ok(res)
}

/// parse a local variable
fn decode_local_variable(iter: &mut IntoIter<u8>, metadata: &Metadata) -> Result<LocalVariable, DecompileError> {

    let identifier_size = decode_int(iter, metadata.u_size, metadata.bigendian)?;

    let identifier = decode_str(iter, identifier_size)?;
    let start_scope   = decode_int(iter, metadata.i_size, metadata.bigendian)? as u32;
    let end_scope     = decode_int(iter, metadata.i_size, metadata.bigendian)? as u32;

    let res = LocalVariable::new(identifier, start_scope, end_scope);

    Ok(res)
}

/// parse a function
fn decode_function_block(iter: &mut IntoIter<u8>, metadata: &Metadata) -> Result<Function, DecompileError> {

    let name_size = decode_int(iter, metadata.u_size, metadata.bigendian)?;

    let res = Function {
        name       : decode_str(iter, name_size)?,
        first_line : decode_int(iter, metadata.i_size, metadata.bigendian)?,
        last_line  : decode_int(iter, metadata.i_size, metadata.bigendian)?,
        up_values  : next_byte_or_error(iter, DecompileError::FileFormatError)?,
        args       : next_byte_or_error(iter, DecompileError::FileFormatError)?,
        vargs      : next_byte_or_error(iter, DecompileError::FileFormatError)?,
        stack      : next_byte_or_error(iter, DecompileError::FileFormatError)?,
        instr_list : decode_list(iter, metadata, decode_instruction)?,
        const_list : decode_list(iter, metadata, decode_constant)?,
        func_list  : decode_list(iter, metadata, decode_function_block)?,
        lines_list : decode_lines_list(iter, metadata)?,
        local_list : decode_list(iter, metadata, decode_local_variable)?,
        upvalues_list : decode_upvalues_list(iter, metadata)?,
        // we assign the identifier to the function in the interpreter
        identifier : 0
    };

    Ok (res)
} 

/// parse the compiled file
fn decode_bytecode(mut iter:IntoIter<u8>, config: Vmconfig) -> Result<Function, DecompileError>{
    decode_signature(&mut iter)?;

    let dump = config.get_dump();

    let metadata = decode_metadata(&mut iter)?;
    verify_metadata(&metadata, config)?;

    let main = decode_function_block(&mut iter, &metadata)?;

    if dump {
        println!("{}", metadata);
        println!("{}", main);
    }

    Ok(main)
}

pub fn decompile(config : Vmconfig) -> Result<Function, Box<dyn Error>> {

    // reads the file ans transform the content into an iterator over an array of bytes
    // We only need to read each byte once
    let bytecode_iter = fs::read(config.get_path())?.into_iter();

    let main = decode_bytecode(bytecode_iter, config)?;

    Ok(main)
}