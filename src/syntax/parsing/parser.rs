use crate::syntax::{lexing::tokens::Token, parsing::ast::Ast};

#[derive(Debug)]
pub enum ParsingError {}

pub fn parse(_tokens: Vec<Token<'_>>) -> Result<Ast, ParsingError> {
    todo!();
}
