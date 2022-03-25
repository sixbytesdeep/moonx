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
#[derive(Debug, Clone)]
pub struct Class {}
#[derive(Debug, Clone)]
pub struct InstanceValue {}
