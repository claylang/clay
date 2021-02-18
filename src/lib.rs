#![allow(dead_code, unreachable_patterns)]

mod ast;
mod lexer;
mod parser;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
