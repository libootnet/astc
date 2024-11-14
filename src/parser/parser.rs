use crate::lexer::token::{Lexer, Token};

use crate::parser::statement::{
    ComparisonOperator, Expression, LogicalOperator, Operator, Statement,
};

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    curr: Option<Token>,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: Lexer<'a>) -> Self {
        let mut parser = Parser { lexer, curr: None };
        parser.advance();
        parser
    }

    fn advance(&mut self) {
        self.curr = self.lexer.next_token();
    }

    fn expect(&mut self, expected: Token) -> Option<()> {
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

    pub fn parse_expression(&mut self) -> Option<Expression> {
        self.parse_logical_expression()
    }

    pub fn parse_logical_expression(&mut self) -> Option<Expression> {
        let mut left = self.parse_comparison_expression()?;

        while let Some(Token::Symbol(op)) = &self.curr {
            match op {
                '&' => {
                    self.advance();
                    if let Some(Token::Symbol('&')) = self.curr {
                        self.advance();
                        let right = self.parse_comparison_expression()?;
                        left = Expression::LogicalOp(
                            Box::new(left),
                            LogicalOperator::And,
                            Box::new(right),
                        );
                    } else {
                        return None;
                    }
                }
                '|' => {
                    self.advance();
                    if let Some(Token::Symbol('|')) = self.curr {
                        self.advance();
                        let right = self.parse_comparison_expression()?;
                        left = Expression::LogicalOp(
                            Box::new(left),
                            LogicalOperator::Or,
                            Box::new(right),
                        );
                    } else {
                        return None;
                    }
                }
                _ => break,
            }
        }

        Some(left)
    }

    pub fn parse_comparison_expression(&mut self) -> Option<Expression> {
        let mut left = self.parse_additive_expression()?;

        while let Some(Token::Symbol(op)) = &self.curr {
            let comparison_operator = match op {
                '=' => {
                    self.advance();
                    if let Some(Token::Symbol('=')) = self.curr {
                        self.advance();
                        ComparisonOperator::Equal
                    } else {
                        return None;
                    }
                }
                '!' => {
                    self.advance();
                    if let Some(Token::Symbol('=')) = self.curr {
                        self.advance();
                        ComparisonOperator::NotEqual
                    } else {
                        return None;
                    }
                }
                '<' => {
                    self.advance();
                    if let Some(Token::Symbol('=')) = self.curr {
                        self.advance();
                        ComparisonOperator::LessThanOrEqual
                    } else {
                        ComparisonOperator::LessThan
                    }
                }
                '>' => {
                    self.advance();
                    if let Some(Token::Symbol('=')) = self.curr {
                        self.advance();
                        ComparisonOperator::GreaterThanOrEqual
                    } else {
                        ComparisonOperator::GreaterThan
                    }
                }
                _ => break,
            };

            let right = self.parse_additive_expression()?;
            left = Expression::ComparisonOp(Box::new(left), comparison_operator, Box::new(right));
        }

        Some(left)
    }

    pub fn parse_additive_expression(&mut self) -> Option<Expression> {
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
                    Some(Expression::Identifier(name))
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
