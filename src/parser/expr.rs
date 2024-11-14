use crate::lexer::token::Token;
use crate::parser::parser::Parser;

use crate::parser::statement::{
    ComparisonOperator, Expression, LogicalOperator, Operator,
};

impl<'a> Parser<'a> {
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
