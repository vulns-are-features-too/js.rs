use crate::lexing::{
    lexer::{Base, Lexer, LexerState},
    tokens::Token,
};

pub struct Ampersand;

impl LexerState for Ampersand {}

impl<'i, 'o> Lexer<'i, Ampersand>
where
    'i: 'o,
{
    #[must_use]
    pub fn lex(mut self, start: usize) -> (Lexer<'i, Base>, Token<'o>) {
        self.next_char();

        if self.eat('=') {
            let token = Token::AmpersandEqual(start);
            return (self.transition(), token);
        }

        if self.eat('&') {
            let token = Token::Ampersand2(start);
            return (self.transition(), token);
        }

        let token = Token::Ampersand(start);
        (self.transition(), token)
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::lexing::lexer::lex_all;
    use rstest::*;

    #[rstest]
    #[case("&", Token::Ampersand(0))]
    #[case("&&", Token::Ampersand2(0))]
    #[case("&=", Token::AmpersandEqual(0))]
    fn single(#[case] s: &str, #[case] expected: Token) {
        let tokens = lex_all(s);
        assert_eq!(expected, tokens[0]);
        assert_eq!(s.len(), tokens[0].len());
        assert_eq!(2, tokens.len());
        assert_eq!(Token::Eof, tokens[1]);
    }

    #[test]
    fn mixed() {
        let tokens = lex_all("a&=b&c&&d");
        assert_eq!(
            vec![
                Token::Identifier("a"),
                Token::AmpersandEqual(1),
                Token::Identifier("b"),
                Token::Ampersand(4),
                Token::Identifier("c"),
                Token::Ampersand2(6),
                Token::Identifier("d"),
                Token::Eof,
            ],
            tokens
        );
    }
}
