#[derive(Debug, Clone)]
pub enum LispObject {
    List(Vec<LispObject>),
    Int(i32),
    Str(String),
    Symbol(String),
    Bool(bool),
}