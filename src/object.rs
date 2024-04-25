use crate::function::LispFunction;
use crate::env::Env;

#[derive(Debug, Clone)]
pub enum LispObject {
    //Function(LispFunction),
    List(Vec<LispObject>),
    Int(i32),
    Float(f64),
    Str(String),
    Symbol(String),
    Bool(bool),
    Err(LispError),
}

#[derive(Debug, Clone)]
pub enum LispError {
    DivideByZero,
    BadOperator,
    BadNumber,
    NotImplemented,
    BadForm,
    SymbolNotResolved,
}
