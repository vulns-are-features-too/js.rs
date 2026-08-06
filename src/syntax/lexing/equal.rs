use crate::syntax::lexing::{
    lexer::{Base, Lexer, LexerState},
    tokens::Token,
};

pub struct Equal;

impl LexerState for Equal {}

impl<'i, 'o> Lexer<'i, Equal>
where
    'i: 'o,
{
    pub fn lex(mut self, start: usize) -> (Lexer<'i, Base>, Token<'o>) {
        self.next_char();

        if self.eat('>') {
            let token = Token::Arrow(start);
            return (self.transition(), token);
        }

        if !self.eat('=') {
            let token = Token::Equal(start);
            return (self.transition(), token);
        }

        let token = if self.eat('=') {
            Token::Equal3(start)
        } else {
            Token::Equal2(start)
        };

        (self.transition(), token)
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::syntax::lexing::lexer::lex_all;
    use rstest::*;

    #[rstest]
    #[case("=", Token::Equal(0))]
    #[case("==", Token::Equal2(0))]
    #[case("===", Token::Equal3(0))]
    #[case("=>", Token::Arrow(0))]
    fn single(#[case] s: &str, #[case] expected: Token) {
        let tokens = lex_all(s);
        assert_eq!(expected, tokens[0]);
        assert_eq!(2, tokens.len());
        assert_eq!(Token::Eof, tokens[1]);
    }

    #[test]
    fn mixed() {
        assert_eq!(
            vec![
                Token::Identifier("a"),
                Token::Equal(1),
                Token::Identifier("b"),
                Token::Equal2(3),
                Token::Identifier("c"),
                Token::Equal3(6),
                Token::Identifier("d"),
                Token::Arrow(10),
                Token::Identifier("e"),
                Token::Eof,
            ],
            lex_all("a=b==c===d=>e")
        );
    }
}
