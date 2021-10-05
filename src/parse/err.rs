use super::lexer::Token;

#[derive(Debug, Copy, Clone)]
pub struct FilePos {
    pub line: usize,
    pub column: usize,
}

impl FilePos {
    pub fn new() -> Self {
        Self { line: 1, column: 1 }
    }

    pub fn advance(&mut self, c: Option<char>) {
        if let Some(c) = c {
            if c == '\n' {
                self.column = 1;
                self.line += 1;
            } else {
                self.column += 1;
            }
        }
    }
}

#[derive(Debug)]
pub enum ParserError {
    // Preprocessor
    NoSubMacros(FilePos),
    NoMacroDef(FilePos),

    // Lexer
    IllegalCharacter(char, FilePos),
    LiteralNotEnded(FilePos),
    FunctionMustEndWithWhitespace(FilePos),
    MacroNotDefined(FilePos),

    // Parser
    UnexpectedToken(Token, FilePos),
    FunctionCallMustBeInFunction(String, FilePos),
    NoNestedFunctionDefines(FilePos),
    LoopNotEnded(FilePos),
    DuplicateFunctionNames(FilePos),
    #[allow(dead_code)]
    Generic,
}
