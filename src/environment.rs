use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::object::Object;

#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub struct Environment {
    pub store: HashMap<String, Object>,
    pub outer: Option<Rc<RefCell<Environment>>>,
}

impl Environment {
    pub fn get(&self, name: &str) -> Object {
        if let Some(obj) = self.store.get(name) {
            obj.clone()
        } else if let Some(outer) = &self.outer {
            outer.borrow().get(name)
        } else {
            Object::NullValue
        }
    }

    pub fn set(&mut self, name: String, value: Object) {
        self.store.insert(name, value);
    }
}
