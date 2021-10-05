#[derive(Debug, Copy, Clone)]
pub struct FilePos {
    pub line: usize,
    pub column: usize,
    pub idx: usize,
}

impl From<(usize, usize)> for FilePos {
    fn from(src: (usize, usize)) -> Self {
        Self {
            line: src.0,
            column: src.1,
            idx: 0,
        }
    }
}

#[derive(Debug)]
pub enum ParserError {
    IllegalCharacter(FilePos),
    NoSubMacros(FilePos),
    NoMacroDef(FilePos),
    LiteralNotEnded(FilePos),
    FunctionMustEndWithWhitespace(FilePos),
    Generic,
}
