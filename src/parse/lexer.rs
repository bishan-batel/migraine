use string_builder::Builder;

use super::err::{FilePos, ParserError};

#[derive(Debug, Clone)]
pub enum Token {
    Op(Op),
    LoopStart,
    LoopEnd,
    FunctionCall(String),
    FunctionDefine,
    FunctionEnd,
}

#[derive(Debug, Clone)]
pub enum Op {
    Inc,
    Dec,
    Dump,
    Take,
    Literal(String),
    PtrRight,
    PtrLeft,
    BitNot,

    // Stack Operations
    PushNew(usize),
    PushOp(StackOp),
    PopOp(StackOp),
    HardPopOp(StackOp),
}

#[derive(Debug, Clone)]
pub enum StackOp {
    Default,
    Set,
    Add,
    Sub,
    Mul,
    Div,

    // Bitwise
    BitAnd,
    BitOr,
    BitXor,
}

enum StackType {
    Pop,
    Push,
    HardPop,
}

pub struct Lexer {
    src: Box<String>,
    idx: usize,
    file_pos: FilePos,
    next_char: Option<char>,
    tokens: Box<Vec<Token>>,
}

impl Lexer {
    pub fn new(src: Box<String>) -> Self {
        Self {
            idx: 0,
            file_pos: FilePos::new(),
            next_char: src.chars().nth(0),
            src,
            tokens: Default::default(),
        }
    }

    pub fn advance(&mut self) {
        self.idx += 1;
        self.next_char = self.src.chars().nth(self.idx);
        self.file_pos.advance(self.next_char);
    }

    pub fn regress(&mut self) {
        self.idx -= 1;
        self.next_char = self.src.chars().nth(self.idx);

        // recalculates filepos
        self.file_pos = FilePos::new();

        for i in 0..(self.idx) {
            self.file_pos.advance(self.src.chars().nth(i));
        }
    }

    /// Consumes self
    pub fn tokenize(mut self) -> Result<Box<Vec<Token>>, ParserError> {
        while let Some(curr) = self.next_char {
            // Skip whitespace
            if curr.is_whitespace() {
                self.advance();
                continue;
            }
            match curr {
                '$' => return Err(ParserError::MacroNotDefined(self.file_pos)),
                // non ops
                '@' => self.function_call()?,
                '{' => {
                    self.tokens.push(Token::FunctionDefine);
                    self.advance();
                }
                '}' => {
                    self.tokens.push(Token::FunctionEnd);
                    self.advance();
                }

                // comments
                '/' => {
                    self.advance();
                    if let Some(curr) = self.next_char {
                        // single line comments with '//'
                        if curr == '/' {
                            while let Some(curr) = self.next_char {
                                if curr == '\n' {
                                    break;
                                }
                                self.advance();
                            }
                            continue;
                        }
                    }
                    return Err(ParserError::IllegalCharacter(curr, self.file_pos));
                }

                // Unary Char Operations
                '.' => self.op(Op::Dump),
                ',' => self.op(Op::Take),
                '>' => self.op(Op::PtrRight),
                '<' => self.op(Op::PtrLeft),
                '+' => self.op(Op::Inc),
                '-' => self.op(Op::Dec),
                '~' => self.op(Op::BitNot),

                // String Literal
                '"' => self.str_literal()?,

                // Stack Operations
                '_' => self.stack_op(StackType::Pop),
                '&' => self.stack_op(StackType::HardPop),
                '^' => self.stack_op(StackType::Push),

                // invalid characters
                _ => return Err(ParserError::IllegalCharacter(curr, self.file_pos)),
            }
        }
        Ok(self.tokens)
    }

    // Used for stack operations toi do with 'pop'
    fn stack_op(&mut self, stack: StackType) {
        self.advance();
        let stack_op = if let Some(curr) = self.next_char {
            // special case for push for push new ( ^10)
            if match stack {
                StackType::Push => true,
                _ => false,
            } {
                // if first letter after is ascii, keep reading
                if curr.is_ascii_digit() {
                    let mut builder = Builder::default();

                    // keeps reading character digits until no longer digits
                    while let Some(curr) = self.next_char {
                        if curr.is_ascii_digit() {
                            builder.append(curr);
                            self.advance();
                        } else {
                            // break when hits a character that isn't a digit
                            break;
                        }
                    }
                    // turns read number into u32
                    let size: usize = builder.string().unwrap().parse().unwrap();
                    self.tokens.push(Token::Op(Op::PushNew(size)));
                    return;
                }
            }

            self.advance();

            // checks if next char creates a dualchar op (_+, &=)
            match curr {
                '+' => StackOp::Add,
                '-' => StackOp::Sub,
                '*' => StackOp::Mul,
                '/' => StackOp::Div,
                '=' => StackOp::Set,

                // Bitwise
                '&' => StackOp::BitAnd,
                '|' => StackOp::BitOr,
                '^' => StackOp::BitXor,

                // go back if next char isn't part of a dualchar op
                _ => {
                    self.regress();
                    StackOp::Default
                }
            }
        } else {
            // Default stack operation (single '&' or '_') if EOF
            StackOp::Default
        };

        let next_tok = Token::Op(match stack {
            StackType::Pop => Op::PopOp(stack_op),
            StackType::HardPop => Op::HardPopOp(stack_op),
            StackType::Push => Op::PushOp(stack_op),
        });
        self.tokens.push(next_tok);
    }

    // Used to read function calls as well as function defines (eg. @main, @main {})
    fn function_call(&mut self) -> Result<(), ParserError> {
        self.advance();
        let mut builder = Builder::default();

        while let Some(curr) = self.next_char {
            if curr.is_whitespace() {
                let func_name = builder.string().unwrap();
                self.advance();
                self.tokens.push(Token::FunctionCall(func_name));
                return Ok(());
            }
            builder.append(curr);
            self.advance();
        }
        Err(ParserError::FunctionMustEndWithWhitespace(self.file_pos))
    }

    fn str_literal(&mut self) -> Result<(), ParserError> {
        self.advance();
        let mut builder = Builder::default();
        while let Some(curr) = self.next_char {
            // escaped characters
            if curr == '\\' {
                self.advance();
                if let Some(curr) = self.next_char {
                    builder.append(match curr {
                        'n' => '\n',
                        't' => '\t',
                        '0' => '\0',
                        _ => curr,
                    });
                    self.advance();
                } else {
                    // returns error if reaches end of file after escape char
                    return Err(ParserError::LiteralNotEnded(self.file_pos));
                }
                continue;
            }

            // termination

            if curr == '"' {
                // finalizes buidler & adds operation
                let literal = builder.string().unwrap();
                self.op(Op::Literal(literal));
                return Ok(());
            }

            builder.append(curr);
            self.advance();
        }
        Err(ParserError::LiteralNotEnded(self.file_pos))
    }

    pub fn op(&mut self, op: Op) {
        self.tokens.push(Token::Op(op));
        self.advance();
    }
}
