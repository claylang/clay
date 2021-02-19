use std::usize;

use crate::{
    errors::error,
    token::{Position, Token, TokenType},
};

pub struct Lexer<'a> {
    input: &'a str,
    position: Position,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Lexer {
        Lexer {
            input,
            position: Position::new(1, 0, 0),
        }
    }

    pub fn consume_char(&mut self) {
        self.position.column += 1;
        self.position.char += 1;
    }

    pub fn get_nth_char(&self, position: usize) -> Option<char> {
        return self.input.chars().nth(position);
    }

    pub fn get_current_char(&self) -> Option<char> {
        return self.input.chars().nth(self.position.char);
    }

    pub fn get_peek_char(&self) -> Option<char> {
        return self.input.chars().nth(self.position.char + 1);
    }

    pub fn lex_single_char<'b>(&mut self, kind: TokenType<'b>) -> Option<Token<'b>> {
        let position = self.position;
        self.consume_char();
        return Some(Token { position, kind });
    }

    pub fn lex_double_char<'b>(&mut self, kind: TokenType<'b>) -> Option<Token<'b>> {
        let position = self.position;
        self.consume_char();
        self.consume_char();
        Some(Token { kind, position })
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Token<'a>> {
        let current_char = self.get_current_char();
        if current_char == None {
            return None;
        }

        let peek_char = self.get_peek_char();

        match current_char.unwrap() {
            '(' => self.lex_single_char(TokenType::LParen),
            ')' => self.lex_single_char(TokenType::RParen),
            '[' => self.lex_single_char(TokenType::LBracket),
            ']' => self.lex_single_char(TokenType::RBracket),
            '{' => self.lex_single_char(TokenType::LBrace),
            '}' => self.lex_single_char(TokenType::RBrace),
            ',' => self.lex_single_char(TokenType::Comma),
            '.' => self.lex_single_char(TokenType::Period),
            '_' => self.lex_single_char(TokenType::Underscore),
            '!' => match peek_char {
                Some('=') => self.lex_double_char(TokenType::BangEqual),
                _ => self.lex_single_char(TokenType::Bang),
                // _ => panic!(
                //     "{} Found illegal token '{}'",
                //     self.position,
                //     String::from(current_char.unwrap()) + &String::from(peek_char.unwrap())[..]
                // ),
            },
            '=' => match peek_char {
                Some('=') => self.lex_double_char(TokenType::DoubleEqual),
                _ => {
                    self.lex_single_char(TokenType::Equal)
                }
                // _ => panic!(
                //     "{} Found illegal token '{}'",
                //     self.position,
                //     String::from(current_char.unwrap()) + &String::from(peek_char.unwrap())[..]
                // ),
            },

            // '=' => match peek_char {
            //     Some('=') => self.lex_double_char(TokenType::DoubleEqual),
            //     None | Some(' ') | Some('\t') | Some('\r') => {
            //         self.lex_single_char(TokenType::Equal)
            //     }
            //     _ => panic!(
            //         "{} Found illegal token '{}'",
            //         self.position,
            //         String::from(current_char.unwrap()) + &String::from(peek_char.unwrap())[..]
            //     ),
            // },
            '|' => self.lex_single_char(TokenType::Bar),

            '&' => match peek_char {
                Some('&') => self.lex_double_char(TokenType::And),
                _ => {
                    self.lex_single_char(TokenType::Ampersand)
                }
                // _ => panic!(
                //     "{} Found illegal token '{}'",
                //     self.position,
                //     String::from(current_char.unwrap()) + &String::from(peek_char.unwrap())[..]
                // ),
            },
            '+' => match peek_char {
                Some('=') => self.lex_double_char(TokenType::PlusEqual),
                _ => self.lex_single_char(TokenType::Plus),
                // _ => panic!(
                //     "{} Found illegal token '{}'",
                //     self.position,
                //     String::from(current_char.unwrap()) + &String::from(peek_char.unwrap())[..]
                // ),
            },
            '-' => match peek_char {
                Some('=') => self.lex_double_char(TokenType::MinusEqual),
                Some('>') => self.lex_double_char(TokenType::Arrow),
                _ => {
                    self.lex_single_char(TokenType::Minus)
                }
                // _ => panic!(
                //     "{} Found illegal token '{}'",
                //     self.position,
                //     String::from(current_char.unwrap()) + &String::from(peek_char.unwrap())[..]
                // ),
            },
            '/' => match peek_char {
                Some('=') => self.lex_double_char(TokenType::SlashEqual),
                _ => {
                    self.lex_single_char(TokenType::Slash)
                }
                // _ => panic!(
                //     "{} Found illegal token '{}'",
                //     self.position,
                //     String::from(current_char.unwrap()) + &String::from(peek_char.unwrap())[..]
                // ),
            },

            '*' => match peek_char {
                Some('=') => self.lex_double_char(TokenType::AsteriskEqual),
                _ => {
                    self.lex_single_char(TokenType::Asterisk)
                }
                // _ => panic!(
                //     "{} Found illegal token '{}'",
                //     self.position,
                //     String::from(current_char.unwrap()) + &String::from(peek_char.unwrap())[..]
                // ),
            },
            '<' => match peek_char {
                Some('=') => self.lex_double_char(TokenType::LTEq),
                _ => {
                    self.lex_single_char(TokenType::Asterisk)
                }
                // _ => panic!(
                //     "{} Found illegal token '{}'",
                //     self.position,
                //     String::from(current_char.unwrap()) + &String::from(peek_char.unwrap())[..]
                // ),
            },
            '>' => match peek_char {
                Some('=') => self.lex_double_char(TokenType::GTEq),
                _ => {
                    self.lex_single_char(TokenType::Asterisk)
                }
                // _ => panic!(
                //     "{} Found illegal token '{}'",
                //     self.position,
                //     String::from(current_char.unwrap()) + &String::from(peek_char.unwrap())[..]
                // ),
            },
            ':' => match peek_char {
                Some('=') => self.lex_double_char(TokenType::ColonEqual),
                _ => {
                    self.lex_single_char(TokenType::Colon)
                }
                // _ => panic!(
                //     "{} Found illegal token '{}'",
                //     self.position,
                //     String::from(current_char.unwrap()) + &String::from(peek_char.unwrap())[..]
                // ),
            },

            '0'..='9' => {
                enum NumberTypes {
                    Int,
                    Float,
                }

                let position = self.position;
                let mut num = String::new();
                let mut num_type = NumberTypes::Int;

                while let Some(ch) = self.get_current_char() {
                    match ch {
                        '0'..='9' => {
                            num.push(ch);
                            self.consume_char();
                        }
                        '.' if matches!(self.get_peek_char(), Some('0'..='9')) => {
                            num_type = NumberTypes::Float;
                            num.push(ch);
                            self.consume_char();
                        }
                        _ => break,
                    }
                }

                match num_type {
                    NumberTypes::Int => match num.parse::<usize>() {
                        Ok(n) => Some(Token {
                            position,
                            kind: TokenType::Integer(n),
                        }),
                        Err(e) => {
                            error(format!(
                                "{} Could not parse '{}' as an integer.",
                                self.position, num
                            ));
                            panic!()
                        }
                    },
                    NumberTypes::Float => match num.parse::<f32>() {
                        Ok(n) => Some(Token {
                            position,
                            kind: TokenType::Float(n),
                        }),
                        Err(e) => {
                            error(format!(
                                "{} Could not parse '{}' as a float.",
                                self.position, num
                            ));
                            panic!()
                        }
                    },
                }
            }
            '"' => {
                self.consume_char();
                let position = self.position;
                let mut end: usize = 0;
                while let Some(ch) = self.get_current_char() {
                    match ch {
                        '"' => {
                            end = self.position.char;
                            self.consume_char();
                            break;
                        }
                        '\n' => {
                            self.position.line += 1;
                            self.position.column = 0;
                            self.consume_char();
                        }
                        _ => self.consume_char(),
                    }
                }
                return Some(Token {
                    kind: TokenType::String(&self.input[position.char..end]),
                    position: self.position,
                });
            }
            'a'..='z' | 'A'..='Z' | '_' => {
                let position = self.position;

                while let Some(ch) = self.get_current_char() {
                    match ch {
                        'A'..='Z' | 'a'..='z' | '_' => {
                            self.consume_char();
                        }
                        _ => break,
                    }
                }
                let end = self.position.char;

                let slice = &self.input[position.char..end];
                let kind = TokenType::match_keyword(slice);

                Some(Token { kind, position })
            }
            '\n' => {
                self.position.line += 1;
                self.position.column = 0;
                self.consume_char();
                self.next()
            }
            ' ' | '\t' | '\r' => {
                self.consume_char();
                self.next()
            }

            _ => {
                error(format!(
                    "{} Found illegal token '{}'",
                    self.position,
                    String::from(current_char.unwrap()) + &String::from(peek_char.unwrap())[..]
                ));
                panic!()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::lexer::Lexer;
    #[test]
    fn it_works() {
        let test_str = r#"1 + 2.3555
        "hi there!" awefawe
        "#;
        let l = Lexer::new(test_str);
        let z = l.collect::<Vec<_>>();
        println!("{:#?}", z);
    }
}
