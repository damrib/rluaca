pub mod config;

pub mod structure {

    pub mod function;
    pub mod local_variable;
    pub mod constant;
    pub mod instruction;

}

pub mod decompile {
    pub mod decompile;
    mod metadata;
}

pub mod garbage_collection;

pub mod interpreter;

pub mod object;

