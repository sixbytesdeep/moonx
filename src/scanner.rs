use crate::token::Token;
use crate::tokentype::TokenType;
use std::collections::HashMap;

pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
    keywords: HashMap<String, TokenType>,
}

impl Scanner {
    fn init_keywords(&mut self) {
        self.keywords.insert("and".to_string(), TokenType::And);
        self.keywords.insert("class".to_string(), TokenType::Class);
        self.keywords.insert("else".to_string(), TokenType::Else);
        self.keywords.insert("false".to_string(), TokenType::False);
        self.keywords.insert("for".to_string(), TokenType::For);
        self.keywords.insert("fun".to_string(), TokenType::Fun);
        self.keywords.insert("if".to_string(), TokenType::If);
        self.keywords.insert("null".to_string(), TokenType::Nil);
        self.keywords.insert("or".to_string(), TokenType::Or);
        self.keywords.insert("print".to_string(), TokenType::Print);
        self.keywords.insert("return".to_string(), TokenType::Return);
        self.keywords.insert("super".to_string(), TokenType::Super);
        self.keywords.insert("this".to_string(), TokenType::This);
        self.keywords.insert("true".to_string(), TokenType::True);
        self.keywords.insert("var".to_string(), TokenType::Var);
        self.keywords.insert("while".to_string(), TokenType::While);
    }

    pub fn new(&mut self, source: String) -> Self {
        Scanner {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
            keywords: HashMap::new(),
        }
    }

    pub fn scan_token(&mut self) -> Result<(), (u64, String)> {
        let c = self.advance();
        match c {
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '{' => self.add_token(TokenType::LeftBrace),
            '}' => self.add_token(TokenType::RightBrace),
            ',' => self.add_token(TokenType::Comma),
            '.' => self.add_token(TokenType::Dot),
            '-' => self.add_token(TokenType::Minus),
            '+' => self.add_token(TokenType::Plus),
            ';' => self.add_token(TokenType::SemiColon),
            '*' => self.add_token(TokenType::Bang),
            _ => todo!(),
        }
        Ok(())
    }

    pub fn advance(&mut self) -> char {
        let returned_char = self.source.chars().nth(self.current).unwrap();
        self.current = self.current + 1;
        returned_char
    }

    pub fn add_token(&mut self, token_type: TokenType) {
        self.add_token_final(token_type);
    }

    pub fn add_token_final(&mut self, token_type: TokenType) {
        let text = &self.source[self.start..self.current];
        self.tokens.push(Token {
            token_type,
            lexeme: String::from(text),
            line: self.line as u64
        });
    }
}

fn is_alphanumeric(c: char) -> bool {
    is_alpha(c) || is_digit(c)
}

fn is_alpha(c: char) -> bool {
    (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || c == '_'
}

fn is_digit(c: char) -> bool {
    c >= '0' && c <= '9'
}
