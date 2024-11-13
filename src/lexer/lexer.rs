use crate::lexer::token::{Lexer, Token};

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut lexer = Lexer {
            input: input.chars(),
            curr: None,
        };
        lexer.advance();
        lexer
    }

    pub fn next_token(&mut self) -> Option<Token> {
        while let Some(c) = self.curr {
            match c {
                ' ' | '\n' | '\t' => {
                    self.advance();
                    continue;
                }

                '"' => {
                    return Some(self.str_literal('"'));
                }

                '`' => {
                    return Some(self.str_literal('`'));
                }

                '\'' => {
                    return Some(self.str_literal('\''));
                }

                'a'..='z' | 'A'..='Z' | '_' => {
                    return Some(self.ident_type());
                }

                '0'..='9' => {
                    return Some(self.number());
                }

                ':' | ';' | ',' | '{' | '}' | '(' | ')' | '+' | '-' | '*' | '/' | '=' | '<'
                | '>' | '.' => {
                    let symbol = Token::Symbol(c);
                    self.advance();
                    return Some(symbol);
                }
                _ => {
                    let unknown = Token::Unknown(c);
                    self.advance();
                    return Some(unknown);
                }
            }
        }
        None
    }

    fn advance(&mut self) {
        self.curr = self.input.next();
    }

    fn ident_type(&mut self) -> Token {
        let mut result = String::new();
        while let Some(c) = self.curr {
            if c.is_alphanumeric() || c == '_' {
                result.push(c);
                self.advance();
            } else {
                break;
            }
        }

        match result.as_str() {
            "let" | "const" | "function" | "if" | "else" | "return" => Token::Keyword(result),
            "i8" | "i16" | "i32" | "i64" | "u8" | "u16" | "u32" | "u64" | "string" | "number" => {
                Token::Type(result)
            }
            _ => Token::Ident(result),
        }
    }

    fn number(&mut self) -> Token {
        let mut result = String::new();
        while let Some(c) = self.curr {
            if c.is_numeric() || c == '.' {
                result.push(c);
                self.advance();
            } else {
                break;
            }
        }
        Token::Number(result.parse::<f64>().unwrap())
    }

    fn str_literal(&mut self, q: char) -> Token {
        let mut result = String::new();
        self.advance();
        while let Some(c) = self.curr {
            if c == q {
                break;
            }
            result.push(c);
            self.advance();
        }
        self.advance();
        Token::StringLiteral(result)
    }
}
