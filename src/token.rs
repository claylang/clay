use colored::*;
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TokenType<'a> {
    RParen,   // )
    LParen,   // (
    RBrace,   // }
    LBrace,   // {
    RBracket, // ]
    LBracket, // [

    Percent,
    Plus,
    Minus,
    Slash,
    Asterisk,
    Equal,
    DoubleEqual,
    Bang,
    BangEqual,
    LT,
    LTEq,
    GT,
    GTEq,
    Arrow,
    Underscore,

    Period,
    Semicolon,
    Ampersand,
    And,
    Bar,
    Or,
    PlusEqual,
    MinusEqual,
    SlashEqual,
    AsteriskEqual,
    Comma,
    Colon,
    ColonEqual,

    Integer(usize),
    Float(f32),
    String(&'a str),

    // Keywords
    Ident(&'a str),
    Match,
    Import,
    Return,
}

impl<'a> TokenType<'a> {
    pub fn match_keyword(string: &'a str) -> TokenType {
        match string {
            "match" => TokenType::Match,
            "import" => TokenType::Import,
            "return" => TokenType::Return,
            _ => TokenType::Ident(string),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Token<'a> {
    pub kind: TokenType<'a>,
    pub position: Position,
}

impl<'a> Token<'a> {
    pub fn new(kind: TokenType, position: Position) -> Token {
        Token { kind, position }
    }

    pub fn from_keyword(keyword: &'a str, position: Position) -> Token {
        Token {
            kind: TokenType::match_keyword(keyword),

            position,
        }
    }
}
impl<'a> std::fmt::Display for Token<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({:?} @ {})", self.kind, self.position)
    }
}

impl std::fmt::Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            format!("({}, {})", self.line, self.column)
                .on_purple()
                .black()
        )
    }
}
#[derive(Debug, Clone, Copy)]
pub struct Position {
    pub line: usize,
    pub column: usize,
    pub char: usize,
}

impl Position {
    pub fn new(line: usize, column: usize, char: usize) -> Position {
        Position { line, column, char }
    }
}
