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
            TokenType::EqualEqual => Ok(is_equal(left, right, false)),
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

pub struct Grouping {
    pub(crate) expression: Rc<dyn Expr>,
}

impl Expr for Grouping {
    fn evaluate(&self, env: Rc<Environment>) -> Result<Value, (String, Token)> {
        self.expression.evaluate(env)
    }

    fn kind(&self) -> Kind {
        Kind::Grouping
    }
}

pub struct Literal {
    pub(crate) value: crate::value::Value,
}

impl Expr for Literal {
    fn evaluate(&self, _env: Rc<Environment>) -> Result<Value, (String, Token)> {
        Ok(self.value.clone())
    }

    fn kind(&self) -> Kind {
        Kind::Literal
    }
}

pub struct Unary {
    pub(crate) operator: Token,
    pub(crate) right: Rc<dyn Expr>,
}

impl Expr for Unary {
    fn evaluate(&self, env: Rc<Environment>) -> Result<Value, (String, Token)> {
        let right = self.right.evaluate(env)?;
        match self.operator.token_type {
            TokenType::Minus => match right {
                Value::Number(a) => Ok(Value::Number(-a.clone())),
                _ => Err((String::from("Jsou mozna jen zaporna cisla."), self.operator.clone())),
            },
            TokenType::Bang => is_truth(right, true),
            _ => Err((String::from("Neznama operace"), self.operator.clone())),
        }
    }

    fn kind(&self) -> Kind {
        Kind::Unary
    }
}

pub struct Variable {
    pub(crate) name: Token,
}

impl Expr for Variable {
    fn evaluate(&self, env: Rc<Environment>) -> Result<Value, (String, Token)> {
        match env.get(&self.name) {
            Ok(val) => Ok(val.clone()),
            Err(e) => Err((e, self.name.clone())),
        }
    }

    fn kind(&self) -> Kind {
        Kind::Variable(self.name.clone())
    }
}

pub struct NoOp {}

impl Expr for NoOp {
    fn evaluate(&self, _env: Rc<Environment>) -> Result<Value, (String, Token)> {
        Ok(Value::None)
    }

    fn kind(&self) -> Kind {
        Kind::NoOp
    }
}

pub struct Assign {
    pub(crate) name: Token,
    pub(crate) value: Rc<dyn Expr>,
}

impl Expr for Assign {
    fn evaluate(&self, env: Rc<Environment>) -> Result<Value, (String, Token)> {
        let value = self.value.evaluate(Rc::clone(&env))?;
        match env.assign(&self.name, value.clone()) {
            Ok(_) => Ok(value.clone()),
            Err((msg, _token)) => Err((msg, self.name.clone())),
        }
    }

    fn kind(&self) -> Kind {
        Kind::Assign
    }
}

pub struct Logical {
    pub(crate) left: Rc<dyn Expr>,
    pub(crate) op: Token,
    pub(crate) right: Rc<dyn Expr>,
}

impl Expr for Logical {
    fn evaluate(&self, env: Rc<Environment>) -> Result<Value, (String, Token)> {
        let left = self.left.evaluate(Rc::clone(&env))?;
        match self.op.token_type {
            TokenType::Or => match is_truth(left.clone(), false)? {
                Value::Bool(true) => Ok(left.clone()),
                _ => Ok(self.right.evaluate(Rc::clone(&env))?),
            },
            _ => match is_truth(left.clone(), true)? {
                Value::Bool(true) => Ok(left.clone()),
                _ => Ok(self.right.evaluate(Rc::clone(&env))?),
            },
        }
    }

    fn kind(&self) -> Kind {
        Kind::Logical
    }
}

pub struct Call {
    pub(crate) calling: Rc<dyn Expr>,
    pub(crate) parent: Token,
    pub(crate) arguments: Vec<Rc<dyn Expr>>,
}

impl Expr for Call {
    fn evaluate(&self, env: Rc<Environment>) -> Result<Value, (String, Token)> {
        let function = self.calling.evaluate(Rc::clone(&env))?;
        let mut arguments: Vec<Value> = Vec::new();
        for argument in &self.arguments {
            arguments.push(argument.evaluate(Rc::clone(&env))?);
        }
        match function {
            Value::Function(callable) => {
                if callable.arity != arguments.len() {
                    Err((format!("Ocekavano {} argumentu ale bylo zadano {}.", callable.arity, arguments.len()), self.parent.clone()))
                } else {
                    match callable.call(arguments) {
                        Ok(a) => Ok(a),
                        Err((msg, token)) => Err((msg, token.clone())),
                    }
                }
            }
            Value::Class(class) => match class.call(arguments) {
                Ok(a) => Ok(a),
                Err((msg, token)) => Err((msg, token.clone())),
            },
            _ => Err((String::from("Lze volat jen funkce a tridy."), self.parent.clone())),
        }
    }

    fn kind(&self) -> Kind {
        Kind::Call 
    }
}

pub struct Get {
    pub(crate) object: Rc<dyn Expr>,
    pub(crate) name: Token,
}

impl Expr for Get {
    fn evaluate(&self, env: Rc<Environment>) -> Result<Value, (String, Token)> {
        let object = self.object.evaluate(env)?;
        match object {
            Value::Instance(instance) => instance.get_value(&self.name),

            _ => Err((String::from("Jen instance maji vlastnosti."), self.name.clone())),
        }
    }

    fn kind(&self) -> Kind {
        Kind::Get(self.name.clone(), Rc::clone(&self.object)) 
    }
}

pub struct Set {
    pub(crate) object: Rc<dyn Expr>,
    pub(crate) name: Token,
    pub(crate) value: Rc<dyn Expr>,
}

impl Expr for Set {
    fn evaluate(&self, env: Rc<Environment>) -> Result<Value, (String, Token)> {
        let object = self.object.evaluate(Rc::clone(&env))?;
        match object {
            Value::Instance(a) => {
                let value = self.value.evaluate(Rc::clone(&env))?;
                a.set_value(self.name.lexeme.clone(), value.clone());
                Ok(value)
            }
            _ => Err((String::from("Jen instance maji pole."), self.name.clone())),
        }
    }

    fn kind(&self) -> Kind {
        Kind::Set 
    }
}

pub struct This {
    pub(crate) keyword: Token,
}

impl Expr for This {
    fn evaluate(&self, env: Rc<Environment>) -> Result<Value, (String, Token)> {
        match env.get(&self.keyword) {
            Ok(a) => Ok(a),
            Err(msg) => Err((msg, self.keyword.clone())),
        }
    }

    fn kind(&self) -> Kind {
        Kind::This 
    }
}

pub struct Super {
    pub(crate) keyword: Token,
    pub(crate) method: Token,
}

impl Expr for Super {
    fn evaluate(&self, env: Rc<Environment>) -> Result<Value, (String, Token)> {
        match env.get_by_string(String::from("super")) {
            Ok(a) => match a {
                Value::Class(super_class) => {
                    match super_class.find_method(self.method.lexeme.clone()) {
                        None => Err((format!("Neznama property'{}'.", self.method.lexeme), self.keyword.clone())),
                        Some(method) => {
                            let this_instance = match env.get_by_string(String::from("this")).unwrap() {
                                Value::Instance(me) => me,
                                _ => {
                                    return Err((String::from("Zde melo byt this."), self.keyword.clone()));
                                }
                            };
                            method.bind(Value::Instance(Rc::clone(&this_instance)));
                            Ok(Value::Function(Rc::clone(&method)))
                        }
                    }
                }
                _ => Err((String::from("Chybi super."), self.keyword.clone())),
            },
            Err(msg) => Err((msg, self.keyword.clone())),
        }
    }

    fn kind(&self) -> Kind {
        Kind::Super 
    }
}

fn is_equal(val1: Value, val2: Value, invert: bool) -> Value {
    if invert {
        Value::Bool(val1 != val2)
    } else {
        Value::Bool(val1 == val2)
    }
}

pub fn is_truth(val: Value, invert: bool) -> Result<Value, (String, Token)> {
    match val {
        Value::Bool(a) => {
            if invert {
                Ok(Value::Bool(!a.clone()))
            } else {
                Ok(val.clone())
            }
        }
        Value::None => Ok(Value::Bool(false)),
        _ => Ok(Value::Bool(true)),
    }
}
