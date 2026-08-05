use crate::syntax::lexing::{
    lexer::{Base, Lexer, LexerState},
    tokens::Token,
};

pub struct Percent;

impl LexerState for Percent {}

impl<'i, 'o> Lexer<'i, Percent>
where
    'i: 'o,
{
    pub fn lex(mut self, start: usize) -> (Lexer<'i, Base>, Token<'o>) {
        self.next_char();

        if self.eat('=') {
            let token = Token::PercentEqual(start);
            return (self.transition(), token);
        }

        let token = Token::Percent(start);
        (self.transition(), token)
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::syntax::lexing::lexer::lex;
    use rstest::*;

    #[rstest]
    #[case("%", Token::Percent(0))]
    #[case("%=", Token::PercentEqual(0))]
    fn single(#[case] s: &str, #[case] expected: Token) {
        let tokens = lex(s);
        assert_eq!(expected, tokens[0]);
        assert_eq!(2, tokens.len());
        assert_eq!(Token::Eof, tokens[1]);
    }

    #[test]
    fn mixed() {
        let tokens = lex("a%=b%c");
        assert_eq!(
            vec![
                Token::Identifier("a"),
                Token::PercentEqual(1),
                Token::Identifier("b"),
                Token::Percent(4),
                Token::Identifier("c"),
                Token::Eof,
            ],
            tokens
        );
    }
}
