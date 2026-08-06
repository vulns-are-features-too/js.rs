use std::str::FromStr;

use crate::syntax::{
    lexing::lexer::{LexingError, lex_all},
    parsing::parser::{ParsingError, parse},
};

#[derive(Debug)]
pub struct Ast {}

#[derive(Debug)]
pub enum SyntaxError {
    Lexing(LexingError),
    Parsing(ParsingError),
}

impl FromStr for Ast {
    type Err = SyntaxError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tokens = lex_all(s);
        parse(tokens).map_err(SyntaxError::Parsing)
    }
}
