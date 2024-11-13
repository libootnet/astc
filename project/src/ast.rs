use crate::statement::Statement;

pub fn print_ast(statements: Vec<Statement>) {
    for stmt in statements {
        println!("{}", stmt);
    }
}
