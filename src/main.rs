mod object;
mod lexer;
mod parser;
mod function;
mod env;

use parser::Parser;
use lexer::Lexer;
use std::io::{self, Write};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("type bye to exit");
    loop {
        print!("> ");
        io::stdout().flush()?;
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim();
        if input == "bye" { break; }
        println!("{:#?}", Parser::new(Lexer::new(input)).parse())
    }

    Ok(())
}
