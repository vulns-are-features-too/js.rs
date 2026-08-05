use crate::syntax::lexing::{
    lexer::{Base, Lexer, LexerState},
    tokens::Token,
};

pub struct Dot;

impl LexerState for Dot {}

impl<'i, 'o> Lexer<'i, Dot>
where
    'i: 'o,
{
    pub fn lex(mut self, start: usize) -> (Lexer<'i, Base>, Token<'o>) {
        self.next_char();

        if !self.eat('.') {
            let token = Token::Dot(start);
            return (self.transition(), token);
        }

        if !self.eat('.') {
            let token = Token::Dot2(start);
            return (self.transition(), token);
        }

        let token = Token::Dot3(start);
        (self.transition(), token)
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::syntax::lexing::lexer::lex;
    use rstest::*;

    #[rstest]
    #[case(".", Token::Dot(0))]
    #[case("..", Token::Dot2(0))]
    #[case("...", Token::Dot3(0))]
    fn single(#[case] s: &str, #[case] expected: Token) {
        let tokens = lex(s);
        assert_eq!(expected, tokens[0]);
        assert_eq!(2, tokens.len());
        assert_eq!(Token::Eof, tokens[1]);
    }

    #[test]
    fn mixed() {
        assert_eq!(
            vec![
                Token::Identifier("a"),
                Token::Dot(1),
                Token::Identifier("b"),
                Token::Dot2(3),
                Token::Identifier("c"),
                Token::Dot3(6),
                Token::Identifier("d"),
                Token::Eof,
            ],
            lex("a.b..c...d")
        );
    }
}
