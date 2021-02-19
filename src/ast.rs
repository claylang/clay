use std::collections::HashMap;

use crate::token::Token;

#[derive(Debug, Clone)]
pub struct Program<'a> {
    pub statements: Vec<Statement<'a>>,
}
#[derive(Debug, Clone)]
pub enum Expression<'a> {
    DefinitionIdentifier {
        idents: Vec<Token<'a>>,
    },
    NormalIdentifier {
        idents: Vec<Token<'a>>,
    },
    StringLiteral {
        token: Token<'a>,
    },
    ArrayLiteral {
        token: Token<'a>,
        elements: Vec<Box<Expression<'a>>>,
    },
    BooleanLiteral {
        token: Token<'a>,
    },
    UnderscoreLiteral {
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
        parameters: Box<Expression<'a>>,
        statements: Statement<'a>,
    },
    CallExpression {
        token: Token<'a>,
        parameters: Vec<Box<Expression<'a>>>,
        function: Box<Expression<'a>>,
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

    MatchExpression {
        token: Token<'a>,
        default: Option<Statement<'a>>,
        pairs: Vec<MatchPairExpression<'a>>,
    },
}

#[derive(Debug, Clone)]
pub struct MatchPairExpression<'a> {
    pub statement: Statement<'a>,
    pub predicate: Vec<Box<Expression<'a>>>,
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
    AssignStatement {
        token: Token<'a>,
        expression: Box<Expression<'a>>,
        defined: Box<Expression<'a>>,
    },

    UpdateStatement {
        token: Token<'a>,
        ident: Box<Expression<'a>>,
        expression: Box<Expression<'a>>,
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
