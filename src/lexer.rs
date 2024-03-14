use std::iter::Iterator;
use crate::parser::Parser;


#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Ident(String),
    Int(i32),
    Float(f64),
    Quote,
    LParen,
    RParen,
    Str(String),
    Bool(bool),
    EOF,
    Illegal
}

const SUPPORTED_IDENT_CHARS: &str = "!$%&*+-./:<=>?@^_~";

impl Token {
    fn is_identifier_letter(c: char) -> bool {
        let elem = |cmp_char: char, string: &str| {
            for char in string.chars() {
                if cmp_char == char {
                    return true;
                }
            }

            false
        };

        c.is_alphabetic() || elem(c, SUPPORTED_IDENT_CHARS)
    }
}

#[derive(Debug, Clone)]
pub struct Lexer {
    input: Vec<char>,
    position: usize,
    read_position: usize,
    line_number:usize,
    ch: char,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Lexer {
            input: input.chars().collect(),
            position: 0,
            read_position: 0,
            line_number: 0,
            ch: ' ',
        }
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        let tok = match self.ch {
            '(' => Token::LParen,
            ')' => Token::RParen,
            '\'' => Token::Quote,
            '\0' => Token::EOF,
            '#' if self.peek_char() == 't' => {
                self.read_char();
                Token::Bool(true)
            }
            '#' if self.peek_char() == 'f' => {
                self.read_char();
                Token::Bool(false)
            },
            '"' => return self.read_string(),
            _ if Token::is_identifier_letter(self.ch) => return self.read_identifier(),
            _ if self.ch.is_digit(10) => return self.read_number(),
            _ => Token::Illegal
        };

        self.read_char();

        tok
    }

    fn read_char(&mut self) {
        self.ch = self.peek_char();

        self.position = self.read_position;
        self.read_position += 1;
    }

    fn peek_char(&self) -> char {
        self.input
            .get(self.read_position)
            .copied()
            .unwrap_or('\0')
    }

    fn skip_whitespace(&mut self) {
        while self.ch == ' ' || self.ch == '\t' || self.ch == '\n' || self.ch == '\r' {
            if self.ch == '\n' {
                self.line_number += 1;
            }
            self.read_char();
        }
    }

    fn read_number(&mut self) -> Token {
        let position = self.position;

        let mut periods: i8 = 0;

        while self.ch.is_digit(10) || self.ch == '.' && periods < 1 {
            if self.ch == '.' {
                periods += 1;
            }

            self.read_char()
        }

        let num: String = self.input[position..(self.position)]
            .iter().collect();

        if periods == 1 {
            Token::Float(num.parse().expect("error parsing float"))
        } else {
            Token::Int(num.parse().expect("error parsing int"))
        }
    }

    fn read_identifier(&mut self) -> Token {
        let position = self.position;

        while Token::is_identifier_letter(self.ch) {
            self.read_char();
        }

        return Token::Ident(self.input[position..self.position].iter().collect::<String>());
    }

    fn read_string(&mut self) -> Token {
        self.read_char(); // skip over first quote
        let position = self.position;
        // TODO: add escaping quotes \"
        while self.ch != '"' {
            self.read_char();
        }

        self.read_char(); // skip over second quote

        return Token::Str(self.input[position..self.position-1].iter().collect())
    }
}

impl Iterator for Lexer {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        match self.next_token() {
            Token::EOF => None,
            x => Some(x),
        }
    }
}