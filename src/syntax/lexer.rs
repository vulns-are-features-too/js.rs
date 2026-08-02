use crate::syntax::{tokens::Token};

#[derive(Debug)]
pub enum LexingError {}

pub fn lex(_s: &str) -> Result<Vec<Token<'_>>, LexingError>
{
    todo!()
}
