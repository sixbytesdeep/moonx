use crate::moonenv::Environment;
use crate::expressions::{is_truth, Expr, Kind};
use crate::value::{Callable, Class, Value};
use crate::token::Token;
use crate::interpreter::Interpreter;
use std::borrow::Borrow;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub trait Statement {
    fn evaluate(&self, env: Rc<Environment>) -> Result<Value, (String, Token)>;
    fn kind(&self) -> StatementKind;
}

pub enum StatementKind {
    Expression,
    Print,
    Var,
    Block,
    If,
    While,
    Function(Function),
    ReturnStatement,
    ClassStatement,
}

pub struct Expression {
    pub(crate) expression: Rc<dyn Expr>,
}

impl Statement for Expression {
    fn evaluate(&self, env: Rc<Environment>) -> Result<Value, (String, Token)> {
        match self.expression.evaluate(env) {
            Ok(value) => {
                println!("{}", value);
                Ok(Value::None)
            }
            Err(err) => Err(err),
        }
    }

    fn kind(&self) -> StatementKind {
        StatementKind::Expression
    }
}

pub struct Print {
    pub(crate) expressions: Rc<dyn Expr>,
}

impl Statement for Print {
    fn evaluate(&self, env: Rc<Environment>) -> Result<Value, (String, Token)> {
        match self.expressions.evaluate(env) {
            Ok(value) => {
                println!("{}", value);
                Ok(Value::None)
            }
            Err(err) => Err(err),
        }
    }

    fn kind(&self) -> StatementKind {
        StatementKind::Print
    }
}

pub struct Var {
    pub(crate) name: Token,
    pub(crate) init: Rc<dyn Expr>,
}

impl Statement for Var {
    fn evaluate(&self, env: Rc<Environment>) -> Result<Value, (String, Token)> {
        let val = self.init.evaluate(Rc::clone(&env))?;
        env.define(self.name.lexeme.clone(), val.clone());
        Ok(val.clone())
    }

    fn kind(&self) -> StatementKind {
        StatementKind::Var
    }
}

pub struct Block {
    pub(crate) statements: Vec<Rc<dyn Statement>>,
}

impl Statement for Block {
    fn evaluate(&self, env: Rc<Environment>) -> Result<Value, (String, Token)> {
        let scoped_environment = Rc::new(Environment::new_child(env.clone()));
        for statement in &self.statements {
            match statement.evaluate(Rc::clone(&scoped_environment))? {
                Value::Return(a) => {
                    return Ok(Value::Return(a.clone()));
                }
                _ => {}
            }
        }
        Ok(Value::None)
    }

    fn kind(&self) -> StatementKind {
        StatementKind::Block
    }
}

pub struct If {
    pub(crate) condition: Rc<dyn Expr>,
    pub(crate) then_branch: Rc<dyn Statement>,
    pub(crate) else_branch: Option<Rc<dyn Statement>>,
}

impl Statement for If {
    fn evaluate(&self, env: Rc<Environment>) -> Result<Value, (String, Token)> {
        match is_truth(self.condition.evaluate(Rc::clone(&env))?, false)? {
            Value::Bool(true) => self.then_branch.evaluate(Rc::clone(&env)),
            _ => match &self.else_branch {
                None => Ok(Value::None),
                Some(a) => a.evaluate(Rc::clone(&env)),
            },
        }
    }

    fn kind(&self) -> StatementKind {
        StatementKind::If
    }
}

pub struct While {
    pub(crate) condition: Rc<dyn Expr>,
    pub(crate) body: Rc<dyn Statement>,
}

impl Statement for While {
    fn evaluate(&self, env: Rc<Environment>) -> Result<Value, (String, Token)> {
        while is_truth(self.condition.evaluate(Rc::clone(&env))?, false)? == Value::Bool(true) {
            match self.body.evaluate(Rc::clone(&env))? {
                Value::Return(a) => {
                    return Ok(Value::Return(a.clone()));
                }
                Value::None => {}
                _ => {}
            }
        }
        Ok(Value::None)
    }

    fn kind(&self) -> StatementKind {
        StatementKind::While
    }
}

pub struct Function {
    pub(crate) name: Token,
    pub(crate) parameters: Vec<Token>,
    pub(crate) body: Vec<Rc<dyn Statement>>,
}

impl Statement for Function {
    fn evaluate(&self, env: Rc<Environment>) -> Result<Value, (String, Token)> {
        let borrow: &Environment = env.borrow();
        let environment_clone = Rc::new(borrow.clone());
        let cloned_body = self.body.clone();
        let cloned_parameters = self.parameters.clone();
        let function = Value::Function(Rc::new(Callable {
            arity: self.parameters.len(),
            function: Rc::new(move |arguments, environment| {
                for (i, parameter) in cloned_parameters.iter().enumerate() {
                    environment.define(
                        parameter.lexeme.clone(),
                        arguments.get(i).expect("error").clone(),
                    );
                }
                let mut interpreter = Interpreter::new_with_env(Rc::clone(&environment));
                interpreter.interpret(cloned_body.clone())
            }),
            string: format!("<fn {}>", self.name.lexeme),
            name: self.name.clone(),
            environment: Rc::clone(&environment_clone),
            is_initializer: RefCell::new(false),
        }));
        env.define(self.name.lexeme.clone(), function.clone());
        Ok(function)
    }

    fn kind(&self) -> StatementKind {
        StatementKind::Function(Function {
            name: self.name.clone(),
            parameters: self.parameters.clone(),
            body: self.body.clone(),
        })
    }
}

pub struct ReturnStatement {
    pub(crate) value: Rc<dyn Expr>,
}

impl Statement for ReturnStatement {
    fn evaluate(&self, env: Rc<Environment>) -> Result<Value, (String, Token)> {
        match self.value.kind() {
            Kind::NoOp => Ok(Value::Return(Box::new(Value::None))),
            _ => Ok(Value::Return(Box::new(self.value.evaluate(env)?))),
        }
    }

    fn kind(&self) -> StatementKind {
        StatementKind::ReturnStatement
    }
}

pub struct ClassStatement {
    pub(crate) name: Token,
    pub(crate) methods: Vec<Rc<dyn Statement>>,
    pub(crate) super_class: Option<Rc<dyn Expr>>,
}

impl Statement for ClassStatement {
    fn evaluate(&self, env: Rc<Environment>) -> Result<Value, (String, Token)> {
        let mut possible_super_class = None;
        match &self.super_class {
            None => {}
            Some(a) => {
                match a.kind() {
                    Kind::Variable(super_class) => {
                        if super_class.lexeme == self.name.lexeme {
                            return Err((String::from("Trida nemuze dedit sama sebe."), super_class));
                        }
                    }
                    _ => {}
                }

                match a.evaluate(Rc::clone(&env))? {
                    Value::Class(actual_super_class) => {
                        possible_super_class = Some(Rc::clone(&actual_super_class));
                    }
                    _ => { return Err((String::from("Super trida musi byt trida"), self.name.clone())); }
                }
            }
        }

        let mut methods: HashMap<String, Value> = HashMap::new();
        for method in &self.methods {
            match method.kind() {
                StatementKind::Function(function) => {
                    let thing = function.evaluate(Rc::clone(&env))?;
                    match thing {
                        Value::Function(callable) => {
                            if callable.name.lexeme == "init" {
                                callable.set_initializer();
                            }
                            match possible_super_class {
                                None => {}
                                Some(ref a) => {
                                    callable.bind_super(Value::Class(Rc::clone(&a)));
                                }
                            }
                            methods.insert(
                                function.name.lexeme.clone(),
                                Value::Function(Rc::clone(&callable)),
                            );
                        }
                        _ => {
                            methods.insert(function.name.lexeme.clone(), thing.clone());
                        }
                    }
                }
                _ => {}
            }
        }
        let class = Value::Class(Rc::new(Class {
            arity: 0,
            name: self.name.lexeme.clone(),
            methods: RefCell::new(methods),
            super_class: possible_super_class,
        }));
        env.define(self.name.lexeme.clone(), class);
        Ok(Value::None)
    }

    fn kind(&self) -> StatementKind {
        StatementKind::ClassStatement
    }
}
