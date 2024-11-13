use crate::lexer::Lexer;
use crate::lexer::Token;
use crate::statement::Statement;
use crate::statement::Expression;
use crate::statement::Operator;

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    curr: Option<Token>,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: Lexer<'a>) -> Self {
        let mut parser = Parser {
            lexer,
            curr: None,
        };
        parser.advance();
        parser
    }

    fn advance(&mut self) {
        self.curr = self.lexer.next_token();
    }

    pub fn parse(&mut self) -> Vec<Statement> {
        let mut statements = Vec::new();
        while let Some(ref token) = self.curr.clone() {
            match token {
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

    pub fn parse_expression(&mut self) -> Option<Expression> {
        let mut left = self.parse_term()?;
    
        while let Some(Token::Symbol(op)) = &self.curr {
            match op {
                '+' => {
                    self.advance();
                    let right = self.parse_term()?;
                    left = Expression::BinaryOp(Box::new(left), Operator::Add, Box::new(right));
                }
                '-' => {
                    self.advance();
                    let right = self.parse_term()?;
                    left = Expression::BinaryOp(Box::new(left), Operator::Sub, Box::new(right));
                }
                _ => break,
            }
        }

        Some(left)
    }
    
    pub fn parse_term(&mut self) -> Option<Expression> {
        let mut left = self.parse_factor()?;
    
        while let Some(Token::Symbol(op)) = &self.curr {
            match op {
                '*' => {
                    self.advance();
                    let right = self.parse_factor()?;
                    left = Expression::BinaryOp(Box::new(left), Operator::Mul, Box::new(right));
                }
                '/' => {
                    self.advance();
                    let right = self.parse_factor()?;
                    left = Expression::BinaryOp(Box::new(left), Operator::Div, Box::new(right));
                }
                _ => break,
            }
        }

        Some(left)
    }
    
    pub fn parse_factor(&mut self) -> Option<Expression> {
        match &self.curr {
            Some(Token::Ident(name)) => {
                let name = name.clone();
                self.advance();
                if let Some(Token::Symbol('(')) = &self.curr {
                    self.advance();
                    let mut args = Vec::new();
    
                    while let Some(expr) = self.parse_expression() {
                        args.push(expr);
                        if let Some(Token::Symbol(',')) = &self.curr {
                            self.advance();
                        } else {
                            break;
                        }
                    }
    
                    if let Some(Token::Symbol(')')) = &self.curr {
                        self.advance();
                        return Some(Expression::FunctionCall { name, args });
                    } else {
                        None
                    }
                } else {
                    return Some(Expression::Identifier(name));
                }
            }
            Some(Token::Number(num)) => {
                let num = *num;
                self.advance();
                Some(Expression::Number(num))
            }
            Some(Token::StringLiteral(s)) => {
                let s = s.clone();
                self.advance();
                Some(Expression::StringLiteral(s))
            }
            Some(Token::Symbol('(')) => {
                self.advance();
                let expr = self.parse_expression();
                if let Some(Token::Symbol(')')) = &self.curr {
                    self.advance();
                }
                expr
            }
            _ => None,
        }
    }
}