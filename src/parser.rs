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
            _ => Some(Precedence::LOWEST),
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

    pub fn get_current_token(&self) -> Option<Token<'a>> {
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
        let peek_token = self.get_peek_token();

        if let Some(tok) = peek_token {
            return Precedence::from_tok(tok.kind);
        }

        return None;
    }

    pub fn parse_program(&mut self) -> Program<'a> {
        let mut statements: Vec<Statement> = Vec::new();
        let current_pos = self.current_position;
        let length = self.tokens.len();

        while current_pos != length {
            let parsed_statement = self.parse_statement();

            if let Some(stmt) = parsed_statement {
                statements.push(stmt);
                self.consume_token();
            } else {
                break;
            }
        }

        return Program { statements };
    }
}

impl<'a> Parser<'a> {
    pub fn parse_integer_literal(&self) -> Box<Expression<'a>> {
        Box::new(Expression::IntegerLiteral {
            token: self.get_current_token().unwrap(),
        })
    }

    pub fn parse_float_literal(&self) -> Box<Expression<'a>> {
        Box::new(Expression::FloatLiteral {
            token: self.get_current_token().unwrap(),
        })
    }

    pub fn parse_string_literal(&self) -> Box<Expression<'a>> {
        Box::new(Expression::StringLiteral {
            token: self.get_current_token().unwrap(),
        })
    }
}

impl<'a> Parser<'a> {
    pub fn parse_statement(&mut self) -> Option<Statement<'a>> {
        let token = self.get_current_token();
        if let Some(to) = token {
            return match to.kind {
                TokenType::Import => self.parse_import_statement(),
                // TokenType::Ident(_) => self.parse_assignment_statement(),
                _ => self.parse_expression_statement(),
            };
        } else {
            return None;
        }
    }

    pub fn parse_import_statement(&mut self) -> Option<Statement<'a>> {
        let token = self.get_current_token().unwrap();

        let z = self.consume_token();
        let value = self.get_current_token().unwrap();
        Some(Statement::ImportStatement { token, value })
    }

    pub fn parse_expression_statement(&mut self) -> Option<Statement<'a>> {
        let token = self.get_current_token().unwrap();
        let expression = self.parse_expression(Precedence::LOWEST);
        if let Some(exp) = expression {
            return Some(Statement::ExpressionStatement {
                token,
                expression: exp,
            });
        }

        return None;
    }
}

impl<'a> Parser<'a> {
    pub fn parse_expression(&mut self, precedence: Precedence) -> Option<Box<Expression<'a>>> {
        let prefix_tok = self.get_current_token().unwrap().kind;
        println!("{:?}", prefix_tok);
        let prefix = self.prefix_fn(prefix_tok, false);
        // .unwrap_or_else(|| panic!("could not find prefix parser."));

        if (!prefix.0) {
            return None;
        }

        let mut prefixExp = self.prefix_fn(prefix_tok, true).1.unwrap();

        while precedence
            < self
                .get_peek_precedence()
                .unwrap_or_else(|| Precedence::LOWEST)
        {
            println!("peek token: {:?}", self.tokens[self.current_position + 1]);
            let z = self.get_peek_token().unwrap().kind;
            println!("peek token kind: {:?}", z);
            let (infix, z) = self.infix_fn(z, false, None);
            if (!infix) {
                return Some(prefixExp);
            }
            self.consume_token();
            prefixExp = self
                .infix_fn(
                    self.get_current_token().unwrap().kind,
                    true,
                    Some(prefixExp),
                )
                .1
                .unwrap();
        }
        // self.consume_token();

        return Some(prefixExp);
    }

    pub fn prefix_fn(
        &mut self,
        kind: TokenType<'a>,
        execute: bool,
    ) -> (bool, Option<Box<Expression<'a>>>) {
        match (kind, execute) {
            (TokenType::Integer(_), false) => (true, None),
            (TokenType::Integer(_), true) => (true, Some(self.parse_integer_literal())),
            (TokenType::Float(_), false) => (true, None),
            (TokenType::Float(_), true) => (true, Some(self.parse_float_literal())),
            (TokenType::String(_), false) => (true, None),
            (TokenType::String(_), true) => (true, Some(self.parse_string_literal())),
            _ => (false, None),
        }
    }

    pub fn infix_fn(
        &mut self,
        kind: TokenType,
        execute: bool,
        left: Option<Box<Expression<'a>>>,
    ) -> (bool, Option<Box<Expression<'a>>>) {
        match (kind, execute) {
            (TokenType::Plus, false) => (true, None),
            (TokenType::Plus, true) => (true, Some(self.parse_infix_expression(left.unwrap()))),
            (TokenType::Minus, false) => (true, None),
            (TokenType::Minus, true) => (true, Some(self.parse_infix_expression(left.unwrap()))),
            (TokenType::Slash, false) => (true, None),
            (TokenType::Slash, true) => (true, Some(self.parse_infix_expression(left.unwrap()))),
            (TokenType::Asterisk, false) => (true, None),
            (TokenType::Asterisk, true) => (true, Some(self.parse_infix_expression(left.unwrap()))),
            (TokenType::Minus, false) => (true, None),
            (TokenType::Minus, true) => (true, Some(self.parse_infix_expression(left.unwrap()))),
            (TokenType::PlusEqual, false) => (true, None),
            (TokenType::PlusEqual, true) => {
                (true, Some(self.parse_infix_expression(left.unwrap())))
            }
            (TokenType::AsteriskEqual, false) => (true, None),
            (TokenType::AsteriskEqual, true) => {
                (true, Some(self.parse_infix_expression(left.unwrap())))
            }
            (TokenType::SlashEqual, false) => (true, None),
            (TokenType::SlashEqual, true) => {
                (true, Some(self.parse_infix_expression(left.unwrap())))
            }
            (TokenType::DoubleEqual, false) => (true, None),
            (TokenType::DoubleEqual, true) => {
                (true, Some(self.parse_infix_expression(left.unwrap())))
            }
            (TokenType::BangEqual, false) => (true, None),
            (TokenType::BangEqual, true) => {
                (true, Some(self.parse_infix_expression(left.unwrap())))
            }
            (TokenType::LT, false) => (true, None),
            (TokenType::LT, true) => (true, Some(self.parse_infix_expression(left.unwrap()))),
            (TokenType::GT, false) => (true, None),
            (TokenType::GT, true) => (true, Some(self.parse_infix_expression(left.unwrap()))),
            (TokenType::Or, false) => (true, None),
            (TokenType::Or, true) => (true, Some(self.parse_infix_expression(left.unwrap()))),
            (TokenType::And, false) => (true, None),
            (TokenType::And, true) => (true, Some(self.parse_infix_expression(left.unwrap()))),
            (TokenType::LTEq, false) => (true, None),
            (TokenType::LTEq, true) => (true, Some(self.parse_infix_expression(left.unwrap()))),
            (TokenType::GTEq, false) => (true, None),
            (TokenType::GTEq, true) => (true, Some(self.parse_infix_expression(left.unwrap()))),
            _ => (false, None),
        }
    }
}

impl<'a> Parser<'a> {
    pub fn parse_infix_expression(&mut self, left: Box<Expression<'a>>) -> Box<Expression<'a>> {
        let current = self.get_current_token().unwrap();
        let precedence = self.get_current_precedence().unwrap();
        self.consume_token();
        let right = self.parse_expression(precedence).unwrap();
        return Box::new(Expression::InfixExpression {
            token: current,
            right,
            left,
        });
    }
}

#[cfg(test)]
mod tests {
    use crate::lexer::lexer::Lexer;
    use crate::parser::Parser;
    #[test]
    fn it_works() {
        let test_str = r#"2 / 5.5"#;
        let l = Lexer::new(test_str);
        let z = l.collect::<Vec<_>>();
        println!("{:#?}", z);
        let mut p = Parser::new(z);
        println!("{:#?}", p.parse_program());
    }
}
