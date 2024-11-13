mod lexer;
mod parser;
mod statement;
mod ast;

fn main() {
    let code = r#"
    let x: number = 42 * 10 * 20 / 20 * 2 + 40;
    "#;

    let lexer = lexer::Lexer::new(code);
    let mut parser = parser::Parser::new(lexer);

    let statements = parser.parse();
    ast::print_ast(statements);
}
