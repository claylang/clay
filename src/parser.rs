use core::panic;
use std::collections::HashMap;

use crate::{
    ast::{Expression, MatchPairExpression, Program, Statement},
    errors::error,
    token::{self, Token, TokenType},
};

#[derive(Debug, PartialOrd, PartialEq)]
pub enum Precedence {
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
    MATCH = 10,
}

pub enum IdentTypes {
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
            TokenType::Match => Some(Precedence::MATCH),
            _ => Some(Precedence::LOWEST),
        }
    }
}

pub struct Parser<'a> {
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

    pub fn get_peek_token(&self) -> Option<Token<'a>> {
        match self.tokens.get(self.current_position + 1) {
            Some(z) => Some(*z),
            _ => None,
        }
    }

    pub fn expect_peek(&mut self, kind: TokenType, reason: &str) -> bool {
        let current_tok = self.get_current_token().unwrap();
        if let Some(peek) = self.get_peek_token() {
            if peek.kind == kind {
                self.consume_token();
                return true;
            } else {
                error(format!("{} Expected token of type '{:?}' to be after token of type '{:?}' when {}, received '{:?}' instead.", peek.position, kind, current_tok.kind, reason, peek.kind));
                return false;
            }
        } else {
            error(format!(
                "{} Out of index for peek token.",
                current_tok.position
            ));
            return false;
        }
    }

    pub fn peek_is(&mut self, kind: TokenType) -> bool {
        if let Some(peek) = self.get_peek_token() {
            return kind == peek.kind;
        } else {
            error(format!("Out of index for peek token."));
            return false;
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

    pub fn parse_underscore_literal(&self) -> Box<Expression<'a>> {
        Box::new(Expression::UnderscoreLiteral {
            token: self.get_current_token().unwrap(),
        })
    }

    pub fn parse_ident_literal(&mut self, already: Option<Vec<Token<'a>>>) -> Box<Expression<'a>> {
        let mut idents: Vec<Token<'a>> = Vec::new();
        if let Some(z) = already {
            // self.consume_token();
            return Box::new(Expression::NormalIdentifier { idents: z });
        }
        idents.push(self.get_current_token().unwrap());

        while let Some(peek) = self.get_peek_token() {
            match peek.kind {
                TokenType::Period => {
                    self.consume_token();
                    if let Some(peek) = self.get_peek_token() {
                        match peek.kind {
                            TokenType::Ident(_) => {
                                self.consume_token();
                                idents.push(self.get_current_token().unwrap());
                            }
                            _ => error(format!("Expected Ident after period.")),
                        }
                    } else {
                        error(format!("Expected Ident after period."));
                    }
                }
                _ => {
                    break;
                }
            }
        }

        return Box::new(Expression::NormalIdentifier { idents });
    }

    pub fn parse_ident_literals(&mut self) -> (Vec<Token<'a>>, IdentTypes) {
        let mut kind = IdentTypes::Destructuring;

        let mut idents: Vec<Token<'a>> = Vec::new();
        if let Some(t) = self.get_current_token() {
            if let TokenType::Ident(_) = t.kind {
                idents.push(self.get_current_token().unwrap());
            }
        }

        while let Some(peek) = self.get_peek_token() {
            match peek.kind {
                TokenType::Period => {
                    // idents.push(self.get_current_token().unwrap());
                    kind = IdentTypes::Normal;
                    self.consume_token();
                    if let Some(peek) = self.get_peek_token() {
                        match peek.kind {
                            TokenType::Ident(_) => {
                                self.consume_token();
                                idents.push(self.get_current_token().unwrap());
                            }
                            _ => error(format!("Expected IDEnt after period.")),
                        }
                    } else {
                        error(format!("Expected IDEnt after period."));
                    }
                }
                TokenType::Comma => {
                    // idents.push(self.get_current_token().unwrap());
                    kind = IdentTypes::Destructuring;
                    self.consume_token();
                    if let Some(peek) = self.get_peek_token() {
                        match peek.kind {
                            TokenType::Ident(_) => {
                                self.consume_token();
                                idents.push(self.get_current_token().unwrap());
                            }
                            _ => error(format!("Expected IDEnt after comma.")),
                        }
                    } else {
                        error(format!("Expected IDEnt after comma."));
                    }
                }
                _ => {
                    self.consume_token();

                    return (idents, kind);
                }
            }
        }

        (idents, kind)
    }

    pub fn parse_function_expression(&mut self) -> Box<Expression<'a>> {
        let token = self.get_current_token().unwrap();
        self.consume_token();
        let (idents, _) = self.parse_ident_literals();

        if idents.len() != 0 && self.get_current_token().unwrap().kind != TokenType::Bar {
            error(format!("{} Parameter declarations in a function definition must be followed by a '|', received {:?} instead.", self.get_current_token().unwrap().position, self.get_current_token().unwrap().kind))
        }

        if self.get_current_token().unwrap().kind == TokenType::Bar {
            self.consume_token();
        }

        if let Some(tok) = self.get_current_token() {
            if tok.kind != TokenType::Arrow {
                error(format!("{} Expected '->' to follow function parameter declarations, received {:?} instead. Make sure your bar pair is followed by an arrow.", tok.position, tok.kind));
            }
        } else {
            error(format!("{} A function needs to have a body, but I couldn't find any after the declaration.", self.tokens[self.current_position-1].position));
        }

        self.consume_token();

        let current_tok = self.get_current_token();
        if let Some(tok) = current_tok {
            match tok.kind {
                TokenType::LBrace => {
                    let statements = self.parse_block_statement(TokenType::RBrace);
                    // self.consume_token();
                    return Box::new(Expression::FunctionLiteral {
                        token,
                        parameters: Box::new(Expression::DefinitionIdentifier { idents }),
                        statements,
                    });
                }
                _ => {
                    let expr = self.parse_expression(Precedence::LOWEST, None).unwrap_or_else(|| {error(format!("{} When '{{' is not provided in the function body, Clay expects a single expression.", tok.position));
                panic!()});
                    let stmt = Statement::ReturnStatement {
                        token: tok,
                        value: expr,
                    };
                    let mut stmts: Vec<Statement<'a>> = Vec::new();
                    stmts.push(stmt);

                    self.consume_token();
                    return Box::new(Expression::FunctionLiteral {
                        token,
                        parameters: Box::new(Expression::DefinitionIdentifier { idents }),
                        statements: Statement::BlockStatement {
                            token: tok,
                            statements: stmts,
                        },
                    });
                }
            }
        } else {
            error(format!("{} Expected to see a function body after parameter declarations, but received nothing.", self.tokens[self.current_position-1]));
            panic!();
        }
    }

    pub fn parse_array_literal(&mut self) -> Box<Expression<'a>> {
        let token = self.get_current_token().unwrap();
        let expressions = self.parse_expression_list(TokenType::RBracket);

        return Box::new(Expression::ArrayLiteral {
            token,
            elements: expressions,
        });
    }
}

impl<'a> Parser<'a> {
    pub fn parse_statement(&mut self) -> Option<Statement<'a>> {
        let token = self.get_current_token();
        if let Some(to) = token {
            return match to.kind {
                TokenType::Import => self.parse_import_statement(),
                TokenType::Ident(_) => self.parse_identifier_statement(),
                TokenType::Return => self.parse_return_statement(),
                _ => self.parse_expression_statement(None),
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

    pub fn parse_expression_statement(
        &mut self,
        idents: Option<Vec<Token<'a>>>,
    ) -> Option<Statement<'a>> {
        let token = self.get_current_token().unwrap();
        let expression = self.parse_expression(Precedence::LOWEST, idents);
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
        if let Some(token) = self.get_current_token() {
            match token.kind {
                TokenType::Ident(_) => {}
                _ => self.current_position -= 1,
            }
        }

        if let Some(token) = self.get_peek_token() {
            match self.get_peek_token().unwrap().kind {
                TokenType::ColonEqual => {
                    kind = IdentTypes::Destructuring;
                }
                TokenType::Equal => {
                    kind = IdentTypes::Normal;
                }
                _ => {
                    let exp = self.parse_expression_statement(Some(idents));

                    return exp;
                }
            }
            self.consume_token();

            let token = self.get_current_token().unwrap();

            self.consume_token();
            let expr = self.parse_expression(Precedence::LOWEST, None);

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
                error(format!("Expected expression after assignment operator."));
                panic!();
            }
        } else {
            let exp = self.parse_expression_statement(Some(idents));

            return exp;
        }
    }

    pub fn parse_return_statement(&mut self) -> Option<Statement<'a>> {
        let token = self.get_current_token().unwrap();
        self.consume_token();

        let value = self.parse_expression(Precedence::LOWEST, None).unwrap();
        return Some(Statement::ReturnStatement { token, value });
    }

    pub fn parse_block_statement(&mut self, end_type: TokenType<'a>) -> Statement<'a> {
        let token = self.get_current_token().unwrap();

        self.consume_token();
        let mut statements: Vec<Statement<'a>> = Vec::new();
        while (self.get_current_token().unwrap().kind != end_type)
            && self.current_position != self.tokens.len()
        {
            let statement = self.parse_statement();
            match statement {
                Some(statement) => {
                    statements.push(statement);
                    self.consume_token();
                }
                None => self.consume_token(),
            }
        }

        return Statement::BlockStatement { token, statements };
    }
}

impl<'a> Parser<'a> {
    pub fn parse_expression(
        &mut self,
        precedence: Precedence,
        idents: Option<Vec<Token<'a>>>,
    ) -> Option<Box<Expression<'a>>> {
        let prefix_tok = self.get_current_token().unwrap().kind;

        let prefix = self.prefix_fn(prefix_tok, false, None);

        if !prefix.0 {
            return None;
        }

        let mut prefix_exp = self.prefix_fn(prefix_tok, true, idents).1.unwrap();

        while precedence
            < self
                .get_peek_precedence()
                .unwrap_or_else(|| Precedence::LOWEST)
        {
            let peek = self.get_peek_token().unwrap().kind;

            let (infix_exists, _) = self.infix_fn(peek, false, None);

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

    pub fn prefix_fn(
        &mut self,
        kind: TokenType<'a>,
        execute: bool,
        idents: Option<Vec<Token<'a>>>,
    ) -> (bool, Option<Box<Expression<'a>>>) {
        match (kind, execute) {
            (TokenType::Integer(_), false) => (true, None),
            (TokenType::Integer(_), true) => (true, Some(self.parse_integer_literal())),
            (TokenType::Underscore, false) => (true, None),
            (TokenType::Underscore, true) => (true, Some(self.parse_underscore_literal())),
            (TokenType::Float(_), false) => (true, None),
            (TokenType::Float(_), true) => (true, Some(self.parse_float_literal())),
            (TokenType::String(_), false) => (true, None),
            (TokenType::String(_), true) => (true, Some(self.parse_string_literal())),
            (TokenType::LParen, false) => (true, None),
            (TokenType::LParen, true) => (true, Some(self.parse_grouped_expression())),
            (TokenType::LBracket, false) => (true, None),
            (TokenType::LBracket, true) => (true, Some(self.parse_array_literal())),
            (TokenType::Bar, false) => (true, None),
            (TokenType::Bar, true) => (true, Some(self.parse_function_expression())),
            (TokenType::Ident(_), false) => (true, None),
            (TokenType::Ident(_), true) => (true, Some(self.parse_ident_literal(idents))),
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
            (TokenType::LParen, false) => (true, None),
            (TokenType::LParen, true) => (true, Some(self.parse_call_expression(left.unwrap()))),
            (TokenType::Match, false) => (true, None),
            (TokenType::Match, true) => (true, Some(self.parse_match_expression(left.unwrap()))),
            _ => (false, None),
        }
    }
}

impl<'a> Parser<'a> {
    pub fn parse_infix_expression(&mut self, left: Box<Expression<'a>>) -> Box<Expression<'a>> {
        let current = self.get_current_token().unwrap();
        let precedence = self.get_current_precedence().unwrap();
        self.consume_token();
        let right = self.parse_expression(precedence, None).unwrap();
        return Box::new(Expression::InfixExpression {
            token: current,
            right,
            left,
        });
    }

    pub fn parse_grouped_expression(&mut self) -> Box<Expression<'a>> {
        // self.consume_token();
        if let Some(s) = self.get_current_token() {
            if s.kind == TokenType::LParen {
                self.consume_token();
            }
        }

        let expr = self.parse_expression(Precedence::LOWEST, None);

        if let Some(expr) = expr {
            if let Some(tok) = self.get_peek_token() {
                self.expect_peek(TokenType::RParen, "defining a grouped expression");
            } else {
                if let Some(tok) = self.get_current_token() {
                    if (tok.kind == TokenType::RParen) {
                    } else {
                        error(format!("{} Expected closing parenthesis.", tok.position));
                    }
                }
            }

            return expr;
        } else {
            error(format!("Expected some expression after opening bracket."));
            panic!();
        }
    }

    pub fn parse_call_expression(
        &mut self,
        fn_literal: Box<Expression<'a>>,
    ) -> Box<Expression<'a>> {
        let token = self.get_current_token().unwrap();

        if token.kind != TokenType::LParen {
            error(format!("{} Function calls must start with token of type 'LParen', received '{:?}' instead.", token.position, token.kind));
        }
        let parameters = self.parse_expression_list(TokenType::RParen);
        return Box::new(Expression::CallExpression {
            token,
            parameters,
            function: fn_literal,
        });
    }

    fn parse_expression_list(&mut self, end: TokenType) -> Vec<Box<Expression<'a>>> {
        let mut exprs: Vec<Box<Expression<'a>>> = Vec::new();
        if self.peek_is(end) {
            self.consume_token();
            return exprs;
        }

        self.consume_token();

        exprs.push(self.parse_expression(Precedence::LOWEST, None).unwrap());
        self.consume_token();

        while let Some(tok) = self.get_current_token() {
            if tok.kind != end {
                // self.consume_token();
                if let Some(expr) = self.parse_expression(Precedence::LOWEST, None) {
                    exprs.push(expr)
                }
                // exprs.push(.unwrap());
                self.consume_token();
            } else {
                break;
            }
        }

        return exprs;
    }

    fn parse_match_expression(&mut self, expression: Box<Expression<'a>>) -> Box<Expression<'a>> {
        let token = self.get_current_token().unwrap();

        self.expect_peek(
            TokenType::LBrace,
            "defining the opening of a match expression",
        );

        let mut pairs: Vec<MatchPairExpression<'a>> = Vec::new();
        let mut default: Option<Statement<'a>> = None;
        self.consume_token();

        while self.get_peek_token().unwrap().kind != TokenType::RBrace {
            let mut destructures: Vec<Box<Expression<'a>>> = Vec::new();
            let mut statements: Vec<Statement<'a>> = Vec::new();
            let expr = self.parse_expression(Precedence::LOWEST, None).unwrap();

            match *expr {
                Expression::UnderscoreLiteral { token } => {
                    self.expect_peek(TokenType::Arrow, "defining a match clause");

                    if let Some(peek) = self.get_peek_token() {
                        match peek.kind {
                            TokenType::LBrace => {
                                self.consume_token();
                                let statement = self.parse_block_statement(TokenType::RBrace);
                                default = Some(statement);
                            }
                            _ => {
                                let token = self.get_current_token().unwrap();
                                self.consume_token();
                                let expression =
                                    self.parse_expression(Precedence::LOWEST, None).unwrap(); // TODO: Error check
                                statements.push(Statement::ReturnStatement {
                                    token,
                                    value: expression,
                                });
                                default = Some(Statement::BlockStatement { statements, token })
                            }
                        }
                    } else {
                        error(format!(
                            "{} Expected expression or statement after match predicate.",
                            self.get_current_token().unwrap().position
                        ))
                    }

                    if let Some(peek) = self.get_peek_token() {
                        if peek.kind == TokenType::Comma {
                            self.consume_token();
                            self.consume_token();
                        }
                    }

                    continue;
                }
                _ => {
                    destructures.push(expr);
                }
            }

            // self.expect_peek(TokenType::Arrow, "defining a match clause");

            while let Some(peek) = self.get_peek_token() {
                match peek.kind {
                    TokenType::Comma => {
                        self.consume_token();
                        if let Some(peek2) = self.get_peek_token() {
                            self.consume_token();
                            destructures
                                .push(self.parse_expression(Precedence::LOWEST, None).unwrap());
                        } else {
                            error(format!("{} Expected some token after comma", peek.position));
                            panic!()
                        }
                    }
                    _ => {
                        // self.consume_token();
                        break;
                    }
                }
            }

            self.expect_peek(TokenType::Arrow, "defining a match clause");

            if let Some(peek) = self.get_peek_token() {
                match peek.kind {
                    TokenType::LBrace => {
                        self.consume_token();
                        let statement = self.parse_block_statement(TokenType::RBrace);
                        pairs.push(MatchPairExpression {
                            predicate: destructures,
                            statement,
                        });
                    }
                    _ => {
                        let token = self.get_current_token().unwrap();
                        self.consume_token();
                        let expression = self.parse_expression(Precedence::LOWEST, None).unwrap(); // TODO: Error check
                        statements.push(Statement::ReturnStatement {
                            token,
                            value: expression,
                        });
                        pairs.push(MatchPairExpression {
                            predicate: destructures,
                            statement: Statement::BlockStatement { statements, token },
                        });
                    }
                }
            } else {
                error(format!(
                    "{} Expected expression or statement after match predicate.",
                    self.get_current_token().unwrap().position
                ))
            }

            if let Some(peek) = self.get_peek_token() {
                if peek.kind == TokenType::Comma {
                    self.consume_token();
                    self.consume_token();
                }
            }
        }

        Box::new(Expression::MatchExpression {
            token,
            default,
            pairs,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::lexer::Lexer;
    use crate::parser::Parser;
    #[test]
    fn full_test() {
        let test_str = r#"
        import z
        2 + 2 * 3
        "#;
        let l = Lexer::new(test_str);
        let z = l.collect::<Vec<_>>();

        let mut p = Parser::new(z);
    }

    #[test]
    fn ident_assign() {
        let test_str = r#"
        import http
        z, x := (15 + 2) * 3
        z = 5
        "#;
    }

    #[test]
    fn function_check() {
        let test_str = r#"
        x := (|z, m| -> {
            return m + z.length
        })("hi", 2)

        z := "hi"
        "#;

        let lexer = Lexer::new(test_str).collect::<Vec<_>>();
        let parser = Parser::new(lexer).parse_program();
        println!("{:#?}", parser);
    }

    #[test]
    fn match_check() {
        let test_str = r#"
        x := |n| -> x match { 5, x -> "five", x, _ -> "z", _ -> "not five :)"}
        "#;

        let lexer = Lexer::new(test_str).collect::<Vec<_>>();
        let parser = Parser::new(lexer).parse_program();
        println!("{:#?}", parser);
    }

    #[test]
    fn arr_check() {
        let test_str = r#"
        arr := ["hi", 3, 4, 5]
        mapped := (arr.map((|v| -> v + 2)))
        "#;

        let lexer = Lexer::new(test_str).collect::<Vec<_>>();
        let parser = Parser::new(lexer).parse_program();
        println!("{:#?}", parser);
    }
    #[test]
    fn z_check() {
        let test_str = r#"
        arr := http.io(3)
        
        "#;

        let lexer = Lexer::new(test_str).collect::<Vec<_>>();
        let parser = Parser::new(lexer).parse_program();
        println!("{:#?}", parser);
    }
}
