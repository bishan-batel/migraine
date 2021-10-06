pub mod err;
pub mod lexer;
pub mod parser;
pub mod preproc;
use err::*;
use parser::Func;

use self::{lexer::Lexer, parser::Parser};

pub fn parse(content: String) -> Result<Vec<Func>, ParserError> {
    let processed = preproc::process(content)?;
    println!("Preprocess Done");

    let tokens = Lexer::new(processed).tokenize()?;
    println!("Tokenization done");

    let functions = Parser::new(tokens).create_functions()?;
    println!("Parsing done");
    Ok(functions)
}
