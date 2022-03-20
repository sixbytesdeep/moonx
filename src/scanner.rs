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
}
