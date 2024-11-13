#[derive(Debug)]
pub enum Statement {
    VarDeclaration { 
        name: String, 
        var_type: Option<String>, 
        value: Expression 
    },
    FunctionCall { 
        name: String, 
        args: Vec<Expression> 
    },
    If {
        condition: Expression,
        then_branch: Vec<Statement>,
        else_branch: Option<Vec<Statement>>,
    },
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