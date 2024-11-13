mod lexer;
mod parser;
mod ast;

use std::fs::File;
use std::io::{self, Read};

fn main() {
    let path = "examples/if.astc";
    
    let code = match read_file(path) {
        Ok(contents) => contents,
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            return;
        }
    };

    let lexer = lexer::token::Lexer::new(&code);
    let mut parser = parser::parser::Parser::new(lexer);

    let statements = parser.parse();
    ast::print_ast(statements);
}

fn read_file(path: &str) -> Result<String, io::Error> {
    let mut file = match File::open(path) {
        Ok(file) => file,
        Err(error) => match error.kind() {
            io::ErrorKind::NotFound => {
                eprintln!("Error: File not found at path");
                return Err(error);
            }
            io::ErrorKind::PermissionDenied => {
                eprintln!("Error: Permission denied for file");
                return Err(error);
            }
            other_error => {
                eprintln!("Error: Could not open file '{}': {:?}", path, other_error);
                return Err(error);
            }
        },
    };

    let mut contents = String::new();
    if let Err(error) = file.read_to_string(&mut contents) {
        eprintln!("Error: Could not read from file '{}': {:?}", path, error.kind());
        return Err(error);
    }

    Ok(contents)
}