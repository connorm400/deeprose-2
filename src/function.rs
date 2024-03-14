use crate::object::LispObject;
use crate::env::Env;

pub struct LispFunction {
    env: Env,
    args: LispObject, // must be List(Vec<LispObject>)
    body: LispObject,
}

//TODO: implement lispFunction parsing stuff with lambda. & function calls & builtins