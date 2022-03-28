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

pub struct Callable {
    pub(crate) arity: usize,
    pub(crate) function: Rc<dyn Fn(Vec<Value>, Rc<Environment>) -> Result<Value, (String, Token)>>,
    pub(crate) string: String,
    pub(crate) name: Token,
    pub(crate) environment: Rc<Environment>,
    pub(crate) is_initializer: RefCell<bool>,
}

impl Debug for Callable {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("Callable")
            .field("string", &self.string)
            .field("arity", &self.arity)
            .field("name", &self.name)
            .finish()
    }
}

impl Clone for Callable {
    fn clone(&self) -> Callable {
        let borrow: &Environment = self.environment.borrow();
        let env_clone = Rc::new(borrow.clone());
        Callable {
            arity: self.arity,
            function: Rc::clone(&self.function),
            string: self.string.clone(),
            name: self.name.clone(),
            environment: env_clone,
            is_initializer: RefCell::new(*self.is_initializer.borrow()),
        }
    }
}

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

impl Callable {
    pub(crate) fn call(&self, arguments: Vec<Value>) -> Result<Value, (String, Token)> {
        if self.arity != arguments.len() {
            return Err((
                    format!(
                        "Expected {} arguments but got {}.",
                        self.arity,
                        arguments.len()
                    ),
                    self.name.clone(),
            ));
        };

        self.environment.define(
            self.name.lexeme.clone(),
            Value::Function(Rc::new(self.clone())),
        );

        let result = (self.function) (arguments, Rc::clone(&self.environment));

        if *self.is_initializer.borrow() {
            match self.environment.get_by_string(String::from("this")) {
                Ok(a) => Ok(a),
                Err(msg) => Err((msg, self.name.clone())),
            }
        } else {
            result 
        }
    }

    pub(crate) fn bind(&self, instance: Value) {
        self.environment.define(String::from("this"), instance);
    }

    pub(crate) fn bind_super(&self, instance: Value) {
        self.environment.define(String::from("super"), instance);
    }

    pub(crate) fn set_initializer(&self) {
        self.is_initializer.swap(&RefCell::new(true));
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::String(a), Value::String(b)) => a == b,
            (Value::Number(a), Value::Number(b)) => a == b,
            (Value::None, Value::None) => true,
            (Value::Bool(a), Value::Bool(b)) => a == b,
            (Value::Function(a), Value::Function(b)) => Rc::ptr_eq(a, b),
            _ => false,
        }
    }
}

impl Eq for Value {}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Value::String(a) => write!(f, "\"{}\"", a),
            Value::Number(a) => write!(f, "{}", a),
            Value::Bool(a) => write!(f, "{}", a),
            Value::None => write!(f, "nil"),
            Value::Function(a) => write!(f, "{}", a.string),
            Value::Return(a) => write!(f, "<return {}>", a),
            Value::Class(a) => write!(f, "{}", a.name),
            Value::Instance(a) => write!(f, "{} instance", a.class.name),
        }
    }
}
