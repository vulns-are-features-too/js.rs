use crate::syntax::{parsing::ast::Ast, lexing::tokens::Token};

#[derive(Debug)]
pub enum ParsingError {}

pub fn parse(_tokens: Vec<Token<'_>>) -> Result<Ast, ParsingError> {
    todo!();
}
