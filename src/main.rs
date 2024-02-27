mod ast;
mod lexer;
mod parser;

use parser::Parser;
use lexer::Lexer;

const INPUT: &str =
    r#"
    1234 '(+ 1 2) #t #f
    (lamda '(x y) (+ x y))
    "#;


fn main() -> Result<(), Box<dyn std::error::Error>>{
    let lex = Lexer::new(INPUT);

    println!("{:#?}", Parser::new(lex).parse()?);

    Ok(())
}