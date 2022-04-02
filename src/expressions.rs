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

impl Expr for Binary {
    fn evaluate(&self, env: Rc<Environment>) -> Result<Value, (String, Token)> {
        let left = self.left.evaluate(Rc::clone(&env))?;
        let right = self.right.evaluate(Rc::clone(&env))?;
        let token = self.op.clone();
        match self.op.token_type {
            TokenType::BangEqual => Ok(is_equal(left, right, true)),
            TokenType::EqualEqual => Ok(is_equal(left, right, true)),
            TokenType::Greater => match (left, right) {
                (Value::Number(a), Value::Number(b)) => Ok(Value::Bool(a > b)),
                _ => Err((String::from("Lze porovnat jen 2 cisla."), token)),
            },
            TokenType::GreaterEqual => match (left, right) {
                (Value::Number(a), Value::Number(b)) => Ok(Value::Bool(a >= b)),
                _ => Err((String::from("Lze porovnat jen 2 cisla."), token)),
            },
            TokenType::Less => match (left, right) {
                (Value::Number(a), Value::Number(b)) => Ok(Value::Bool(a < b)),
                _ => Err((String::from("Lze porovnat jen 2 cisla."), token)),
            },
            TokenType::LessEqual => match (left, right) {
                (Value::Number(a), Value::Number(b)) => Ok(Value::Bool(a <= b)),
                _ => Err((String::from("Lze porovnat jen 2 cisla."), token)),
            },
            TokenType::Minus => match (left, right) {
                (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a.clone() - b.clone())),
                _ => Err((String::from("Lze odecist jen 2 cisla."), token)),
            },
            TokenType::Plus => match (left, right) {
                (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a.clone() + b.clone())),
                (Value::String(a), Value::String(b)) => Ok(Value::String(format!("{}{}", a, b))),
                _ => Err((String::from("Lze spojit/secist jen dva retezce/cisla."), token)),
            },
            TokenType::Slash => match (left, right) {
                (Value::Number(a), Value::Number(b)) => {
                    if b.clone() == 0.0 {
                        Err((String::from("Nelze delit nulou."), token))
                    } else {
                        Ok(Value::Number(a.clone() / b.clone()))
                    }
                }
                _ => Err((String::from("Lze delit jen 2 cisla."), token)),
            },
            TokenType::Star => match (left, right) {
                (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a.clone() * b.clone())),
                _ => Err((String::from("Lze nasobit jen 2 cisla."), token)),
            },
            _ => Err((String::from("Neznama operace."), token)),
        }
    }

    fn kind(&self) -> Kind {
        Kind::Binary 
    }
}

fn is_equal(val1: Value, val2: Value, invert: bool) -> Value {
    if invert {
        Value::Bool(val1 != val2)
    } else {
        Value::Bool(val1 == val2)
    }
}
