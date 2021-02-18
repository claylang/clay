use crate::{
    ast::{Expression, Program, Statement},
    lexer::token::{self, Token, TokenType},
};

#[derive(Debug, PartialOrd, PartialEq)]
enum Precedence {
    LOWEST = 0,
    AND = 1,
    OR = 2,
    EQUALS = 3,
    LESSGREATER = 4,
    SUM = 5,
    PRODUCT = 6,
    PREFIX = 7,
    CALL = 8,
    INDEX = 9,
}

impl Precedence {
    fn from_tok(token: TokenType) -> Option<Precedence> {
        match token {
            TokenType::Equal => Some(Precedence::EQUALS),
            TokenType::BangEqual => Some(Precedence::EQUALS),
            TokenType::LT => Some(Precedence::LESSGREATER),
            TokenType::GT => Some(Precedence::LESSGREATER),
            TokenType::LTEq => Some(Precedence::LESSGREATER),
            TokenType::GTEq => Some(Precedence::LESSGREATER),
            TokenType::Or => Some(Precedence::OR),
            TokenType::And => Some(Precedence::AND),
            TokenType::Plus => Some(Precedence::SUM),
            TokenType::Minus => Some(Precedence::SUM),
            TokenType::Asterisk => Some(Precedence::PRODUCT),
            TokenType::Slash => Some(Precedence::PRODUCT),
            TokenType::LParen => Some(Precedence::CALL),
            TokenType::LBracket => Some(Precedence::INDEX),
            _ => None,
        }
    }
}

struct Parser<'a> {
    tokens: Vec<Token<'a>>,
    current_position: usize,
}

impl<'a, 'b> Parser<'a> {
    pub fn new(tokens: Vec<Token<'a>>) -> Parser {
        Parser {
            tokens,
            current_position: 0,
        }
    }

    pub fn consume_token(&mut self) {
        self.current_position += 1;
    }

    pub fn get_current_token(&mut self) -> Option<Token<'a>> {
        match self.tokens.get(self.current_position) {
            Some(z) => Some(*z),
            _ => None,
        }
    }

    pub fn get_peek_token(&mut self) -> Option<Token<'a>> {
        match self.tokens.get(self.current_position + 1) {
            Some(z) => Some(*z),
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

    pub fn peek_is(&mut self, kind: TokenType) -> bool {
        if let Some(peek) = self.get_peek_token() {
            return kind == peek.kind;
        } else {
            panic!("Out of index for peek token.")
        }
    }

    pub fn get_current_precedence(&mut self) -> Option<Precedence> {
        let current_tok = self.get_current_token().unwrap();

        return Precedence::from_tok(current_tok.kind);
    }

    pub fn get_peek_precedence(&mut self) -> Option<Precedence> {
        let peek_token = self.get_peek_token().unwrap();

        return Precedence::from_tok(peek_token.kind);
    }

    pub fn parse_program(&mut self) -> Program {
        let mut statements: Vec<Statement> = Vec::new();

        while self.current_position != self.tokens.len() {
            statements.push(self.parse_statement());

            self.consume_token();
        }
        return Program { statements };
    }
}

impl<'a> Parser<'a> {
    pub fn parse_integer_literal(&mut self) -> Expression<'a> {
        Expression::IntegerLiteral {
            token: self.get_current_token().unwrap(),
        }
    }

    // pub fn parse_string_literal(&self) -> &Expression {
    //     &Expression::StringLiteral {
    //         token: self.get_current_token().unwrap(),
    //     }
    // }
}

impl<'a> Parser<'a> {
    pub fn parse_statement(&mut self) -> Statement {
        match self.get_current_token().unwrap().kind {
            TokenType::Import => self.parse_import_statement(),
            // TokenType::Ident(_) => self.parse_assignment_statement(),
            _ => self.parse_expression_statement(),
        }
    }

    pub fn parse_import_statement(&mut self) -> Statement {
        let token = self.get_current_token().unwrap();

        let z = self.consume_token();
        let value = self.get_current_token().unwrap();
        Statement::ImportStatement { token, value }
    }

    pub fn parse_expression_statement(&mut self) -> Statement {
        let token = self.get_current_token().unwrap();
        let expression = self.parse_expression(Precedence::LOWEST);
        Statement::ExpressionStatement { token, expression }
    }
}

impl<'a> Parser<'a> {
    pub fn parse_expression(&mut self, precedence: Precedence) -> Box<Expression> {
        let prefix_tok = self.get_current_token().unwrap().kind;
        let prefix = self.find_prefix_fn(prefix_tok);
        // .unwrap_or_else(|| panic!("could not find prefix parser."));

        if (!prefix) {
            panic!("no prefix parse function found.");
        }

        let mut prefixExp = self.call_prefix_fn(prefix_tok);

        while precedence < self.get_current_precedence().unwrap() {
            let infix = self.find_infix_fn(self.get_peek_token().unwrap().kind);
            if (!infix) {
                return prefixExp;
            }
            self.consume_token();
            prefixExp = self.call_infix_fn(self.tokens[self.current_position + 1].kind, prefixExp);
        }
        return prefixExp;
    }

    pub fn find_prefix_fn(&mut self, kind: TokenType<'a>) -> bool {
        match kind {
            TokenType::Integer(_) => true,
            _ => panic!("No prefix parse function found."),
        }
    }

    pub fn call_prefix_fn(&mut self, kind: TokenType<'a>) -> Box<Expression> {
        match kind {
            TokenType::Integer(_) => Box::new(self.parse_integer_literal()),
            _ => panic!("No prefix parse fucntion found."),
        }
    }

    pub fn find_infix_fn(&mut self, kind: TokenType) -> bool {
        match kind {
            // TokenType::Integer(_) => Some(Box::new(|| Box::new(self.parse_integer_literal()))),
            _ => panic!("No infix parse function found."),
        }
    }

    pub fn call_infix_fn(
        &mut self,
        kind: TokenType<'a>,
        expression: Box<Expression>,
    ) -> Box<Expression> {
        match kind {
            // TokenType::Integer(_) => Box::new(self.parse_integer_literal()),
            _ => panic!("No infix parse fucntion found."),
        }
    }
}
