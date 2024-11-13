use std::fmt;

#[derive(Debug)]
pub enum Statement {
    VarDeclaration { name: String, var_type: Option<String>, value: Expression },
    FunctionCall { name: String, args: Vec<Expression> },
}

#[derive(Debug)]
pub enum Expression {
    Identifier(String),
    Number(f64),
    StringLiteral(String),
    BinaryOp(Box<Expression>, Operator, Box<Expression>),
    FunctionCall { name: String, args: Vec<Expression> },
}

#[derive(Debug, PartialEq)]
pub enum Operator {
    Add,
    Sub,
    Mul,
    Div,
}

impl fmt::Display for Statement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Statement::VarDeclaration { name, var_type, value } => {
                let var_type_str = match var_type {
                    Some(t) => t.clone(),
                    None => "unknown".to_string(),
                };
                write!(f, "Var: {}: {} = {}", name, var_type_str, value)
            }
            Statement::FunctionCall { name, args } => {
                let args_str = args.iter().map(|arg| format!("{}", arg)).collect::<Vec<String>>().join(", ");
                write!(f, "FunctionCall: {}({})", name, args_str)
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
            Expression::FunctionCall { name, args } => {
                let args_str = args.iter().map(|arg| format!("{}", arg)).collect::<Vec<String>>().join(", ");
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
