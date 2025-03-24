use std::fmt;

#[derive(Debug)]
pub struct LocalVariable {
    identifier  : String,
    start_scope : u32,
    end_scope : u32
}


impl fmt::Display for LocalVariable {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {{ scope: {} - {} }}", self.identifier, self.start_scope, self.end_scope)
    }

}

impl LocalVariable {

    pub fn new(id : String, start : u32, end : u32) -> LocalVariable {

        LocalVariable {
            identifier  : id,
            start_scope : start,
            end_scope   : end
        }

    }

}