use crate::lexer::{Token, Lexer};
use crate::object::LispObject;
use std::fmt::{Display, Formatter};
use std::error::Error;

#[derive(Debug)]
pub struct Parser {
    input: Vec<Token>,
    position: usize,
    read_position: usize,
}

#[derive(Debug)]
pub enum ParserError {
    UnexpectedToken(Token),
}

// need to implement a bunch of stuff for error propagation dw about it
impl Error for ParserError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }

    fn description(&self) -> &str {
        match self {
            ParserError::UnexpectedToken(_t) => "unexpected token",
        }
    }
}
impl Display for ParserError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            ParserError::UnexpectedToken(t) => format!("unexpected token: {t:?}"),
        })
    }
}

type ParserObject = Result<LispObject, ParserError>;

impl Parser {
    pub fn new(lex: Lexer) -> Self {
        let mut p = Parser {
            input: lex.collect(),
            position: 0,
            read_position: 0,
        };
        p.advance_token();

        return p
    }

    pub fn parse(&mut self) -> ParserObject {
        let mut statements: Vec<LispObject> = Vec::new();
        while self.current_token() != Token::EOF {
            statements.push(self.parse_expr()?);
            self.advance_token();
        }

        Ok(LispObject::List(statements))
    }

    fn parse_expr(&mut self) -> ParserObject {
        Ok(match self.current_token() {
            Token::Quote => self.parse_quoted_list()?,
            Token::LParen => self.parse_list()?,
            Token::Str(s) => LispObject::Str(s),
            Token::Ident(s) => LispObject::Symbol(s),
            Token::Int(i) => LispObject::Int(i),
            Token::Float(f) => LispObject::Float(f),
            Token::Bool(b) => LispObject::Bool(b),
            t => {
                return Err(ParserError::UnexpectedToken(t))
            }
        })
    }

    fn current_token(&self) -> Token {
        self.input.get(self.position).unwrap_or(&Token::EOF).clone()
    }

    fn advance_token(&mut self) {
        self.position = self.read_position;
        self.read_position += 1;
    }

    #[allow(unused)]
    fn peek_token(&self) -> Token {
        self.input.get(self.read_position).unwrap_or(&Token::EOF).clone()
    }
    fn parse_list(&mut self) -> ParserObject {
        let mut elements: Vec<LispObject> = Vec::new();

        self.advance_token();
        while self.current_token() != Token::RParen {
            elements.push(self.parse_expr()?);
            self.advance_token();
        }

        Ok(LispObject::List(elements))
    }

    fn parse_quoted_list(&mut self) -> ParserObject {
        self.advance_token();

        Ok(LispObject::List(
            vec![LispObject::Symbol("quote".into()), self.parse_expr()?]
        ))
    }
}