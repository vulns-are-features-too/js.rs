use crate::lexing::{
    lexer::{Base, Lexer, LexerState},
    tokens::Token,
};

pub struct Less;

impl LexerState for Less {}

impl<'i, 'o> Lexer<'i, Less>
where
    'i: 'o,
{
    #[must_use]
    pub fn lex(mut self, start: usize) -> (Lexer<'i, Base>, Token<'o>) {
        self.next_char();

        if self.eat('=') {
            let token = Token::LessEqual(start);
            return (self.transition(), token);
        }

        if !self.eat('<') {
            let token = Token::Less(start);
            return (self.transition(), token);
        }

        if self.eat('=') {
            let token = Token::Less2Equal(start);
            return (self.transition(), token);
        }

        let token = Token::Less2(start);
        (self.transition(), token)
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::lexing::lexer::lex_all;
    use rstest::*;

    #[rstest]
    #[case("<", Token::Less(0))]
    #[case("<=", Token::LessEqual(0))]
    #[case("<<", Token::Less2(0))]
    #[case("<<=", Token::Less2Equal(0))]
    fn single(#[case] s: &str, #[case] expected: Token) {
        let tokens = lex_all(s);
        assert_eq!(expected, tokens[0]);
        assert_eq!(s.len(), tokens[0].len());
        assert_eq!(2, tokens.len());
        assert_eq!(Token::Eof, tokens[1]);
    }

    #[test]
    fn mixed() {
        let tokens = lex_all("< <= << <<=");
        assert_eq!(
            vec![
                Token::Less(0),
                Token::WhiteSpace(" "),
                Token::LessEqual(2),
                Token::WhiteSpace(" "),
                Token::Less2(5),
                Token::WhiteSpace(" "),
                Token::Less2Equal(8),
                Token::Eof,
            ],
            tokens
        );
    }
}
