use crate::{
    ast::{Expression, Program, Statement},
    lexer::token::{Token, TokenType},
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

enum IdentTypes {
    Destructuring,
    Normal,
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
                panic!("Expected token");
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

    pub fn parse_ident_literals(&mut self) -> (Vec<Token<'a>>, IdentTypes) {
        let mut kind = IdentTypes::Destructuring;

        let mut idents: Vec<Token<'a>> = Vec::new();

        while let Some(peek) = self.get_peek_token() {
            match peek.kind {
                TokenType::Period => {
                    idents.push(self.get_current_token().unwrap());
                    kind = IdentTypes::Normal;
                    self.consume_token();
                    if let Some(peek) = self.get_peek_token() {
                        match peek.kind {
                            TokenType::Ident(_) => {
                                self.consume_token();
                                idents.push(self.get_current_token().unwrap());
                            }
                            _ => panic!("Expected IDEnt after period."),
                        }
                    } else {
                        panic!("Expected IDEnt after period.");
                    }
                }
                TokenType::Comma => {
                    idents.push(self.get_current_token().unwrap());
                    kind = IdentTypes::Destructuring;
                    self.consume_token();
                    if let Some(peek) = self.get_peek_token() {
                        match peek.kind {
                            TokenType::Ident(_) => {
                                self.consume_token();
                                idents.push(self.get_current_token().unwrap());
                            }
                            _ => panic!("Expected IDEnt after comma."),
                        }
                    } else {
                        panic!("Expected IDEnt after comma.");
                    }
                }
                _ => {
                    // idents.push(self.get_current_token().unwrap());
                    self.consume_token();

                    return (idents, kind);
                }
            }
        }

        (idents, kind)
    }
}

impl<'a> Parser<'a> {
    pub fn parse_statement(&mut self) -> Option<Statement<'a>> {
        let token = self.get_current_token();
        if let Some(to) = token {
            return match to.kind {
                TokenType::Import => self.parse_import_statement(),
                TokenType::Ident(_) => self.parse_identifier_statement(),
                // TokenType::Ident(_) => self.parse_assignment_statement(),
                _ => self.parse_expression_statement(),
            };
        } else {
            return None;
        }
    }

    pub fn parse_import_statement(&mut self) -> Option<Statement<'a>> {
        let token = self.get_current_token().unwrap();

        self.consume_token();
        let ident = self.get_current_token().unwrap();
        Some(Statement::ImportStatement {
            token,
            value: ident,
        })
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

    pub fn parse_identifier_statement(&mut self) -> Option<Statement<'a>> {
        let mut kind = IdentTypes::Destructuring;
        let (idents, _) = self.parse_ident_literals();

        match self.get_current_token().unwrap().kind {
            TokenType::ColonEqual => {
                kind = IdentTypes::Destructuring;
            }
            TokenType::Equal => {
                kind = IdentTypes::Normal;
            }
            _ => {
                panic!("Expected token of kind ':=' or '=' in assignment statement.");
            }
        }

        let token = self.get_current_token().unwrap();

        self.consume_token();
        let expr = self.parse_expression(Precedence::LOWEST);

        if let Some(expression) = expr {
            match kind {
                IdentTypes::Destructuring => {
                    return Some(Statement::AssignStatement {
                        token,
                        expression,
                        defined: Box::new(Expression::DefinitionIdentifier { idents }),
                    })
                }
                IdentTypes::Normal => {
                    return Some(Statement::UpdateStatement {
                        token,
                        expression,
                        ident: Box::new(Expression::NormalIdentifier { idents }),
                    })
                }
            }
        } else {
            panic!("Expected expression after assignment operator.")
        }
    }
}

impl<'a> Parser<'a> {
    pub fn parse_expression(&mut self, precedence: Precedence) -> Option<Box<Expression<'a>>> {
        let prefix_tok = self.get_current_token().unwrap().kind;
        println!("{:?}", prefix_tok);
        let prefix = self.prefix_fn(prefix_tok, false);

        if !prefix.0 {
            return None;
        }

        let mut prefix_exp = self.prefix_fn(prefix_tok, true).1.unwrap();

        while precedence
            < self
                .get_peek_precedence()
                .unwrap_or_else(|| Precedence::LOWEST)
        {
            let z = self.get_peek_token().unwrap().kind;

            let (infix_exists, _) = self.infix_fn(z, false, None);

            if !infix_exists {
                return Some(prefix_exp);
            }

            self.consume_token();
            prefix_exp = self
                .infix_fn(
                    self.get_current_token().unwrap().kind,
                    true,
                    Some(prefix_exp),
                )
                .1
                .unwrap();
        }

        return Some(prefix_exp);
    }

    // pub fn

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
            (TokenType::LParen, false) => (true, None),
            (TokenType::LParen, true) => (true, Some(self.parse_grouped_expression())),
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

    pub fn parse_grouped_expression(&mut self) -> Box<Expression<'a>> {
        self.consume_token();

        let expr = self.parse_expression(Precedence::LOWEST);

        if let Some(expr) = expr {
            self.expect_peek(TokenType::RParen);
            return expr;
        } else {
            panic!("Expected some expression after opening bracket.");
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::lexer::lexer::Lexer;
    use crate::parser::Parser;
    #[test]
    fn full_test() {
        let test_str = r#"
        import z
        2 + 2 * 3
        "#;
        let l = Lexer::new(test_str);
        let z = l.collect::<Vec<_>>();
        println!("{:#?}", z);
        let mut p = Parser::new(z);
        println!("{:#?}", p.parse_program());
    }

    #[test]
    fn ident_assign() {
        let test_str = r#"
        x, z := 3 + 2
        m.e = "3"
        "#;

        println!(
            "{:#?}",
            Parser::new(Lexer::new(test_str).collect::<Vec<_>>()).parse_program()
        );
    }
}
