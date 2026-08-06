use std::str::FromStr;

use crate::{
    lexing::lexer::lex_all,
    parsing::parser::{ParsingError, parse},
};

#[derive(Debug)]
pub struct Ast {}

impl FromStr for Ast {
    type Err = ParsingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tokens = lex_all(s);
        parse(tokens)
    }
}
