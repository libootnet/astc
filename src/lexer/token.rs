use std::str::Chars;

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Keyword(String),
    Ident(String),
    Number(f64),
    StringLiteral(String),
    Type(String),
    Symbol(char),
    Unknown(char),
}

pub struct Lexer<'a> {
    pub input: Chars<'a>,
    pub curr: Option<char>,
}