use crate::expressions::*;
use crate::value::Value;
use crate::statements::*;
use crate::token::Token;
use crate::tokentype::TokenType;
use std::rc::Rc;

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
    in_a_class: bool,
    in_an_init: bool,
    in_a_subclass: bool,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens,
            current: 0,
            in_a_class: false,
            in_an_init: false,
            in_a_subclass: false,
        }
    }

    pub(crate) fn parse(&mut self) -> (Vec<Rc<dyn Statement>>, Vec<(Token, String)>) {
        let mut statements: Vec<Rc<dyn Statement>> = Vec::new();
        let mut errors: Vec<(Token, String)> = Vec::new();
        while !self.is_at_end() {
            match self.declaration() {
                Ok(statement) => statements.push(statement),
                Err((msg, token)) => errors.push((token.clone(), msg)),
            }
        }
        (statements, errors)
    }

    fn matching(&mut self, types: &[TokenType]) -> bool {
        for ttype in types {
            if self.check(ttype.clone()) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn consume(&mut self, ttype: TokenType, msg: String) -> Result<&Token, (String, Token)> {
        if self.check(ttype) {
            Ok(self.advance())
        } else {
            Err((msg, self.peek().clone()))
        }
    }

    fn check(&self, ttype: TokenType) -> bool {
        !self.is_at_end() && (self.peek().token_type == ttype)
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current = self.current + 1;
        }

        self.previous()
    }

    fn is_at_end(&self) -> bool {
        self.peek().token_type == TokenType::EOF
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }
    
    fn synchronize(&mut self) {
        self.advance();
        while !self.is_at_end() {
            if self.previous().token_type == TokenType::SemiColon {
                return;
            }
            
            match self.peek().token_type {
                TokenType::Class
                | TokenType::For
                | TokenType::Fun
                | TokenType::If
                | TokenType::Print
                | TokenType::Return
                | TokenType::Var
                | TokenType::While => return,
                _ => {}
            }
            
            self.advance();
        }
    }
    
    fn expression(&mut self) -> Result<Rc<dyn Statement>, (String, Token)> {
        self.assignment()
    }

    fn declaration(&mut self) -> Result<Rc<dyn Statement>, (String, Token)> {
        if self.matching(&[TokenType::Class]) {
            self.class_declaration()
        } else if self.matching(&[TokenType::Fun]) {
            self.fun_declaration("function")
        } else if self.matching(&[TokenType::Var]) {
            self.var_declaration()
        } else {
            let statement = self.statement();
            match statement {
                Ok(_) => statement,
                Err(err) => {
                    self.synchronize();
                    Err(err)
                }
            }
        }
    }

    fn class_declaration(&mut self) -> Result<Rc<dyn Statement>, (String, Token)> {
        self.in_a_class = true;
        let name = self
            .consume(TokenType::Identifier, String::from("Ocekavam nazev tridy."))?
            .clone();

        let mut super_class: Option<Rc<dyn Expr>> = None;
        if self.matching(&[TokenType::Less]) {
            self.in_a_subclass = true;
            self.consume(
                TokenType::Identifier,
                String::from("Ocekavam nazev supertridy."),
            )?;
            super_class = Some(Rc::new(Variable {
                name: self.previous().clone(),
            }));
        }

        self.consume(
            TokenType::LeftBrace,
            String::from("Ocekavam '{' pred zacatkem tela tridy."),
        )?;
        let mut methods: Vec<Rc<dyn Statement>> = Vec::new();

        while !self.check(TokenType::RightBrace) && !self.is_at_end() {
            methods.push(self.function("method")?);
        }

        self.consume(
            TokenType::RightBrace,
            String::from("Ocekavam '}' po tele tridy."),
        )?;

        self.in_a_class = false;
        self.in_a_subclass = false;

        Ok(Rc::new(ClassStatement {
            name,
            methods,
            super_class,
        }))
    }

    fn statement(&mut self) -> Result<Rc<dyn Statement>, (String, Token)> {
        if self.matching(&[TokenType::For]) {
            return self.for_statement();
        }
        if self.matching(&[TokenType::If]) {
            return self.if_statement();
        }
        if self.matching(&[TokenType::Print]) {
            return self.print_statement();
        }
        if self.matching(&[TokenType::Return]) {
            return self.return_statement();
        }
        if self.matching(&[TokenType::While]) {
            return self.while_statement();
        }
        if self.matching(&[TokenType::LeftBrace]) {
            let statements = self.block()?;
            return Ok(Rc::new(Block { statements }));
        }
        self.expression_statement()
    }

    fn if_statement(&mut self) -> Result<Rc<dyn Statement>, (String, Token)> {
        self.consume(TokenType::LeftParen, String::from("Ocekavam '(' po 'if'."))?;
        let condition = self.expression()?;
        self.consume(
            TokenType::RightParen,
            String::from("Ocekavam ')' na konci podminky."),
        )?;

        let then_branch = self.statement()?;
        let mut else_branch = None;

        if self.matching(&[TokenType::Else]) {
            else_branch = Some(self.statement()?);
        }

        Ok(Rc::new(If { condition, then_branch, else_branch }))
    }

}
