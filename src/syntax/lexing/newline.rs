use crate::syntax::lexing::{
    lexer::{Base, Lexer, LexerState},
    tokens::Token,
};

pub struct LF;
pub struct CR;

impl LexerState for LF {}
impl LexerState for CR {}

impl<'i, 'o> Lexer<'i, LF>
where
    'i: 'o,
{
    pub fn lex(mut self, start: usize) -> (Lexer<'i, Base>, Token<'o>) {
        self.col_num = 0;
        self.line_num += 1;
        self.chars.next();
        let token = Token::NewLine(&self.input[start..=start]);
        (self.transition(), token)
    }
}

impl<'i, 'o> Lexer<'i, CR>
where
    'i: 'o,
{
    pub fn lex(mut self, start: usize) -> (Lexer<'i, Base>, Token<'o>) {
        self.col_num = 0;
        self.line_num += 1;
        self.chars.next();
        let mut end = start;
        if let Some(&(_, '\n')) = self.chars.peek() {
            self.chars.next();
            end += 1;
        }
        let token = Token::NewLine(&self.input[start..=end]);
        (self.transition(), token)
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::syntax::lexing::lexer::lex;
    use rstest::*;

    #[rstest]
    // newlines
    #[case("\n")]
    #[case("\r\n")]
    #[case("\r")]
    fn single(#[case] s: &str) {
        let tokens = lex(s);
        let expected = Token::NewLine(s);
        assert_eq!(expected, tokens[0]);
        assert_eq!(2, tokens.len());
        assert_eq!(Token::Eof, tokens[1]);
    }

    #[rstest]
    // only newlines
    #[case("\n", 1)]
    #[case("\r", 1)]
    #[case("\n\n", 2)]
    #[case("\n\r", 2)]
    #[case("\r\n", 1)]
    #[case("\r\r", 2)]
    #[case("\r\n\n", 2)]
    #[case("\n\r\n", 2)]
    #[case("\n\n\r", 3)]
    #[case("\n\r\r", 3)]
    #[case("\r\n\r", 2)]
    #[case("\r\r\n", 2)]
    #[case("\r\n\r\n", 2)]
    // with extras
    #[case(" \n ", 1)]
    #[case(" \r ", 1)]
    #[case(" \n\n ", 2)]
    #[case(" \n\r ", 2)]
    #[case(" \r\n ", 1)]
    #[case(" \r\r ", 2)]
    #[case("\r \n", 2)]
    #[case("\n \r", 2)]
    #[case(" \r \n ", 2)]
    fn count_new_lines(#[case] s: &str, #[case] expected: usize) {
        let tokens = lex(s);
        let count = tokens
            .iter()
            .filter(|x| matches!(x, Token::NewLine(_)))
            .count();
        assert_eq!(expected, count);
    }
}
