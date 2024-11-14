use std::fmt;

use crate::parser::statement::{
    ComparisonOperator, Expression, LogicalOperator, Operator, Statement,
};

impl fmt::Display for Statement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Statement::VarDeclaration {
                name,
                var_type,
                value,
            } => {
                let var_type_str = match var_type {
                    Some(t) => t.clone(),
                    None => "unknown".to_string(),
                };
                write!(f, "Var: {}: {} = {}", name, var_type_str, value)
            }
            Statement::FunctionCall { name, args } => {
                let args_str = args
                    .iter()
                    .map(|arg| format!("{}", arg))
                    .collect::<Vec<String>>()
                    .join(", ");
                write!(f, "FunctionCall: {}({})", name, args_str)
            }
            Statement::If {
                condition,
                then_branch,
                else_branch,
            } => {
                write!(f, "If: ({}) {{\n", condition)?;
                for stmt in then_branch {
                    writeln!(f, "    {}", stmt)?;
                }
                write!(f, "}}")?;

                if let Some(else_branch) = else_branch {
                    match **else_branch {
                        Statement::If { .. } => {
                            write!(f, " else {}", else_branch)?;
                        }
                        Statement::Block(ref stmts) => {
                            write!(f, " else {{\n")?;
                            for stmt in stmts {
                                writeln!(f, "    {}", stmt)?;
                            }
                            write!(f, "}}")?;
                        }
                        _ => {}
                    }
                }

                Ok(())
            }
            Statement::Block(stmts) => {
                write!(f, "{{\n")?;
                for stmt in stmts {
                    writeln!(f, "    {}", stmt)?;
                }
                write!(f, "}}")
            }
        }
    }
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Expression::Identifier(name) => write!(f, "Ident: {}", name),
            Expression::Number(num) => write!(f, "{}", num),
            Expression::StringLiteral(s) => write!(f, "StringLiteral: {}", s),
            
            Expression::BinaryOp(left, op, right) => {
                let op_str = match op {
                    Operator::Add => "+",
                    Operator::Sub => "-",
                    Operator::Mul => "*",
                    Operator::Div => "/",
                };
                write!(f, "({} {} {})", left, op_str, right)
            }

            Expression::ComparisonOp(left, op, right) => {
                let op_str = match op {
                    ComparisonOperator::Equal => "==",
                    ComparisonOperator::NotEqual => "!=",
                    ComparisonOperator::LessThan => "<",
                    ComparisonOperator::GreaterThan => ">",
                    ComparisonOperator::LessThanOrEqual => "<=",
                    ComparisonOperator::GreaterThanOrEqual => ">=",
                };
                write!(f, "({} {} {})", left, op_str, right)
            }

            Expression::LogicalOp(left, op, right) => {
                let op_str = match op {
                    LogicalOperator::And => "&&",
                    LogicalOperator::Or => "||",
                };
                write!(f, "({} {} {})", left, op_str, right)
            }

            Expression::FunctionCall { name, args } => {
                let args_str = args
                    .iter()
                    .map(|arg| format!("{}", arg))
                    .collect::<Vec<String>>()
                    .join(", ");
                write!(f, "FunctionCall: {}({})", name, args_str)
            }
        }
    }
}


impl fmt::Display for Operator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let op_str = match self {
            Operator::Add => "+",
            Operator::Sub => "-",
            Operator::Mul => "*",
            Operator::Div => "/",
        };
        write!(f, "{}", op_str)
    }
}
