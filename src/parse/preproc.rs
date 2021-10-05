use super::err::*;
use string_builder::Builder;
use substring::Substring;
const MACRO_DEF_WORD: &str = "$macrodef ";
const MACRO_END_WORD: &str = "$macroend\n";

pub fn process(src: String) -> Result<Box<String>, ParserError> {
    PreProccessor::new(src).process()
}

#[derive(Debug)]
struct Macro {
    name: String,
    content: String,
}

struct PreProccessor {
    src: String,
    out: Builder,
    idx: usize,
    file_pos: FilePos,
    next: Option<char>,
    macros: Vec<Macro>,
    defining: Option<(String, usize)>,
}

impl PreProccessor {
    fn new(src: String) -> Self {
        let mut macros = Default::default();

        Self::add_default_macros(&mut macros);
        Self {
            next: src.chars().nth(0),
            src,
            file_pos: FilePos::new(),
            idx: 0,
            out: Default::default(),
            defining: None,
            macros,
        }
    }

    fn add_default_macros(macros: &mut Vec<Macro>) {
        macros.push(Macro {
            name: "$FUNNY!".to_string(),
            content: "420 69".to_string(),
        });
    }

    fn advance(&mut self) {
        self.idx += 1;
        self.next = self.src.chars().nth(self.idx);
        self.file_pos.advance(self.next);
    }

    /// Consumes self
    pub fn process(mut self) -> Result<Box<String>, ParserError> {
        // Processes Keywords
        while let Some(curr) = self.next {
            // continue if char matches first letter of macro keyword
            if self.macro_def_read(self.idx)?
                || self.macro_end_read(self.idx)?
                || self.macro_call_read(self.idx)?
            {
                continue;
            }

            if self.defining.is_none() {
                self.out.append(curr);
            }
            self.advance();
        }

        let finished = self.out.string().unwrap();
        Ok(Box::new(finished))
    }

    fn macro_call_read(&mut self, idx: usize) -> Result<bool, ParserError> {
        // checks if call matches any defined macros
        for r#macro in self.macros.iter() {
            let name = r#macro.name.as_str();
            let name_len = name.len();

            if self.read_word(name, idx) {
                self.out.append(r#macro.content.clone());
                self.skip(name_len);
                return Ok(true);
            }
        }
        Ok(false)
    }

    fn macro_end_read(&mut self, end_idx: usize) -> Result<bool, ParserError> {
        // if next characters is the mac def keyword, process
        if self.read_word(MACRO_END_WORD, end_idx) {
            // if no macrodef was called then return err
            if self.defining.is_none() {
                return Err(ParserError::NoMacroDef(self.file_pos));
            }

            let (name, start_idx) = self.defining.clone().unwrap();
            // clean defining
            self.defining = None;

            let r#macro = Macro {
                name: "$".to_string() + name.as_str() + "!",
                content: self.src.substring(start_idx, end_idx - 1).to_string(),
            };

            println!(
                "Macro defined (name: {}, content:`{}`)",
                r#macro.name, r#macro.content
            );
            self.macros.push(r#macro);

            // skips end word
            self.skip(MACRO_END_WORD.len());
            return Ok(true);
        }

        Ok(false)
    }

    // returns Ok(true) if a macro was successfully read
    fn macro_def_read(&mut self, start_idx: usize) -> Result<bool, ParserError> {
        if self.read_word(MACRO_DEF_WORD, start_idx) {
            if self.defining.is_some() {
                return Err(ParserError::NoSubMacros(self.file_pos));
            }

            // Advances to skip macro keyword
            self.skip(MACRO_DEF_WORD.len());

            // gets name
            let mut name = Builder::default();
            while let Some(curr) = self.next {
                self.advance();
                if curr.is_whitespace() {
                    break;
                }
                name.append(curr);
            }
            let name = name.string().unwrap();

            let skip_len = MACRO_DEF_WORD.len() + name.len();

            self.defining = Some((name, start_idx + skip_len + 1));
            return Ok(true);
        }

        Ok(false)
    }

    fn skip(&mut self, n: usize) {
        for _ in 0..n {
            self.advance();
        }
    }

    fn read_word(&self, word: &str, idx: usize) -> bool {
        // if length of word is greater than how much the file is left to read,
        // return false for no macro
        if idx + word.len() >= self.src.len() {
            return false;
        }

        let next_chars = self.src.substring(idx, idx + word.len());
        return next_chars == word;
    }
}
