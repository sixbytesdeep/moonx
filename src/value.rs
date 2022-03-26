use crate::token::Token;
use crate::moonenv::Environment;
use std::borrow::Borrow;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;
use std::fmt::{Debug, Formatter};
use std::rc::Rc;

#[derive(Debug, Clone)]
pub enum Value {
    String(String),
    Number(f64),
    Bool(bool),
    None,
    Function(Rc<Callable>),
    Return(Box<Value>),
    Class(Rc<Class>),
    Instance(Rc<InstanceValue>),
}

#[derive(Debug, Clone)]
pub struct Callable {}
#[derive(Debug)]
pub struct Class {
    pub(crate) name: String,
    pub(crate) arity: usize,
    pub(crate) methods: RefCell<HashMap<String, Value>>,
    pub(crate) super_class: Option<Rc<Class>>,
}
#[derive(Debug, Clone)]
pub struct InstanceValue {
    pub(crate) class: Rc<Class>,
    pub(crate) fields: RefCell<HashMap<String, Value>>,
}

impl InstanceValue {
    pub fn get_value(&self, name: &Token) -> Result<Value, (String, Token)> {
        match self.class.find_method(name.clone().lexeme) {
            None => {},
            Some(callable) => {
                let updated_method = callable.clone();
                updated_method.bind(Value::Instance(Rc::new(self.clone())));
                return Ok(Value::Function(updated_method));
            }
        }

        match self.fields.borrow_mut().get(&*name.lexeme) {
            None => Err((
                    format!("Undefined property '{}'.", name.lexeme),
                    name.clone(),
            )),
            Some(value) => Ok(value.clone()),
        }
    }

    pub fn set_value(&self, name: String, value: Value) {
        self.fields.borrow_mut().insert(name, value);
    }
}

impl Clone for Class {
    fn clone(&self) -> Self {
        Class {
            name: self.name.clone(),
            arity: self.arity,
            methods: RefCell::clone(&self.methods),
            super_class: self.super_class.clone(),
        }
    }
}

impl Class {
    pub(crate) fn call(&self, arguments: Vec<Value>) -> Result<Value, (String, Token)> {
        let instance = Rc::new(InstanceValue {
            class: Rc::new(self.clone()),
            fields: RefCell::new(HashMap::new()),
        });
        match self.methods.borrow().get("init") {
            Some(a) => match a {
                Value::Function(callable) => {
                    callable.bind(Value::Instance(Rc::clone(&instance)));
                    return callable.call(arguments);
                }
                _ => {},
            }
            _ => {},
        }
        Ok(Value::Instance(instance))
    }

    pub(crate) fn find_method(&self, name: String) -> Option<Rc<Callable>> {
        match self.methods.borrow().get(&*name) {
            None => match &self.super_class {
                None => None,
                Some(a) => a.find_method(name),
            },
            Some(method) => match method {
                Value::Function(callable) => Some(Rc::clone(callable)),
                _ => None,
            },
        }
    }
}
