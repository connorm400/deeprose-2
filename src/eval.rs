use crate::env::Env;
use crate::object::{LispError as Err, LispObject as Obj};

pub fn eval(e: &Env, v: Obj) -> Obj {
    match v {
        Obj::Symbol(s) => {
            e.get(s.as_str())
                .map(|x| x.clone())
                .unwrap_or(Obj::Err(Err::SymbolNotResolved))
        },
        Obj::List(l) => eval_sexpr(e, l),
        // otherwise just return it
        _ => v
    }
}
fn eval_sexpr(e: &Env, mut l: Vec<Obj>) -> Obj {
    for (index, obj) in l.clone().iter().enumerate() {
        l[index] = eval(e, obj.clone());
    }

    for obj in &l {
        if let Obj::Err(e) = obj {
            return Obj::Err(e.clone());
        }
    }

    if l.len() == 0 { return Obj::List(l); }


    todo!()
}