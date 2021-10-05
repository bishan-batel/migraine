pub mod err;
pub mod lexer;
mod preproc;
use err::*;

use self::lexer::Lexer;

const TEST_PGM: &str = include_str!("../../test/test.migraine");

pub fn parse() -> Result<(), ParserError> {
    let processed = preproc::process(TEST_PGM.to_string())?;

    println!("Tokenizing...");
    let tokens = Lexer::new(processed).tokenize()?;
    println!("{:?}", tokens);
    Ok(())
}
