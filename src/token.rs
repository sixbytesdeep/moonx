use std::fmt;

use crate::tokentype::TokenType;

#[derive(Clone)]
pub struct Token {
    pub(crate) token_type: TokenType,
    pub(crate) lexeme: String,
    pub(crate) line: u64,
}

impl fmt::Debug for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Token")
            .field("type", &self.token_type)
            .field("lexeme", &self.lexeme)
            .field("line", &self.line)
            .finish()
    }
}
