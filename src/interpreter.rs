use crate::moonenv::Environment;
use crate::value::{Callable, Value};
use crate::statements::Statement;
use crate::token::Token;
use crate::tokentype::TokenType;
use std::cell::RefCell;
use std::rc::Rc;
use std::time::{SystemTime, UNIX_EPOCH};

pub struct Interpreter {
    envi: Rc<Environment>
}

impl Interpreter {
    pub fn new() -> Self {
        let env = Rc::new(Environment::new());
        let callable = Callable {
            arity: 0,
            function: Rc::new(|_arguments, _env| {
                Ok(Value::Number(
                        SystemTime::now()
                            .duration_since(UNIX_EPOCH)
                            .expect("Cas jde pozpatku.")
                            .as_secs_f64(),
                    ))
            }),
            string: "<native fn>".to_string(),
            name: Token {
                token_type: TokenType::Identifier, 
                lexeme: "clock".to_string(),
                literal: Value::None,
                line: 0,
            },
            environment: Rc::clone(&env),
            is_initializer: RefCell::new(false),
        };
        env.define(String::from("clock"), Value::Function(Rc::new(callable)));
        Interpreter {envi: env}
    }

    pub fn new_with_env(environment: Rc<Environment>) -> Self {
        Interpreter {
            envi: Rc::clone(&environment),
        }
    }

    pub fn interpret(
        &mut self,
        statements: Vec<Rc<dyn Statement>>,
        ) -> Result<Value, (String, Token)> {
        for statement in statements {
            match statement.evaluate(Rc::clone(&self.envi)) {
                Ok(Value::Return(value)) => {
                    return Ok(*value);
                }

                Ok(_) => {}
                Err((msg, token)) => return Err((String::from(msg), token.clone())),
            }
        }
        Ok(Value::None)
    }
}
