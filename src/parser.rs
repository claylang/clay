use crate::lexer::token::{Token, TokenType};

struct Parser<'a> {
    tokens: Vec<Token<'a>>,
    current_position: usize,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: Vec<Token<'a>>) -> Parser {
        Parser {
            tokens,
            current_position: 0,
        }
    }

    pub fn consume_token(&mut self) {
        self.current_position += 1;
    }

    pub fn get_current_token(&self) -> Option<&Token> {
        match self.tokens.get(self.current_position) {
            Some(z) => Some(z),
            _ => None,
        }
    }

    pub fn get_peek_token(&self) -> Option<&Token> {
        match self.tokens.get(self.current_position + 1) {
            Some(z) => Some(z),
            _ => None,
        }
    }

    pub fn expect_peek(&mut self, kind: TokenType) -> bool {
        if let Some(peek) = self.get_peek_token() {
            if peek.kind == kind {
                self.consume_token();
                return true;
            } else {
                return false;
            }
        } else {
            panic!("Out of index for peek token.")
        }
    }

    pub fn peek_is(&self, kind: TokenType) -> bool {
        if let Some(peek) = self.get_peek_token() {
            return kind == peek.kind;
        } else {
            panic!("Out of index for peek token.")
        }
    }
}
