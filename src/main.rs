use std::{
    env,
    io::{self, Write},
};

use colored::Colorize;

mod ast;
mod errors;
mod lexer;
mod parser;
mod token;
fn main() -> io::Result<()> {
    let mut args = env::args().collect::<Vec<_>>();

    match args.len() {
        1 => {
            println!(
                "Clay language REPL @{}",
                "0.0.1".on_bright_magenta().black()
            );
            println!("Type `exit` to exit.\n");

            loop {
                print!("{} ", "#>".on_bright_yellow().black());
                io::stdout().flush()?;

                let mut input = String::new();
                io::stdin().read_line(&mut input)?;

                match input.trim() {
                    "exit" => break,
                    _ => {
                        let tokens = lexer::Lexer::new(input.trim()).collect::<Vec<_>>();
                        let stack = parser::Parser::new(tokens).parse_program();

                        println!("\n{:#?}\n", stack);
                    }
                }
            }
        }
        _ => {
            println!("Unfortunately, only the REPL is supported at the moment.")
        }
    }

    Ok(())
}
