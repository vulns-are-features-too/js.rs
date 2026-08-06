use crate::lexing::{
    lexer::{Base, Lexer, LexerState},
    tokens::Token,
};

pub struct WhiteSpace;

impl LexerState for WhiteSpace {}

impl<'i, 'o> Lexer<'i, WhiteSpace>
where
    'i: 'o,
{
    pub fn lex(mut self, start: usize) -> (Lexer<'i, Base>, Token<'o>) {
        let mut end = start;
        self.next_char();
        while let Some((_, c2)) = &self.chars.peek()
            && matches!(c2, ' ' | '\t')
        {
            end += 1;
            self.next_char();
        }
        let token = Token::WhiteSpace(&self.input[start..=end]);
        (self.transition(), token)
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::lexing::lexer::lex_all;
    use rstest::*;

    #[rstest]
    #[case(" ")]
    #[case("\t")]
    #[case(" \t ")]
    fn single(#[case] s: &str) {
        let tokens = lex_all(s);
        let expected = Token::WhiteSpace(s);
        assert_eq!(expected, tokens[0]);
        assert_eq!(2, tokens.len());
        assert_eq!(Token::Eof, tokens[1]);
    }

    #[rstest]
    #[case(" ", 1)]
    #[case("  ", 1)]
    #[case("\t", 1)]
    #[case("\t\t", 1)]
    #[case(" \t", 1)]
    #[case("\t ", 1)]
    #[case(" \t ", 1)]
    #[case("\t \t", 1)]
    #[case(" \n ", 2)]
    #[case("\t\n\t", 2)]
    #[case(" ; ", 2)]
    #[case(" ;\t", 2)]
    #[case("\t; ", 2)]
    #[case("\t;\t", 2)]
    fn count(#[case] s: &str, #[case] expected: usize) {
        let tokens = lex_all(s);
        let count = tokens
            .iter()
            .filter(|x| matches!(x, Token::WhiteSpace(_)))
            .count();
        assert_eq!(expected, count);
    }
}
