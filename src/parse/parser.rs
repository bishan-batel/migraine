use std::slice::Iter;

use super::err::*;
use super::lexer::{Op, Token};

#[derive(Debug)]
struct UnparsedFunc {
    name: String,
    content: Vec<(Token, FilePos)>,
}

#[derive(Debug, Clone)]
pub struct Func {
    pub name: String,
    pub node: OpNode,
}

#[derive(Debug, Clone)]
pub enum OpNode {
    Loop(Vec<OpNode>),
    Root(Vec<OpNode>),
    Operation(Op),
    FuncCall(String),
}

pub struct Parser {
    src_toks: Vec<(Token, FilePos)>,
}

impl Parser {
    pub fn new(src_toks: Vec<(Token, FilePos)>) -> Self {
        Self { src_toks }
    }

    pub fn create_functions(&mut self) -> Result<Vec<Func>, ParserError> {
        let mut funcs = Vec::<Func>::default();

        for unparsed in self.get_unparsed_functions()? {
            funcs.push(Func {
                name: unparsed.name,
                node: OpNode::Root(Self::create_parse_tree(
                    &mut unparsed.content.iter(),
                    false,
                    FilePos::new(),
                )?),
            })
        }

        Ok(funcs)
    }

    fn create_parse_tree(
        tok_iter: &mut Iter<(Token, FilePos)>,
        allow_loop_end: bool,
        start_pos: FilePos,
    ) -> Result<Vec<OpNode>, ParserError> {
        let mut nodes = Vec::<OpNode>::default();
        while let Some((tok, pos)) = tok_iter.next() {
            // break if at end of loop
            if allow_loop_end
                && match tok {
                    Token::LoopEnd => true,
                    _ => false,
                }
            {
                return Ok(nodes);
            }

            match tok {
                Token::LoopStart => {
                    nodes.push(OpNode::Loop(Self::create_parse_tree(
                        tok_iter,
                        true,
                        pos.clone(),
                    )?));
                }
                Token::Op(op) => nodes.push(OpNode::Operation(op.clone())),
                Token::FunctionCall(name) => nodes.push(OpNode::FuncCall(name.clone())),
                _ => return Err(ParserError::UnexpectedToken(tok.clone(), FilePos::new())),
            }
        }

        if allow_loop_end {
            return Err(ParserError::LoopNotEnded(start_pos));
        }
        Ok(nodes)
    }

    fn get_unparsed_functions(&self) -> Result<Vec<UnparsedFunc>, ParserError> {
        let mut funcs = Vec::<UnparsedFunc>::default();
        let mut tok_iter = self.src_toks.iter();

        // loops through all tokens
        while let Some((tok, file_pos)) = tok_iter.next() {
            match tok {
                Token::FunctionCall(name) => {
                    // checks next token if it is a func define
                    if let Some((tok, _)) = tok_iter.next() {
                        if match tok {
                            Token::FunctionDefine => true,
                            _ => false,
                        } {
                            let mut toks = Vec::default();

                            // reads everything until function end token
                            while let Some((tok, pos)) = tok_iter.next() {
                                match tok {
                                    // Nested function definition error
                                    Token::FunctionDefine => {
                                        return Err(ParserError::NoNestedFunctionDefines(*pos))
                                    }
                                    Token::FunctionEnd => break,
                                    _ => toks.push((tok.clone(), *pos)),
                                }
                            }
                            funcs.push(UnparsedFunc {
                                name: name.clone(),
                                content: toks,
                            });
                            continue;
                        }
                    }

                    return Err(ParserError::FunctionCallMustBeInFunction(
                        name.clone(),
                        *file_pos,
                    ));
                }
                _ => return Err(ParserError::UnexpectedToken(tok.clone(), *file_pos)),
            }
        }

        Ok(funcs)
    }
}
