use crate::syntax::lexing::{
    lexer::{Base, Lexer, LexerState},
    tokens::Token,
};

pub struct Div;

impl LexerState for Div {}

impl<'i, 'o> Lexer<'i, Div>
where
    'i: 'o,
{
    pub fn lex(mut self, start: usize) -> (Lexer<'i, Base>, Token<'o>) {
        self.next_char();

        if self.eat('=') {
            let token = Token::DivEqual(start);
            return (self.transition(), token);
        }

        let token = Token::Div(start);
        (self.transition(), token)
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::syntax::lexing::lexer::lex_all;
    use rstest::*;

    #[rstest]
    #[case("/", Token::Div(0))]
    #[case("/=", Token::DivEqual(0))]
    fn single(#[case] s: &str, #[case] expected: Token) {
        let tokens = lex_all(s);
        assert_eq!(expected, tokens[0]);
        assert_eq!(2, tokens.len());
        assert_eq!(Token::Eof, tokens[1]);
    }

    #[test]
    fn mixed() {
        let tokens = lex_all("a/=b/c");
        assert_eq!(
            vec![
                Token::Identifier("a"),
                Token::DivEqual(1),
                Token::Identifier("b"),
                Token::Div(4),
                Token::Identifier("c"),
                Token::Eof,
            ],
            tokens
        );
    }
}
