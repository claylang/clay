use std::collections::HashMap;

use crate::lexer::token::Token;

#[derive(Debug, Clone)]
pub struct Program<'a> {
    pub statements: Vec<Statement<'a>>,
}
#[derive(Debug, Clone)]
pub enum Expression<'a> {
    IdentifierLiteral {
        token: Token<'a>,
    },
    StringLiteral {
        token: Token<'a>,
    },
    ArrayLiteral {
        token: Token<'a>,
        elements: Vec<Expression<'a>>,
    },
    BooleanLiteral {
        token: Token<'a>,
    },
    IntegerLiteral {
        token: Token<'a>,
    },
    FloatLiteral {
        token: Token<'a>,
    },
    IndexExpression {
        token: Token<'a>,
        left: Box<Expression<'a>>,
        index: Box<Expression<'a>>,
    },
    MapLiteral {
        token: Token<'a>,
        pairs: HashMap<Box<Expression<'a>>, Box<Expression<'a>>>,
    },
    FunctionLiteral {
        token: Token<'a>,
        parameters: Vec<Expression<'a>>,
    },

    PrefixExpression {
        token: Token<'a>,
        right: Box<Expression<'a>>,
    },

    InfixExpression {
        token: Token<'a>,
        right: Box<Expression<'a>>,
        left: Box<Expression<'a>>,
    },

    IfExpression {
        token: Token<'a>,
        condition: Box<Expression<'a>>,
        consequence: Statement<'a>,
        alternative: Statement<'a>,
    },
}

#[derive(Debug, Clone)]
pub enum Statement<'a> {
    BlockStatement {
        token: Token<'a>,
        statements: Vec<Statement<'a>>,
    },
    ExpressionStatement {
        token: Token<'a>,
        expression: Box<Expression<'a>>,
    },
    ValueStatement {
        token: Token<'a>,
        name: Token<'a>,
        value: Box<Expression<'a>>,
    },
    UpdateStatement {
        token: Token<'a>,
        name: Token<'a>,
        value: Box<Expression<'a>>,
    },
    ReturnStatement {
        token: Token<'a>,
        value: Box<Expression<'a>>,
    },
    ImportStatement {
        token: Token<'a>,
        value: Token<'a>,
    },
}
