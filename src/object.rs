use crate::structure::function::Function;

#[derive(PartialEq, Debug)]
pub enum TypeLua{
    Number,
    Boolean,
    String,
    Nil,
    Table,
    Function
}

// I used a similar representation as the one described here: https://

#[derive(Clone, Copy, Debug)]
pub enum Value<'gc> {
    Number(f64),
    Boolean(bool),
    LuaFunction(&'gc Function),
    Nil
}

impl <'gc> Value<'gc> {

    pub fn is_nil(&self) -> bool {
        match *self {
            Self::Nil => { true }
            _         => { false }
        }
    }

    pub fn get_number(&self) -> Option<f64> {
        dbg![self.get_type()];
        match self {
            Self::Number(res) => { Some(*res) }
            _ => { None }
        }
    }

    pub fn get_boolean(&self) -> Option<bool> {
        match self {
            Self::Boolean(b) => { Some(*b) }
            _ => { None }
        }
    }

    pub fn get_function(&self) -> Option<&Function> {
        match self {
            Self::LuaFunction(f) => { Some(f) }
            _ => { None }
        }
    }

    pub fn get_type(&self) -> TypeLua {

        match self {
            Self::Boolean(_) => { TypeLua::Boolean }
            Self::Nil => { TypeLua::Nil }
            Self::Number(_) => { TypeLua::Number }
            Self::LuaFunction(_) => { TypeLua::Function }
        }

    }

}