pub mod err;
pub mod lexer;
pub mod parser;
pub mod preproc;
use err::*;
use parser::Func;

use self::{lexer::Lexer, parser::Parser};

const TEST_PGM: &str = include_str!("../../test/test.migraine");

pub fn parse() -> Result<Vec<Func>, ParserError> {
    let processed = preproc::process(TEST_PGM.to_string())?;
    println!("Preprocess Done");

    let tokens = Lexer::new(processed).tokenize()?;
    println!("Tokenization done");

    let functions = Parser::new(tokens).create_functions()?;
    println!("Parsing done");
    Ok(functions)
}
