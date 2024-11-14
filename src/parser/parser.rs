use crate::lexer::token::{Lexer, Token};
// use crate::parser::expr;

use crate::parser::statement::Statement;

pub struct Parser<'a> {
    pub lexer: Lexer<'a>,
    pub curr: Option<Token>,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: Lexer<'a>) -> Self {
        let mut parser = Parser { lexer, curr: None };
        parser.advance();
        parser
    }

    pub fn advance(&mut self) {
        self.curr = self.lexer.next_token();
    }

    pub fn expect(&mut self, expected: Token) -> Option<()> {
        if let Some(token) = &self.curr {
            if *token == expected {
                self.advance();
                Some(())
            } else {
                eprintln!("Error: Expected {:?}", expected);
                None
            }
        } else {
            eprintln!("Error: Unexpected end of input.");
            None
        }
    }

    pub fn parse(&mut self) -> Vec<Statement> {
        let mut statements = Vec::new();
        while let Some(ref token) = self.curr.clone() {
            println!("debug {:?}", token);
            match token {
                // block end
                Token::Symbol(ref k) if *k == '}' => {
                    break;
                }

                Token::Keyword(ref k) if k == "if" => {
                    if let Some(stmt) = self.parse_if_statement() {
                        statements.push(stmt);
                    }
                }

                Token::Keyword(ref k) if k == "let" || k == "const" => {
                    self.advance();
                    if let Some(stmt) = self.parse_var_decl() {
                        statements.push(stmt);
                    }
                }

                Token::Ident(ref ident) => {
                    self.advance();
                    if let Some(stmt) = self.parse_function_call(ident.clone()) {
                        statements.push(stmt);
                    }
                }

                _ => self.advance(),
            }
        }
        statements
    }

    fn parse_if_statement(&mut self) -> Option<Statement> {
        self.advance();

        /*
        if ( condition ) {
           block;
        }
        */

        if self.expect(Token::Symbol('(')).is_none() {
            return None;
        }

        let condition = self.parse_expression()?;

        if self.expect(Token::Symbol(')')).is_none() {
            return None;
        }

        if self.expect(Token::Symbol('{')).is_none() {
            return None;
        }

        let mut then_branch = Vec::new();
        while let Some(token) = &self.curr {
            if let Token::Symbol('}') = token {
                break;
            }
            then_branch.extend(self.parse());
        }

        if self.expect(Token::Symbol('}')).is_none() {
            return None;
        }

        let else_branch = if let Some(Token::Keyword(ref k)) = self.curr {
            if k == "else" {
                self.advance();
                if let Some(Token::Keyword(ref k)) = self.curr {
                    if k == "if" {
                        return Some(Statement::If {
                            condition,
                            then_branch,
                            else_branch: Some(Box::new(self.parse_if_statement()?)),
                        });
                    }
                }

                if self.expect(Token::Symbol('{')).is_none() {
                    return None;
                }

                let mut else_branch = Vec::new();
                while let Some(token) = &self.curr {
                    if let Token::Symbol('}') = token {
                        break;
                    }
                    else_branch.extend(self.parse());
                }

                if self.expect(Token::Symbol('}')).is_none() {
                    return None;
                }

                Some(Box::new(Statement::Block(else_branch)))
            } else {
                None
            }
        } else {
            None
        };

        Some(Statement::If {
            condition,
            then_branch,
            else_branch,
        })
    }

    fn parse_var_decl(&mut self) -> Option<Statement> {
        if let Some(Token::Ident(name)) = &self.curr {
            let var_name = name.clone();
            self.advance();

            let var_type = if let Some(Token::Symbol(':')) = self.curr {
                self.advance();
                if let Some(Token::Type(type_name)) = &self.curr {
                    let type_name = type_name.clone();
                    self.advance();
                    Some(type_name)
                } else {
                    None
                }
            } else {
                None
            };

            if let Some(Token::Symbol('=')) = self.curr {
                self.advance();
                if let Some(value) = self.parse_expression() {
                    return Some(Statement::VarDeclaration {
                        name: var_name,
                        var_type,
                        value,
                    });
                }
            }
        }
        None
    }

    fn parse_function_call(&mut self, name: String) -> Option<Statement> {
        if let Some(Token::Symbol('(')) = self.curr {
            self.advance();
            let mut args = Vec::new();

            while let Some(expr) = self.parse_expression() {
                args.push(expr);
                if let Some(Token::Symbol(',')) = self.curr {
                    self.advance();
                } else {
                    break;
                }
            }

            if let Some(Token::Symbol(')')) = self.curr {
                self.advance();
                return Some(Statement::FunctionCall { name, args });
            }
        }
        None
    }
}
