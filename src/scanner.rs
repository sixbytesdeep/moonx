use crate::token::Token;
use crate::tokentype::TokenType;
use crate::value::Value;
use phf::phf_map;

pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

static KEYWORDS: phf::Map<&'static str, TokenType> = phf_map! {
"and" => TokenType::And,
"class" => TokenType::Class,
"else" => TokenType::Else,
"false" => TokenType::False,
"for" => TokenType::For,
"fun" => TokenType::Fun,
"if" => TokenType::If,
"nil" => TokenType::Nil,
"or" => TokenType::Or,
"print" => TokenType::Print,
"return" => TokenType::Return,
"super" => TokenType::Super,
"this" => TokenType::This,
"true" => TokenType::True,
"var" => TokenType::Var,
"while" => TokenType::While,
};

impl Scanner {
    pub fn new(source: String) -> Self {
        Scanner {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(&mut self) -> Result<Vec<Token>, (u64, String)> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token()?;
        }

        self.tokens.push(Token {
            token_type: TokenType::EOF,
            lexeme: "".to_string(),
            literal: Value::None,
            line: self.line as u64,
        });
        Ok(self.tokens.to_vec())
    }

    fn scan_token(&mut self) -> Result<(), (u64, String)> {
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
            '!' => {
                let following = self.match_char('=');
                self.add_token(if following {
                    TokenType::BangEqual
                } else {
                    TokenType::Bang
                });
            }
            '=' => {
                let following = self.match_char('=');
                self.add_token(if following {
                    TokenType::EqualEqual
                } else {
                    TokenType::Equal
                });
            }
            '<' => {
                let following = self.match_char('=');
                self.add_token(if following {
                    TokenType::LessEqual 
                } else {
                    TokenType::Less 
                });
            }
            '>' => {
                let following = self.match_char('=');
                self.add_token(if following {
                    TokenType::GreaterEqual 
                } else {
                    TokenType::Greater 
                });
            }
            '/' => {
                let following = self.match_char('/');
                if following {
                    let mut next = self.peek();
                    while next != '\n' && !self.is_at_end() {
                        self.advance();
                        next = self.peek();
                    }
                } else {
                    self.add_token(TokenType::Slash);
                }
            }
            ' ' | '\r' | '\t' => (),
            '\n' => self.line = self.line + 1,
            '"' => self.string()?,
            ch => {
                if is_digit(ch) {
                    self.number();
                } else if is_alpha(ch) {
                    self.identifier();
                } else {
                    return Err((self.line as u64, String::from("unexpected character.")));
                }
            }
        }
        Ok(())
    }

    fn string(&mut self) -> Result<(), (u64, String)> {
        let mut peeked = self.peek();
        while peeked != '"' && !self.is_at_end() {
            if peeked == '\n' {
                self.line += 1;
            }
            self.advance();
            peeked = self.peek();
        }

        if self.is_at_end() {
            return Err((self.line as u64, String::from("Unterminated string.")));
        }

        self.advance();

        let value: String = String::from(&self.source[self.start + 1 .. self.current -1]);
        self.add_token_final(TokenType::String, Value::String(value));
        Ok(())
    }
    fn number(&mut self) {
        while is_digit(self.peek()) {
            self.advance();
        }

        if self.peek() == '.' && is_digit(self.peek_next()) {
            self.advance();
            while is_digit(self.peek()) {
                self.advance();
            }
        }
        let number_string = &self.source[self.start..self.current];
        let number: f64 = number_string.parse().unwrap();
        self.add_token_final(TokenType::Number, Value::Number(number));
    }
    fn identifier(&mut self) {
        while is_alphanumeric(self.peek()) {
            self.advance();
        }

        let text = &self.source[self.start..self.current];
        match KEYWORDS.get(text) {
            None => self.add_token(TokenType::Identifier),
            Some(tokentype) => self.add_token(tokentype.clone())
        }
    }

    fn advance(&mut self) -> char {
        let returned_char = self.source.chars().nth(self.current).unwrap();
        self.current = self.current + 1;
        returned_char
    }

    fn add_token(&mut self, token_type: TokenType) {
        self.add_token_final(token_type, Value::None);
    }

    fn add_token_final(&mut self, token_type: TokenType, literal: Value) {
        let text = &self.source[self.start..self.current];
        self.tokens.push(Token {
            token_type,
            lexeme: String::from(text),
            literal,
            line: self.line as u64
        });
    }

    fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        if self.source.chars().nth(self.current).unwrap() != expected {
            return false;
        }
        self.current += 1;
        return true;
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        self.source.chars().nth(self.current).unwrap()
    }

    fn peek_next(&self) -> char {
        if self.current + 1 > self.source.len() {
            return '\0';
        }
        self.source.chars().nth(self.current + 1).unwrap()
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
