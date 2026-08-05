use crate::syntax::lexing::{
    lexer::{Base, Lexer, LexerState},
    tokens::Token,
};

pub struct Bang;

impl LexerState for Bang {}

impl<'i, 'o> Lexer<'i, Bang>
where
    'i: 'o,
{
    pub fn lex(mut self, start: usize) -> (Lexer<'i, Base>, Token<'o>) {
        self.next_char();

        if !self.eat('=') {
            let token = Token::Bang(start);
            return (self.transition(), token);
        }

        let token = if self.eat('=') {
            Token::BangEqual2(start)
        } else {
            Token::BangEqual(start)
        };

        (self.transition(), token)
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::syntax::lexing::lexer::lex;
    use rstest::*;

    #[rstest]
    #[case("!", Token::Bang(0))]
    #[case("!=", Token::BangEqual(0))]
    #[case("!==", Token::BangEqual2(0))]
    fn single(#[case] s: &str, #[case] expected: Token) {
        let tokens = lex(s);
        assert_eq!(expected, tokens[0]);
        assert_eq!(2, tokens.len());
        assert_eq!(Token::Eof, tokens[1]);
    }

    #[test]
    fn mixed() {
        let tokens = lex("a!=b!==c!d");
        assert_eq!(
            vec![
                Token::Identifier("a"),
                Token::BangEqual(1),
                Token::Identifier("b"),
                Token::BangEqual2(4),
                Token::Identifier("c"),
                Token::Bang(8),
                Token::Identifier("d"),
                Token::Eof,
            ],
            tokens
        );
    }
}
