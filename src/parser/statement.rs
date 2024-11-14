#[derive(Debug)]
pub enum Statement {
    VarDeclaration {
        name: String,
        var_type: Option<String>,
        value: Expression,
    },
    FunctionCall {
        name: String,
        args: Vec<Expression>,
    },
    If {
        condition: Expression,
        then_branch: Vec<Statement>,
        else_branch: Option<Box<Statement>>,
    },
    Block(Vec<Statement>),
}

#[derive(Debug)]
pub enum Expression {
    Identifier(String),
    Number(f64),
    StringLiteral(String),
    BinaryOp(Box<Expression>, Operator, Box<Expression>),
    LogicalOp(Box<Expression>, LogicalOperator, Box<Expression>), // 論理演算子
    ComparisonOp(Box<Expression>, ComparisonOperator, Box<Expression>), // 比較演算子
    FunctionCall { name: String, args: Vec<Expression> },
}

#[derive(Debug, PartialEq)]
pub enum Operator {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug, PartialEq)]
pub enum LogicalOperator {
    And, // &&
    Or,  // ||
}

#[derive(Debug, PartialEq)]
pub enum ComparisonOperator {
    Equal,              // ==
    NotEqual,           // !=
    LessThan,           // <
    GreaterThan,        // >
    LessThanOrEqual,    // <=
    GreaterThanOrEqual, // >=
}
