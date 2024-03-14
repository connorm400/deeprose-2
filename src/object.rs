use crate::function::LispFunction;

#[derive(Debug, Clone)]
pub enum LispObject {
    //Function(LispFunction),
    List(Vec<LispObject>),
    Int(i32),
    Float(f64),
    Str(String),
    Symbol(String),
    Bool(bool),
}

