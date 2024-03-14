use std::rc::Rc;
use std::collections::HashMap;
use crate::object::LispObject;

pub struct Env {
    parent: Option<Rc<Env>>,
    store: HashMap<String, LispObject>
}

impl Env {
    pub fn new(outer: Option<Rc<Env>>) -> Rc<Env> {
        Rc::new(Env {
            parent: outer,
            store: HashMap::new(),
        })
    }

    pub fn get(&self, name: &str) -> Option<&LispObject> {
        self.store.get(name).or_else(move || {
            self.parent.as_ref().and_then(move |parent| {
                parent.get(name)
            })
        })
    }

    pub fn set(&mut self, name: String, value: LispObject) {
        self.store.insert(name, value);
    }
}