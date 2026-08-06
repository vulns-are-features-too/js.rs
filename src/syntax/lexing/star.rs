use crate::syntax::lexing::{
    lexer::{Base, Lexer, LexerState},
    tokens::Token,
};

pub struct Star;

impl LexerState for Star {}

impl<'i, 'o> Lexer<'i, Star>
where
    'i: 'o,
{
    pub fn lex(mut self, start: usize) -> (Lexer<'i, Base>, Token<'o>) {
        self.next_char();

        if self.eat('=') {
            let token = Token::StarEqual(start);
            return (self.transition(), token);
        }

        if self.eat('*') {
            let token = Token::Star2(start);
            return (self.transition(), token);
        }

        let token = Token::Star(start);
        (self.transition(), token)
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::syntax::lexing::lexer::lex_all;
    use rstest::*;

    #[rstest]
    #[case("*", Token::Star(0))]
    #[case("**", Token::Star2(0))]
    #[case("*=", Token::StarEqual(0))]
    fn single(#[case] s: &str, #[case] expected: Token) {
        let tokens = lex_all(s);
        assert_eq!(expected, tokens[0]);
        assert_eq!(2, tokens.len());
        assert_eq!(Token::Eof, tokens[1]);
    }

    #[test]
    fn mixed() {
        let tokens = lex_all("a*=b*c**d");
        assert_eq!(
            vec![
                Token::Identifier("a"),
                Token::StarEqual(1),
                Token::Identifier("b"),
                Token::Star(4),
                Token::Identifier("c"),
                Token::Star2(6),
                Token::Identifier("d"),
                Token::Eof,
            ],
            tokens
        );
    }
}
