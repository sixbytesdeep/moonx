use crate::moonenv::Environment;
use crate::value::Value;
use crate::token::Token;
use crate::tokentype::TokenType;
use std::rc::Rc;

pub trait Expr {
    fn evaluate(&self, env: Rc<Environment>) -> Result<Value, (String, Token)>;
    fn kind(&self) -> Kind;
}

pub enum Kind {
    Binary,
    Literal,
    Unary,
    Grouping,
    Variable(Token),
    NoOp,
    Logical,
    Assign,
    Call,
    Get(Token, Rc<dyn Expr>),
    This,
    Super,
    Set,
}

pub struct Binary {
    pub(crate) left: Rc<dyn Expr>,
    pub(crate) op: Token,
    pub(crate) right: Rc<dyn Expr>,
}
